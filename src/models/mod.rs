use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate, NaiveTime};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Patient,
    Doctor,
    Nurse,
    Receptionist,
    LabTechnician,
    Pharmacist,
    Accountant,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "appointment_status", rename_all = "lowercase")]
pub enum AppointmentStatus {
    Requested,
    Confirmed,
    CheckedIn,
    InConsult,
    Completed,
    Cancelled,
    NoShow,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "priority_level", rename_all = "lowercase")]
pub enum PriorityLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub is_active: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Patient {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date_of_birth: NaiveDate,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub blood_type: Option<String>,
    pub allergies: Option<Vec<String>>,
    pub chronic_conditions: Option<Vec<String>>,
    pub insurance_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Doctor {
    pub id: Uuid,
    pub user_id: Uuid,
    pub specialization: String,
    pub license_number: String,
    pub department: Option<String>,
    pub consultation_fee: Option<rust_decimal::Decimal>,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Appointment {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub doctor_id: Uuid,
    pub appointment_date: NaiveDate,
    pub appointment_time: NaiveTime,
    pub duration: Option<i32>,
    pub status: AppointmentStatus,
    pub reason: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TriageQueue {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub priority: PriorityLevel,
    pub chief_complaint: String,
    pub vital_signs: Option<serde_json::Value>,
    pub assigned_doctor_id: Option<Uuid>,
    pub estimated_wait_time: Option<i32>,
    pub queue_position: Option<i32>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub message_type: String,
    pub content: String,
    pub is_read: bool,
    pub is_urgent: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub appointments_today: AppointmentStats,
    pub triage_queue: TriageStats,
    pub staff_status: StaffStats,
    pub inventory_alerts: InventoryStats,
    pub billing_summary: BillingStats,
    pub messages: MessageStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentStats {
    pub total: i64,
    pub confirmed: i64,
    pub in_progress: i64,
    pub completed: i64,
    pub cancelled: i64,
    pub no_shows: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageStats {
    pub waiting: i64,
    pub urgent_cases: i64,
    pub average_wait_time: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffStats {
    pub online: i64,
    pub in_consultation: i64,
    pub on_break: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryStats {
    pub low_stock_items: i64,
    pub expiring_soon: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingStats {
    pub revenue_today: rust_decimal::Decimal,
    pub unpaid_invoices: i64,
    pub failed_payments: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStats {
    pub unread_messages: i64,
    pub urgent_messages: i64,
}

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAppointmentRequest {
    pub patient_id: Uuid,
    pub doctor_id: Uuid,
    pub appointment_date: NaiveDate,
    pub appointment_time: NaiveTime,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAppointmentStatusRequest {
    pub status: AppointmentStatus,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub recipient_id: Uuid,
    pub content: String,
    pub is_urgent: Option<bool>,
}
