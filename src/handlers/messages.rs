use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use crate::{AppState, models::*};
use crate::auth::AuthenticatedUser;
use crate::services::messages as message_service;

pub async fn list_messages(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<Vec<Message>>, StatusCode> {
    let authenticated_user = request
        .extensions()
        .get::<AuthenticatedUser>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user_id = authenticated_user.user_id;

    match message_service::get_user_messages(&state.db, user_id).await {
        Ok(messages) => Ok(Json(messages)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn send_message(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<Message>, StatusCode> {
    let authenticated_user = request
        .extensions()
        .get::<AuthenticatedUser>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let sender_id = authenticated_user.user_id;

    // Extract the JSON body manually since we already consumed request for extensions
    let body = axum::body::to_bytes(request.into_body(), 1024 * 1024)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let send_request: SendMessageRequest = serde_json::from_slice(&body)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    match message_service::send_message(&state.db, sender_id, send_request).await {
        Ok(message) => {
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
