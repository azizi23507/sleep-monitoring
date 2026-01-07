use axum::{
    Router,
    routing::{get, post},
    middleware,
};
use std::path::PathBuf;
use tower_http::services::ServeDir;
pub mod auth;
pub mod fhir_api;
pub mod health;
pub mod ml_results;
pub mod sensor_data;

use crate::routes::sensor_data::{AppState, ingest_sensor_data};
use crate::routes::health::health_check;
use crate::routes::fhir_api::{get_observation_by_id, search_observations};
use crate::routes::ml_results::{get_sleep_records, get_sleep_record_by_date, get_latest_sleep_quality};
use crate::routes::auth::get_token;
use crate::websocket::ws_handler;

/// Builds the main application router with all routes and middleware
/// 
/// Routes:
/// - POST /api/auth/token: Get JWT token (NO AUTH REQUIRED)
/// - POST /api/sensor-data: Ingest sensor data from Raspberry Pi (AUTH REQUIRED)
/// - GET /ws: WebSocket endpoint for real-time data streaming (AUTH REQUIRED)
/// - GET /health: Health check endpoint (NO AUTH REQUIRED)
/// - FHIR API routes (AUTH REQUIRED)
/// - ML Results API routes (AUTH REQUIRED)
/// - GET /: Serve index.html from frontend (NO AUTH REQUIRED)
/// - Static files: /js and /css directories (NO AUTH REQUIRED)
/// 
/// State:
/// - realtime: Real-time buffer for WebSocket streaming (Branch 1)
/// - db_pool: PostgreSQL connection pool (Branch 2)
/// - broadcast_tx: Instant notification channel for WebSocket (Branch 1 improvement)
pub fn build_router(
    realtime: crate::websocket::RealtimeState, 
    db_pool: sqlx::PgPool,
    broadcast_tx: tokio::sync::broadcast::Sender<crate::models::sensor_data::SensorData>,
) -> Router {
    // Global application state shared across all routes
    let app_state = AppState { realtime, db_pool, broadcast_tx };

    // Path to frontend (sibling folder of backend)
    // Assumes directory structure:
    // project/
    //   ├── backend/
    //   └── frontend/
    let frontend_root: PathBuf =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../frontend");

    // ONLY Pi sensor data endpoint requires authentication
    let protected_routes = Router::new()
        .route("/api/sensor-data", post(ingest_sensor_data))
        .layer(middleware::from_fn(crate::auth::middleware::auth_middleware));

    // Combine all routes
    Router::new()
        // PUBLIC routes (NO authentication required)
        .route("/health", get(health_check))
        .route("/api/auth/token", post(get_token))
        .route("/", get(index))
        
        // WebSocket - PUBLIC (no auth)
        .route("/ws", get(ws_handler))
        
        // FHIR API - PUBLIC (no auth)
        .route("/api/fhir/Observation/:id", get(get_observation_by_id))
        .route("/api/fhir/Observation", get(search_observations))
        
        // ML Results API - PUBLIC (no auth)
        .route("/api/sleep-records", get(get_sleep_records))
        .route("/api/sleep-records/:date", get(get_sleep_record_by_date))
        .route("/api/sleep-quality/latest", get(get_latest_sleep_quality))
        
        // Merge protected route (only sensor-data)
        .merge(protected_routes)
        
        // Static files (JavaScript and CSS - no auth required)
        .nest_service("/js", ServeDir::new(frontend_root.join("js")))
        .nest_service("/css", ServeDir::new(frontend_root.join("css")))
        
        // Attach application state
        .with_state(app_state)
}

/// Serves the main index.html file
/// 
/// This function is called when the user visits the root URL (/)
/// It returns the HTML content from the filesystem
async fn index() -> axum::response::Html<String> {
    let html = std::fs::read_to_string("/app/frontend/index.html")
        .unwrap_or_else(|_| "<h1>Frontend not found</h1>".to_string());
    axum::response::Html(html)
}
