use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use crate::{AppState, models::*};
use crate::services::messages as message_service;

pub async fn list_messages(
    State(state): State<AppState>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    // In real implementation, extract user_id from JWT token
    let user_id = Uuid::new_v4(); // Placeholder
    
    match message_service::get_user_messages(&state.db, user_id).await {
        Ok(messages) => Ok(Json(messages)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn send_message(
    State(state): State<AppState>,
    Json(request): Json<SendMessageRequest>,
) -> Result<Json<Message>, StatusCode> {
    // In real implementation, extract sender_id from JWT token
    let sender_id = Uuid::new_v4(); // Placeholder
    
    match message_service::send_message(&state.db, sender_id, request).await {
        Ok(message) => {
            // Send WebSocket notification to recipient
            let message_data = serde_json::to_value(&message).unwrap();
            state.websocket_state.send_message_notification(&message.recipient_id, message_data).await;
            
            Ok(Json(message))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn mark_as_read(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    match message_service::mark_message_as_read(&state.db, id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
