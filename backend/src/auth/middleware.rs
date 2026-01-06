/// Authentication Middleware
/// 
/// Protects API endpoints by validating JWT tokens from Authorization headers.
/// Adds device_id to request extensions for use in handlers.

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::auth::jwt::verify_token;
use crate::error::{ApiError, ApiResult};

/// Extract JWT token from Authorization header
/// 
/// Expects format: "Bearer <token>"
fn extract_token(headers: &HeaderMap) -> ApiResult<String> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiError::Unauthorized("Missing Authorization header".to_string()))?;
    
    if !auth_header.starts_with("Bearer ") {
        return Err(ApiError::Unauthorized(
            "Invalid Authorization header format. Use: Bearer <token>".to_string(),
        ));
    }
    
    let token = auth_header.trim_start_matches("Bearer ").to_string();
    
    if token.is_empty() {
        return Err(ApiError::Unauthorized("Empty token".to_string()));
    }
    
    Ok(token)
}

/// Authentication middleware function
/// 
/// Validates JWT token and adds claims to request extensions.
/// Returns 401 Unauthorized if token is missing, invalid, or expired.
pub async fn auth_middleware(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // Get JWT secret from environment
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default-secret-change-in-production".to_string());
    
    // Extract token from header
    let token = match extract_token(&headers) {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!("Auth failed: {}", e);
            return Err((StatusCode::UNAUTHORIZED, e.to_string()));
        }
    };
    
    // Verify token
    let claims = match verify_token(&token, &jwt_secret) {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Token verification failed: {}", e);
            return Err((StatusCode::UNAUTHORIZED, e.to_string()));
        }
    };
    
    tracing::debug!("Authenticated request from device: {}", claims.sub);
    
    // Add claims to request extensions for use in handlers
    req.extensions_mut().insert(claims);
    
    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;
    
    #[test]
    fn test_extract_token_valid() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_static("Bearer my.jwt.token"),
        );
        
        let token = extract_token(&headers).unwrap();
        assert_eq!(token, "my.jwt.token");
    }
    
    #[test]
    fn test_extract_token_missing() {
        let headers = HeaderMap::new();
        let result = extract_token(&headers);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_extract_token_invalid_format() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_static("InvalidFormat token"),
        );
        
        let result = extract_token(&headers);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_extract_token_empty() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_static("Bearer "),
        );
        
        let result = extract_token(&headers);
        assert!(result.is_err());
    }
}
