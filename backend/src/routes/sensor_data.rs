use axum::{Json, extract::State};

use crate::{models::sensor_data::SensorData, validation::sensor::validate_sensor_data};
use crate::websocket::RealtimeState;
use crate::error::{ApiError, ApiResult};

/// Application state shared across all routes
/// 
/// Contains:
/// - realtime: Redis connection for WebSocket streaming (Branch 1)
/// - db_pool: PostgreSQL connection pool (Branch 2)
/// - broadcast_tx: Channel for instant WebSocket notifications (Branch 1 improvement)
/// 
/// Future additions:
/// - None (all major components now included)
#[derive(Clone)]
pub struct AppState {
    /// Redis connection for real-time cache (Branch 1)
    pub realtime: RealtimeState,
    
    /// PostgreSQL connection pool (Branch 2)
    pub db_pool: sqlx::PgPool,
    
    /// Broadcast channel for instant WebSocket notifications
    pub broadcast_tx: tokio::sync::broadcast::Sender<SensorData>,
}

/// POST /api/sensor-data
/// 
/// Receives sensor data from Raspberry Pi and processes it through
/// the 3-branch architecture.
/// 
/// Current implementation (Branch 1 only):
/// 1. Validate sensor data (check ranges)
/// 2. Store in Redis cache (last 100 readings)
/// 3. Notify WebSocket clients (event-driven)
/// 4. Return OK status
/// 
/// Future implementation (Branches 2A & 2B):
/// - Branch 2A: Store in PostgreSQL → Convert to FHIR → Expose via API
/// - Branch 2B: Store in PostgreSQL → Trigger ML processing (nightly)
/// 
/// Request body (JSON):
/// ```json
/// {
///   "temp": 22.5,
///   "hum": 45.0,
///   "motion": false,
///   "sound_db": 35.2,
///   "deviceid": "pi-001",
///   "timestamp": "2024-12-26T15:30:00Z"
/// }
/// ```
/// 
/// Response:
/// - 200 OK: Data accepted and stored
/// - 400 Bad Request: Validation failed (out of range values)
/// - 503 Service Unavailable: Redis connection failed
#[tracing::instrument(skip(state), fields(device_id = %data.deviceid))]
pub async fn ingest_sensor_data(
    State(state): State<AppState>,
    Json(data): Json<SensorData>,
) -> ApiResult<Json<serde_json::Value>> {
    tracing::debug!("Received sensor data");

    // ========================================
    // STEP 1: Validate sensor data
    // ========================================
    // Check if all values are within acceptable ranges:
    // - Temperature: -50 to 50°C
    // - Humidity: 0 to 100%
    // - Sound: 0 to 120 dB
    validate_sensor_data(&data).map_err(|e| {
        tracing::warn!("Validation failed: {}", e);
        ApiError::Validation(e)
    })?;

    tracing::debug!(
        "Validated: temp={:.1}°C, hum={:.1}%, sound={:.1}dB, motion={}",
        data.temp,
        data.hum,
        data.sound_db,
        data.motion
    );

    // ========================================
    // STEP 2: Store in Redis cache (Branch 1)
    // ========================================
    // This is the "Real-time Path" (Branch 1) from documentation:
    // Backend → Redis Cache → WebSocket → Frontend
    // 
    // Purpose: Zero-latency real-time monitoring
    // - Persistent cache (survives server restarts)
    // - Shared across multiple backend instances
    // - Frontend gets instant updates
    // - Acts as backup for reconnection
    {
        // Serialize sensor data to JSON
        let json_data = serde_json::to_string(&data)?;
        
        // Store in Redis list
        // LPUSH: Add to beginning of list (newest first)
        redis::cmd("LPUSH")
            .arg("sensor:latest")
            .arg(&json_data)
            .query_async::<_, ()>(&mut state.realtime.redis.clone())
            .await
            .map_err(|e| {
                tracing::error!("Redis LPUSH failed: {}", e);
                ApiError::Redis(e)
            })?;
        
        // Trim list to keep only last 100 readings
        redis::cmd("LTRIM")
            .arg("sensor:latest")
            .arg(0)
            .arg(99)
            .query_async::<_, ()>(&mut state.realtime.redis.clone())
            .await
            .map_err(|e| {
                tracing::error!("Redis LTRIM failed: {}", e);
                ApiError::Redis(e)
            })?;
        
        // Set expiry: 2 hours (7200 seconds)
        redis::cmd("EXPIRE")
            .arg("sensor:latest")
            .arg(7200)
            .query_async::<_, ()>(&mut state.realtime.redis.clone())
            .await
            .map_err(|e| {
                tracing::error!("Redis EXPIRE failed: {}", e);
                ApiError::Redis(e)
            })?;

        tracing::info!(
            "Stored in Redis: {:.1}°C, {:.1}% hum, {:.1}dB, motion: {}",
            data.temp,
            data.hum,
            data.sound_db,
            data.motion
        );
    }

    // ========================================
    // STEP 3: Store in PostgreSQL (Branch 2)
    // ========================================
    // This enables both Branch 2A (FHIR) and Branch 2B (ML)
    // PostgreSQL provides:
    // - Persistent storage (survives restarts)
    // - Complex queries for ML processing
    // - FHIR conversion source data
    // - Historical analysis
    {
        tracing::debug!("Storing in PostgreSQL...");
        
        // Parse timestamp to proper format
        let timestamp = chrono::DateTime::parse_from_rfc3339(&data.timestamp)
            .map_err(|e| {
                tracing::error!("Invalid timestamp format: {}", e);
                ApiError::Validation(format!("Invalid timestamp: {}", e))
            })?
            .with_timezone(&chrono::Utc);
        
        // Insert into sensor_readings table
        let sensor_reading_id = sqlx::query_scalar!(
            r#"
            INSERT INTO sensor_readings 
                (device_id, temperature, humidity, sound_level, motion_detected, reading_timestamp)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            data.deviceid,
            data.temp as f64,  // Cast to f64 for DECIMAL type
            data.hum as f64,
            data.sound_db as f64,
            data.motion,
            timestamp
        )
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database insert failed: {}", e);
            ApiError::Internal(format!("Database error: {}", e))
        })?;
        
        tracing::debug!("Stored in PostgreSQL successfully with ID: {}", sensor_reading_id);
        
        // ========================================
        // STEP 3.1: FHIR Conversion (Branch 2A)
        // ========================================
        // Convert sensor data to FHIR R4 Observation resources
        // Creates 4 separate observations:
        // - Temperature (LOINC: CUSTOM-TEMP-001)
        // - Humidity (LOINC: CUSTOM-HUM-001)
        // - Sound Level (LOINC: CUSTOM-SOUND-001)
        // - Motion Detection (LOINC: CUSTOM-MOTION-001)
        tracing::debug!("Converting to FHIR observations...");
        
        let fhir_observations = crate::fhir::convert_to_fhir_observations(&data, sensor_reading_id);
        
        // Store FHIR observations in database
        crate::fhir::store_fhir_observations(&state.db_pool, sensor_reading_id, &fhir_observations)
            .await?;
        
        tracing::info!(
            "FHIR conversion complete: {} observations created",
            fhir_observations.len()
        );
    }

    // ========================================
    // STEP 4: Notify WebSocket Clients (Event-driven)
    // ========================================
    // Send instant notification to all connected WebSocket clients
    // This eliminates the 1-second polling delay
    let _ = state.broadcast_tx.send(data.clone());
    tracing::debug!("Broadcast notification sent to WebSocket clients");

    // ========================================
    // STEP 5: ML Processing Trigger (Branch 2B - Future)
    // ========================================
    // ML runs nightly at 8 AM (cron job), not on every data point
    // This ensures enough data accumulated for meaningful analysis
    //
    // Separate Python service reads from PostgreSQL:
    // 1. Query sensor_readings for specific date range
    // 2. Calculate sleep quality score
    // 3. Store results in sleep_records table
    //
    // API endpoints for ML results will be added later

    // Return success response
    Ok(Json(serde_json::json!({
        "status": "ok",
        "message": "Data received and stored successfully"
    })))
}
