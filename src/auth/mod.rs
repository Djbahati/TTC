// JWT authentication middleware and utilities
use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(_state): State<crate::AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract JWT token from Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            // Validate JWT token here
            // For now, just pass through
            return Ok(next.run(request).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
