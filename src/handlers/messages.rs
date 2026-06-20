use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use crate::{AppState, error::AppError, models::*};
use crate::services::messages as message_service;

pub async fn list_messages(
    State(state): State<AppState>,
) -> Result<Json<Vec<Message>>, AppError> {
    // In real implementation, extract user_id from JWT token
    let user_id = Uuid::new_v4(); // Placeholder

    let messages = message_service::get_user_messages(&state.db, user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to list messages for user {user_id}: {e}");
            AppError::from(e)
        })?;
    Ok(Json(messages))
}

pub async fn send_message(
    State(state): State<AppState>,
    Json(request): Json<SendMessageRequest>,
) -> Result<Json<Message>, AppError> {
    // In real implementation, extract sender_id from JWT token
    let sender_id = Uuid::new_v4(); // Placeholder

    let message = message_service::send_message(&state.db, sender_id, request)
        .await
        .map_err(|e| {
            tracing::error!("Failed to send message: {e}");
            AppError::from(e)
        })?;

    // Send WebSocket notification to recipient
    match serde_json::to_value(&message) {
        Ok(message_data) => {
            state.websocket_state.send_message_notification(&message.recipient_id, message_data).await;
        }
        Err(e) => {
            tracing::warn!("Failed to serialize message for WebSocket notification: {e}");
        }
    }

    Ok(Json(message))
}

pub async fn mark_as_read(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    message_service::mark_message_as_read(&state.db, id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to mark message {id} as read: {e}");
            AppError::from(e)
        })?;
    Ok(StatusCode::OK)
}
