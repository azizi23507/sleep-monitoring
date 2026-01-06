/// JWT Token Generation and Validation
/// 
/// Handles creation and verification of JWT tokens for API authentication.
/// Tokens are valid for 24 hours and contain device/patient ID in claims.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{ApiError, ApiResult};

/// JWT Claims structure
/// Contains the subject (device/patient ID) and expiration timestamp
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject - device ID or patient ID
    pub sub: String,
    
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    
    /// Issued at (Unix timestamp)
    pub iat: usize,
}

/// Create a new JWT token for a device/patient
/// 
/// # Arguments
/// * `device_id` - The device or patient identifier
/// * `secret` - JWT secret key from environment
/// 
/// # Returns
/// * JWT token string valid for 24 hours
pub fn create_token(device_id: &str, secret: &str) -> ApiResult<String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| ApiError::Internal(format!("System time error: {}", e)))?
        .as_secs() as usize;
    
    // Token valid for 24 hours
    let expiration = now + 86400; // 24 * 60 * 60
    
    let claims = Claims {
        sub: device_id.to_string(),
        exp: expiration,
        iat: now,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        tracing::error!("Failed to create JWT token: {}", e);
        ApiError::Internal("Token generation failed".to_string())
    })
}

/// Verify and decode a JWT token
/// 
/// # Arguments
/// * `token` - JWT token string
/// * `secret` - JWT secret key from environment
/// 
/// # Returns
/// * Validated claims if token is valid
/// * Error if token is invalid, expired, or malformed
pub fn verify_token(token: &str, secret: &str) -> ApiResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| {
        tracing::warn!("JWT verification failed: {}", e);
        ApiError::Unauthorized("Invalid or expired token".to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_token() {
        let secret = "test-secret-key-12345";
        let device_id = "device-001";
        
        // Create token
        let token = create_token(device_id, secret).unwrap();
        assert!(!token.is_empty());
        
        // Verify token
        let claims = verify_token(&token, secret).unwrap();
        assert_eq!(claims.sub, device_id);
        assert!(claims.exp > claims.iat);
    }
    
    #[test]
    fn test_verify_invalid_token() {
        let secret = "test-secret-key-12345";
        let result = verify_token("invalid.token.here", secret);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_verify_wrong_secret() {
        let secret = "test-secret-key-12345";
        let device_id = "device-001";
        
        let token = create_token(device_id, secret).unwrap();
        
        // Try to verify with wrong secret
        let wrong_secret = "wrong-secret";
        let result = verify_token(&token, wrong_secret);
        assert!(result.is_err());
    }
}
