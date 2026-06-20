// JWT authentication middleware and utilities
use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::services::auth::validate_jwt_token;

/// Authenticated user info extracted from a valid JWT token.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub role: String,
}

pub async fn auth_middleware(
    State(_state): State<crate::AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];

            match validate_jwt_token(token) {
                Ok(claims) => {
                    let user_id = claims.sub.parse::<uuid::Uuid>()
                        .map_err(|_| StatusCode::UNAUTHORIZED)?;

                    let authenticated_user = AuthenticatedUser {
                        user_id,
                        email: claims.email,
                        role: claims.role,
                    };

                    request.extensions_mut().insert(authenticated_user);
                    return Ok(next.run(request).await);
                }
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
