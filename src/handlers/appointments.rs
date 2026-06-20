use axum::{
    extract::{Path, State},
    response::Json,
};
use uuid::Uuid;
use crate::{AppState, error::AppError, models::*};
use crate::services::appointments as appointment_service;

pub async fn list_appointments(
    State(state): State<AppState>,
) -> Result<Json<Vec<Appointment>>, AppError> {
    let appointments = appointment_service::get_appointments(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to list appointments: {e}");
            AppError::from(e)
        })?;
    Ok(Json(appointments))
}

pub async fn create_appointment(
    State(state): State<AppState>,
    Json(request): Json<CreateAppointmentRequest>,
) -> Result<Json<Appointment>, AppError> {
    let appointment = appointment_service::create_appointment(&state.db, request)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create appointment: {e}");
            AppError::from(e)
        })?;

    // Broadcast appointment update via WebSocket
    match serde_json::to_value(&appointment) {
        Ok(appointment_data) => {
            state.websocket_state.broadcast_appointment_update(appointment_data).await;
        }
        Err(e) => {
            tracing::warn!("Failed to serialize appointment for WebSocket broadcast: {e}");
        }
    }

    Ok(Json(appointment))
}

pub async fn get_appointment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Appointment>, AppError> {
    let appointment = appointment_service::get_appointment_by_id(&state.db, id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get appointment {id}: {e}");
            AppError::from(e)
        })?;

    match appointment {
        Some(appt) => Ok(Json(appt)),
        None => Err(AppError::NotFound(format!("Appointment {id} not found"))),
    }
}

pub async fn update_appointment_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateAppointmentStatusRequest>,
) -> Result<Json<Appointment>, AppError> {
    let appointment = appointment_service::update_appointment_status(&state.db, id, request)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update appointment status for {id}: {e}");
            AppError::from(e)
        })?;

    // Broadcast appointment update via WebSocket
    match serde_json::to_value(&appointment) {
        Ok(appointment_data) => {
            state.websocket_state.broadcast_appointment_update(appointment_data).await;
        }
        Err(e) => {
            tracing::warn!("Failed to serialize appointment for WebSocket broadcast: {e}");
        }
    }

    Ok(Json(appointment))
}
