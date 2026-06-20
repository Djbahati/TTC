// JWT authentication middleware and utilities
use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use tracing::warn;

use crate::error::AppError;

pub async fn auth_middleware(
    State(_state): State<crate::AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let _token = &header[7..];
            // Validate JWT token here
            // For now, just pass through
            Ok(next.run(request).await)
        }
        Some(_) => {
            warn!("Invalid authorization header format (expected 'Bearer <token>')");
            Err(AppError::Unauthorized("Invalid authorization header format".to_string()))
        }
        None => {
            warn!("Missing authorization header");
            Err(AppError::Unauthorized("Missing authorization header".to_string()))
        }
    }
}
