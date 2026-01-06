/// Health check endpoint
/// 
/// Provides system health status for monitoring and load balancers.
/// Checks:
/// - Redis connection status
/// - System uptime
/// - Basic diagnostics

use axum::{extract::State, Json};
use serde::Serialize;
use std::time::SystemTime;

use crate::error::{ApiError, ApiResult};
use crate::routes::sensor_data::AppState;

/// Health check response structure
/// 
/// Example response:
/// ```json
/// {
///   "status": "healthy",
///   "redis": "connected",
///   "uptime_seconds": 3600,
///   "timestamp": "2024-12-26T20:30:00Z"
/// }
/// ```
#[derive(Serialize)]
pub struct HealthResponse {
    /// Overall system status: "healthy" or "unhealthy"
    status: String,
    /// Redis connection status: "connected" or "disconnected"
    redis: String,
    /// Server uptime in seconds
    uptime_seconds: u64,
    /// Current ISO 8601 timestamp
    timestamp: String,
}

/// Lazy static for tracking server start time
static START_TIME: std::sync::OnceLock<SystemTime> = std::sync::OnceLock::new();

/// Initialize start time (called once at startup)
pub fn init_start_time() {
    START_TIME.get_or_init(SystemTime::now);
}

/// GET /health
/// 
/// Health check endpoint for monitoring and load balancers.
/// 
/// Returns:
/// - 200 OK: System is healthy, Redis connected
/// - 503 Service Unavailable: Redis connection failed
/// 
/// Response body contains detailed status information.
/// 
/// Example usage:
/// ```bash
/// curl http://localhost:3000/health
/// ```
#[tracing::instrument(skip(state))]
pub async fn health_check(State(state): State<AppState>) -> ApiResult<Json<HealthResponse>> {
    tracing::debug!("Health check requested");

    // Check Redis connection with PING command
    let redis_status = match redis::cmd("PING")
        .query_async::<_, String>(&mut state.realtime.redis.clone())
        .await
    {
        Ok(response) if response == "PONG" => {
            tracing::debug!("Redis health check: OK");
            "connected"
        }
        Ok(response) => {
            tracing::warn!("Redis health check: unexpected response: {}", response);
            "disconnected"
        }
        Err(e) => {
            tracing::error!("Redis health check failed: {}", e);
            "disconnected"
        }
    };

    // Calculate uptime
    let now = SystemTime::now();
    let default_start = now.clone();
    let start_time = START_TIME.get().unwrap_or(&default_start);
    let uptime = now
        .duration_since(*start_time)
        .unwrap_or_default()
        .as_secs();

    // Determine overall status
    let overall_status = if redis_status == "connected" {
        "healthy"
    } else {
        "unhealthy"
    };

    // Generate timestamp
    let timestamp = chrono::Utc::now().to_rfc3339();

    let response = HealthResponse {
        status: overall_status.to_string(),
        redis: redis_status.to_string(),
        uptime_seconds: uptime,
        timestamp,
    };

    // Return 503 if unhealthy (for load balancers)
    if overall_status == "unhealthy" {
        tracing::warn!("Health check: UNHEALTHY - Redis disconnected");
        return Err(ApiError::Internal(
            "Service unhealthy: Redis disconnected".to_string(),
        ));
    }

    tracing::debug!("Health check: HEALTHY");
    Ok(Json(response))
}
