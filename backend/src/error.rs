/// Custom error types for the application
/// 
/// This module defines structured error types using thiserror for better
/// error handling and more informative error messages.
/// 
/// Error Types:
/// - ApiError: High-level errors returned to clients
/// - ValidationError: Input validation failures
/// - RedisError: Redis operation failures
/// - InternalError: Unexpected internal errors

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// Main API error type
/// 
/// This is the top-level error type that gets converted to HTTP responses.
/// All other error types should be convertible to ApiError.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Error response sent to clients
/// 
/// This structure is serialized to JSON and sent as the HTTP response body.
/// 
/// Example:
/// ```json
/// {
///   "error": "Validation failed: temperature out of range",
///   "status": 400
/// }
/// ```
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    status: u16,
}

impl IntoResponse for ApiError {
    /// Convert ApiError to HTTP response
    /// 
    /// Maps error types to appropriate HTTP status codes:
    /// - Validation: 400 Bad Request
    /// - Unauthorized: 401 Unauthorized
    /// - Redis: 503 Service Unavailable
    /// - Internal: 500 Internal Server Error
    /// - Serialization: 500 Internal Server Error
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Redis(e) => (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Redis error: {}", e),
            ),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::Serialization(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Serialization error: {}", e),
            ),
        };

        let body = Json(ErrorResponse {
            error: message,
            status: status.as_u16(),
        });

        (status, body).into_response()
    }
}

/// Result type alias for API operations
/// 
/// Use this instead of Result<T, E> throughout the application.
/// 
/// Example:
/// ```rust
/// pub async fn my_handler() -> ApiResult<Json<MyData>> {
///     let data = fetch_data().await?;
///     Ok(Json(data))
/// }
/// ```
pub type ApiResult<T> = Result<T, ApiError>;
