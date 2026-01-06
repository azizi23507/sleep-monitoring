/// Authentication Module
/// 
/// Provides JWT-based authentication for API endpoints.
/// Includes token generation, validation, and middleware.

pub mod jwt;
pub mod middleware;

// Re-exports for external use
#[allow(unused_imports)]
pub use jwt::{create_token, verify_token, Claims};
#[allow(unused_imports)]
pub use middleware::auth_middleware;
