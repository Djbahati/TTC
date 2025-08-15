use sqlx::PgPool;
use uuid::Uuid;
use crate::models::*;
use anyhow::Result;

pub async fn get_appointments(pool: &PgPool) -> Result<Vec<Appointment>> {
    let appointments = sqlx::query_as!(
        Appointment,
        r#"
        SELECT 
            id, patient_id, doctor_id, appointment_date, appointment_time,
            duration, status as "status: AppointmentStatus", reason, notes,
            created_by, created_at, updated_at
        FROM appointments 
        ORDER BY appointment_date DESC, appointment_time DESC
        LIMIT 100
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(appointments)
}

pub async fn create_appointment(
    pool: &PgPool,
    request: CreateAppointmentRequest,
) -> Result<Appointment> {
    let appointment = sqlx::query_as!(
        Appointment,
        r#"
        INSERT INTO appointments (patient_id, doctor_id, appointment_date, appointment_time, reason)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING 
            id, patient_id, doctor_id, appointment_date, appointment_time,
            duration, status as "status: AppointmentStatus", reason, notes,
            created_by, created_at, updated_at
        "#,
        request.patient_id,
        request.doctor_id,
        request.appointment_date,
        request.appointment_time,
        request.reason
    )
    .fetch_one(pool)
    .await?;

    Ok(appointment)
}

pub async fn get_appointment_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Appointment>> {
    let appointment = sqlx::query_as!(
        Appointment,
        r#"
        SELECT 
            id, patient_id, doctor_id, appointment_date, appointment_time,
            duration, status as "status: AppointmentStatus", reason, notes,
            created_by, created_at, updated_at
        FROM appointments 
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(appointment)
}

pub async fn update_appointment_status(
    pool: &PgPool,
    id: Uuid,
    request: UpdateAppointmentStatusRequest,
) -> Result<Appointment> {
    let appointment = sqlx::query_as!(
        Appointment,
        r#"
        UPDATE appointments 
        SET status = $2, notes = COALESCE($3, notes), updated_at = NOW()
        WHERE id = $1
        RETURNING 
            id, patient_id, doctor_id, appointment_date, appointment_time,
            duration, status as "status: AppointmentStatus", reason, notes,
            created_by, created_at, updated_at
        "#,
        id,
        request.status as AppointmentStatus,
        request.notes
    )
    .fetch_one(pool)
    .await?;

    Ok(appointment)
}
