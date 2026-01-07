use axum::Router;
use tower_http::cors::Any;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod error;
mod fhir;
mod models;
mod routes;
mod validation;
mod websocket;

/// Main entry point for the sleep monitoring backend
/// 
/// Architecture:
/// - Axum web framework (async Rust)
/// - WebSocket for real-time data streaming
/// - Redis cache for last 100 readings
/// - CORS enabled for frontend access
/// - Structured logging with tracing
/// 
/// Server runs on: http://0.0.0.0:3000
#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for structured logging
    // This enables logs with different levels (trace, debug, info, warn, error)
    // Set log level via RUST_LOG environment variable:
    //   RUST_LOG=debug cargo run
    //   RUST_LOG=info cargo run (default)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sleep_backend=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Sleep Monitoring Backend");

    // Initialize health check start time
    routes::health::init_start_time();

    // ========================================
    // PostgreSQL Connection (Branch 2)
    // ========================================
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/sleep_monitor".to_string());
    
    tracing::info!("Connecting to PostgreSQL...");
    
    let db_pool = match sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("PostgreSQL connected successfully");
            pool
        }
        Err(e) => {
            tracing::error!("Failed to connect to PostgreSQL: {}", e);
            tracing::error!("   Make sure PostgreSQL is running");
            tracing::error!("   And database 'sleep_monitor' exists");
            tracing::error!("   Set DATABASE_URL environment variable if needed");
            std::process::exit(1);
        }
    };

    // Run migrations (create tables if they don't exist)
    // Migration files are embedded at compile time from ./migrations
    tracing::info!("Running database migrations...");
    match sqlx::migrate!("./migrations").run(&db_pool).await {
        Ok(_) => tracing::info!("Database migrations complete"),
        Err(e) => {
            tracing::error!("Migration failed: {}", e);
            tracing::warn!("Migration error (may be okay if tables exist): {}", e);
        }
    }

    // ========================================
    // Redis Connection (Branch 1)
    // ========================================
    // Default: localhost:6379 (standard Redis port)
    // Can be configured via environment variable: REDIS_URL
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    
    tracing::info!("Connecting to Redis at: {}", redis_url);
    
    // Initialize real-time state with Redis connection (Branch 1)
    let realtime_state = match websocket::RealtimeState::new(&redis_url).await {
        Ok(state) => {
            tracing::info!("Redis connected successfully");
            state
        }
        Err(e) => {
            tracing::error!("Failed to connect to Redis: {}", e);
            tracing::error!("   Make sure Redis is running: redis-server");
            tracing::error!("   Or install Redis: sudo apt-get install redis-server");
            std::process::exit(1);
        }
    };

    // Build router with all routes
    // Create broadcast channel for instant WebSocket notifications
    // Capacity: 100 messages (if WebSocket can't keep up, oldest messages are dropped)
    let (broadcast_tx, _) = tokio::sync::broadcast::channel(100);
    
    let app: Router = routes::build_router(realtime_state, db_pool, broadcast_tx)
        .layer(tower_http::cors::CorsLayer::new().allow_origin(Any));

    tracing::info!("");
    tracing::info!("Sleep Monitoring Backend - READY");
    tracing::info!("   Server: http://0.0.0.0:3000");
    tracing::info!("   WebSocket: ws://0.0.0.0:3000/ws");
    tracing::info!("   API: POST /api/sensor-data");
    tracing::info!("   Health: GET /health");
    tracing::info!("   PostgreSQL: Connected");
    tracing::info!("   Redis: Connected");
    tracing::info!("");

    // Bind to port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");

    // Start server
    tracing::info!("Server listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server crashed");
}
