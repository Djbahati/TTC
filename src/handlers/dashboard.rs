use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use crate::{AppState, models::DashboardStats};
use crate::services::dashboard as dashboard_service;

pub async fn get_dashboard_stats(
    State(state): State<AppState>,
) -> Result<Json<DashboardStats>, StatusCode> {
    match dashboard_service::get_dashboard_statistics(&state.db).await {
        Ok(stats) => Ok(Json(stats)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_live_data(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match dashboard_service::get_live_dashboard_data(&state.db).await {
        Ok(data) => {
            // Broadcast the live data to all connected WebSocket clients
            state.websocket_state.broadcast_dashboard_update(data.clone()).await;
            Ok(Json(data))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
