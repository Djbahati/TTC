use axum::{
    extract::{Request, State},
    http::StatusCode,
    response::Json,
};
use crate::{AppState, models::*};
use crate::auth::AuthenticatedUser;
use crate::services::auth as auth_service;

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    match auth_service::authenticate_user(&state.db, request).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<UserInfo>, StatusCode> {
    match auth_service::register_user(&state.db, request).await {
        Ok(user_info) => Ok(Json(user_info)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn me(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<UserInfo>, StatusCode> {
    let authenticated_user = request
        .extensions()
        .get::<AuthenticatedUser>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id, email, password_hash, role as "role: UserRole",
            first_name, last_name, phone, is_active, last_login, created_at, updated_at
        FROM users 
        WHERE id = $1 AND is_active = true
        "#,
        authenticated_user.user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(user) => Ok(Json(UserInfo {
            id: user.id,
            email: user.email,
            role: user.role,
            first_name: user.first_name,
            last_name: user.last_name,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}
