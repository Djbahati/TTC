use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use crate::{AppState, models::*};
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
    State(_state): State<AppState>,
) -> Result<Json<UserInfo>, StatusCode> {
    // This would extract user info from JWT token
    // For now, return a placeholder
    Err(StatusCode::NOT_IMPLEMENTED)
}
