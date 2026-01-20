use axum::{
    extract::State,
    response::{sse::Event, Sse},
};
use futures::stream::{self, Stream};
use redis::aio::ConnectionManager;
use std::convert::Infallible;
use std::time::Duration;

use crate::models::sensor_data::SensorData;
use crate::routes::sensor_data::AppState;

/// Real-time state for SSE streaming (Branch 1)
/// 
/// This structure manages Redis cache for the last 100 sensor readings,
/// enabling zero-latency real-time monitoring for the frontend via SSE.
/// 
/// Architecture (Branch 1 - Real-time Path):
/// Pi → Backend → Redis Cache → SSE → Frontend
/// 
/// Why SSE instead of WebSocket?
/// - One-way communication (sensors → frontend only)
/// - Simpler protocol, less overhead
/// - Automatic reconnection built into browser
/// - Better for streaming updates from server
/// 
/// Redis Data Structure:
/// - Key: "sensor:latest"
/// - Type: List (LPUSH for new data, LTRIM to keep last 100)
/// - TTL: 2 hours (auto-expiry if no updates)
#[derive(Clone)]
pub struct RealtimeState {
    /// Redis connection manager for async operations
    pub redis: ConnectionManager,
}

impl RealtimeState {
    /// Create new RealtimeState with Redis connection
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let redis = ConnectionManager::new(client).await?;
        Ok(Self { redis })
    }
}

/// SSE handler - streams sensor data to client
/// 
/// GET /events
/// 
/// This endpoint establishes a Server-Sent Events connection,
/// allowing one-way real-time communication from server to frontend.
/// 
/// Flow:
/// 1. Client sends HTTP GET request to /events
/// 2. Server responds with Content-Type: text/event-stream
/// 3. Server continuously sends data events
/// 4. Connection stays open until client disconnects
/// 
/// Example JavaScript client:
/// ```javascript
/// const eventSource = new EventSource('http://localhost:3000/events');
/// eventSource.onmessage = (event) => {
///   const data = JSON.parse(event.data);
///   console.log('Received:', data);
/// };
/// ```
#[tracing::instrument(skip_all)]
pub async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    tracing::info!("SSE connection established");
    
    let stream = stream::unfold(
        (state, 0_u64, tokio::time::Instant::now()),
        move |(state, mut message_count, mut last_full_sync)| async move {
            // Hybrid approach: instant notifications + periodic full sync
            let mut rx = state.broadcast_tx.subscribe();
            
            tokio::select! {
                // Branch 1: New data notification (instant, zero latency)
                Ok(new_data) = rx.recv() => {
                    tracing::debug!("SSE: Sending instant notification");
                    
                    match serde_json::to_string(&vec![new_data]) {
                        Ok(json) => {
                            message_count += 1;
                            
                            if message_count % 60 == 0 {
                                tracing::debug!("SSE: sent {} instant updates", message_count);
                            }
                            
                            Some((
                                Ok(Event::default().data(json)),
                                (state, message_count, last_full_sync)
                            ))
                        }
                        Err(e) => {
                            tracing::error!("Failed to serialize SSE data: {}", e);
                            None
                        }
                    }
                }
                
                // Branch 2: Periodic full sync (every 5 seconds as fallback)
                _ = tokio::time::sleep_until(last_full_sync + Duration::from_secs(5)) => {
                    tracing::debug!("SSE: Performing periodic full sync");
                    
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
                                    last_full_sync = tokio::time::Instant::now();
                                    tracing::debug!("SSE: Full sync sent - {} readings", data.len());
                                    
                                    Some((
                                        Ok(Event::default().data(json)),
                                        (state, message_count, last_full_sync)
                                    ))
                                }
                                Err(e) => {
                                    tracing::error!("Failed to serialize full sync: {}", e);
                                    None
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Redis error during full sync: {}", e);
                            None
                        }
                    }
                }
            }
        },
    );
    
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive")
    )
}
