use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use crate::{AppState, models::*};
use crate::services::appointments as appointment_service;

pub async fn list_appointments(
    State(state): State<AppState>,
) -> Result<Json<Vec<Appointment>>, StatusCode> {
    match appointment_service::get_appointments(&state.db).await {
        Ok(appointments) => Ok(Json(appointments)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_appointment(
    State(state): State<AppState>,
    Json(request): Json<CreateAppointmentRequest>,
) -> Result<Json<Appointment>, StatusCode> {
    match appointment_service::create_appointment(&state.db, request).await {
        Ok(appointment) => {
            // Broadcast appointment update via WebSocket
            let appointment_data = serde_json::to_value(&appointment).unwrap();
            state.websocket_state.broadcast_appointment_update(appointment_data).await;
            
            Ok(Json(appointment))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_appointment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Appointment>, StatusCode> {
    match appointment_service::get_appointment_by_id(&state.db, id).await {
        Ok(Some(appointment)) => Ok(Json(appointment)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_appointment_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateAppointmentStatusRequest>,
) -> Result<Json<Appointment>, StatusCode> {
    match appointment_service::update_appointment_status(&state.db, id, request).await {
        Ok(appointment) => {
            // Broadcast appointment update via WebSocket
            let appointment_data = serde_json::to_value(&appointment).unwrap();
            state.websocket_state.broadcast_appointment_update(appointment_data).await;
            
            Ok(Json(appointment))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
