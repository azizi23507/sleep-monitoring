use axum::extract::ws::{Message, WebSocket};
use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};
use redis::aio::ConnectionManager;

use crate::models::sensor_data::SensorData;
use crate::routes::sensor_data::AppState;

/// Real-time state for WebSocket streaming (Branch 1)
/// 
/// This structure manages Redis cache for the last 100 sensor readings,
/// enabling zero-latency real-time monitoring for the frontend.
/// 
/// Architecture (Branch 1 - Real-time Path):
/// Pi → Backend → Redis Cache → WebSocket → Frontend
/// 
/// Purpose:
/// - Enable ZERO-LATENCY real-time monitoring
/// - Persistent cache (survives server restarts)
/// - Frontend gets instant updates
/// - Acts as backup for reconnection
/// 
/// Redis Data Structure:
/// - Key: "sensor:latest"
/// - Type: List (LPUSH for new data, LTRIM to keep last 100)
/// - TTL: 2 hours (auto-expiry if no updates)
/// 
/// Why Redis instead of in-memory?
/// - Persistent across server restarts
/// - Shared across multiple backend instances
/// - Production-ready caching solution
/// - Built-in TTL and eviction policies
#[derive(Clone)]
pub struct RealtimeState {
    /// Redis connection manager for async operations
    /// 
    /// ConnectionManager automatically:
    /// - Reconnects on connection loss
    /// - Handles connection pooling
    /// - Thread-safe for concurrent access
    pub redis: ConnectionManager,
}

impl RealtimeState {
    /// Create new RealtimeState with Redis connection
    /// 
    /// Args:
    /// - redis_url: Connection string (e.g., "redis://127.0.0.1:6379")
    /// 
    /// Returns:
    /// - Ok(RealtimeState) on successful connection
    /// - Err(redis::RedisError) if connection fails
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let redis = ConnectionManager::new(client).await?;
        Ok(Self { redis })
    }
}

/// WebSocket handler - upgrades HTTP connection to WebSocket
/// 
/// GET /ws
/// 
/// This endpoint upgrades an HTTP connection to a WebSocket connection,
/// allowing bidirectional real-time communication with the frontend.
/// 
/// Flow:
/// 1. Client sends HTTP GET request to /ws
/// 2. Server upgrades connection to WebSocket protocol
/// 3. Server starts ws_loop to continuously send data
/// 4. Connection stays open until client disconnects
/// 
/// Example JavaScript client:
/// ```javascript
/// const ws = new WebSocket('ws://localhost:3000/ws');
/// ws.onmessage = (event) => {
///   const data = JSON.parse(event.data);
///   console.log('Received:', data);
/// };
/// ```
#[tracing::instrument(skip_all)]
pub async fn ws_handler(
    ws: WebSocketUpgrade, 
    State(state): State<AppState>
) -> impl IntoResponse {
    tracing::info!("WebSocket connection request received");
    
    let state_cloned = state.clone();
    
    // Upgrade HTTP connection to WebSocket and start the loop
    ws.on_upgrade(move |socket| ws_loop(socket, state_cloned))
}

/// WebSocket loop - continuously sends data to connected client
/// 
/// This async function runs for the lifetime of the WebSocket connection,
/// continuously reading from Redis and sending to the client.
/// 
/// Flow:
/// 1. Read last 100 readings from Redis list
/// 2. Deserialize from JSON
/// 3. Send array to client via WebSocket
/// 4. Wait 1 second
/// 5. Repeat until connection closes
/// 
/// The loop stops when:
/// - Client disconnects
/// - Send operation fails (network error)
/// - Redis connection fails
/// - Server shuts down
/// 
/// Update frequency: 1 second (configurable)
/// 
/// Data format sent to client:
/// ```json
/// [
///   {
///     "temp": 22.5,
///     "hum": 45.0,
///     "motion": false,
///     "sound_db": 35.2,
///     "deviceid": "pi-001",
///     "timestamp": "2024-12-26T15:30:00Z"
///   },
///   // ... (up to 100 readings)
/// ]
/// ```
#[tracing::instrument(skip_all)]
async fn ws_loop(mut socket: WebSocket, state: AppState) {
    tracing::info!("WebSocket client connected");
    
    // Subscribe to broadcast channel for instant notifications
    let mut rx = state.broadcast_tx.subscribe();
    
    let mut message_count = 0_u64;
    let mut last_full_sync = tokio::time::Instant::now();
    
    loop {
        // Hybrid approach: instant notifications + periodic full sync
        // - Instant: Send new data immediately when Pi sends
        // - Fallback: Send full cache every 5 seconds in case notifications missed
        
        tokio::select! {
            // Branch 1: New data notification (instant, zero latency)
            Ok(new_data) = rx.recv() => {
                tracing::debug!("Received instant notification for new sensor data");
                
                // Send single new reading immediately
                match serde_json::to_string(&vec![new_data]) {
                    Ok(json) => {
                        if socket.send(Message::Text(json)).await.is_err() {
                            tracing::info!("WebSocket client disconnected (sent {} messages)", message_count);
                            break;
                        }
                        message_count += 1;
                        
                        if message_count % 60 == 0 {
                            tracing::debug!("WebSocket: sent {} instant updates", message_count);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to serialize new data: {}", e);
                    }
                }
            }
            
            // Branch 2: Periodic full sync (every 5 seconds as fallback)
            _ = tokio::time::sleep_until(last_full_sync + tokio::time::Duration::from_secs(5)) => {
                tracing::debug!("Performing periodic full sync from Redis");
                
                // Read last 100 readings from Redis
                let readings_result: Result<Vec<String>, redis::RedisError> = redis::cmd("LRANGE")
                    .arg("sensor:latest")
                    .arg(0)
                    .arg(99)
                    .query_async(&mut state.realtime.redis.clone())
                    .await;
                
                match readings_result {
                    Ok(readings_json) => {
                        let mut data: Vec<SensorData> = Vec::new();
                        for json_str in readings_json {
                            if let Ok(sensor_data) = serde_json::from_str::<SensorData>(&json_str) {
                                data.push(sensor_data);
                            }
                        }
                        
                        match serde_json::to_string(&data) {
                            Ok(json) => {
                                if socket.send(Message::Text(json)).await.is_err() {
                                    tracing::info!("WebSocket client disconnected during sync");
                                    break;
                                }
                                tracing::debug!("Full sync sent: {} readings", data.len());
                            }
                            Err(e) => {
                                tracing::error!("Failed to serialize full sync: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Redis error during full sync: {}", e);
                    }
                }
                
                last_full_sync = tokio::time::Instant::now();
            }
        }
    }
}
