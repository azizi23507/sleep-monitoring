/// Authentication Endpoints
/// 
/// Handles JWT token generation for devices/patients.

use axum::Json;
use serde::{Deserialize, Serialize};

use crate::auth::jwt::create_token;
use crate::error::{ApiError, ApiResult};

/// Token request payload
#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    /// Device ID or patient ID
    pub device_id: String,
}

/// Token response
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    /// JWT token (valid for 24 hours)
    pub token: String,
    
    /// Token type (always "Bearer")
    pub token_type: String,
    
    /// Expiration time in seconds (86400 = 24 hours)
    pub expires_in: usize,
}

/// POST /api/auth/token
/// 
/// Generate JWT token for a device/patient.
/// 
/// # Request Body
/// ```json
/// {
///   "device_id": "pi-001"
/// }
/// ```
/// 
/// # Response
/// ```json
/// {
///   "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
///   "token_type": "Bearer",
///   "expires_in": 86400
/// }
/// ```
#[tracing::instrument(skip_all)]
pub async fn get_token(
    Json(request): Json<TokenRequest>,
) -> ApiResult<Json<TokenResponse>> {
    tracing::info!("Token request for device: {}", request.device_id);
    
    // Validate device_id
    if request.device_id.is_empty() {
        return Err(ApiError::Validation(
            "device_id cannot be empty".to_string(),
        ));
    }
    
    // Get JWT secret from environment (required - no fallback for security)
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in environment variables");
    
    // Generate token
    let token = create_token(&request.device_id, &jwt_secret)?;
    
    tracing::info!("Token generated successfully for: {}", request.device_id);
    
    Ok(Json(TokenResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: 86400, // 24 hours
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_token_valid() {
        let request = TokenRequest {
            device_id: "test-device-001".to_string(),
        };
        
        let result = get_token(Json(request)).await;
        assert!(result.is_ok());
        
        let response = result.unwrap().0;
        assert!(!response.token.is_empty());
        assert_eq!(response.token_type, "Bearer");
        assert_eq!(response.expires_in, 86400);
    }
    
    #[tokio::test]
    async fn test_get_token_empty_device_id() {
        let request = TokenRequest {
            device_id: "".to_string(),
        };
        
        let result = get_token(Json(request)).await;
        assert!(result.is_err());
    }
}
