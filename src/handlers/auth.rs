use axum::{
    extract::State,
    response::Json,
};
use crate::{AppState, error::AppError, models::*};
use crate::services::auth as auth_service;

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let response = auth_service::authenticate_user(&state.db, request)
        .await
        .map_err(|e| {
            tracing::warn!("Login failed: {e}");
            AppError::Unauthorized("Invalid email or password".to_string())
        })?;
    Ok(Json(response))
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<UserInfo>, AppError> {
    let user_info = auth_service::register_user(&state.db, request)
        .await
        .map_err(|e| {
            tracing::error!("Registration failed: {e}");
            AppError::BadRequest(format!("Registration failed: {e}"))
        })?;
    Ok(Json(user_info))
}

pub async fn me(
    State(_state): State<AppState>,
) -> Result<Json<UserInfo>, AppError> {
    // This would extract user info from JWT token
    Err(AppError::Internal("User info extraction not yet implemented".to_string()))
}
