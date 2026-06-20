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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_request_deserializes_from_json() {
        let json = r#"{"email": "test@example.com", "password": "secret"}"#;
        let req: LoginRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.email, "test@example.com");
        assert_eq!(req.password, "secret");
    }

    #[test]
    fn user_info_serializes_to_json() {
        let info = UserInfo {
            id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            email: "user@example.com".to_string(),
            role: UserRole::Doctor,
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("user@example.com"));
        assert!(json.contains("Jane"));
        assert!(json.contains("Doctor"));
    }

    #[test]
    fn login_response_serializes_with_token_and_user() {
        let resp = LoginResponse {
            token: "abc123".to_string(),
            user: UserInfo {
                id: Uuid::new_v4(),
                email: "test@test.com".to_string(),
                role: UserRole::Admin,
                first_name: "Admin".to_string(),
                last_name: "User".to_string(),
            },
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("abc123"));
        assert!(json.contains("Admin"));
    }

    #[test]
    fn create_appointment_request_deserializes() {
        let json = r#"{
            "patient_id": "550e8400-e29b-41d4-a716-446655440000",
            "doctor_id": "660e8400-e29b-41d4-a716-446655440000",
            "appointment_date": "2025-06-15",
            "appointment_time": "14:30:00",
            "reason": "Checkup"
        }"#;
        let req: CreateAppointmentRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.reason, Some("Checkup".to_string()));
        assert_eq!(req.appointment_date.to_string(), "2025-06-15");
    }

    #[test]
    fn create_appointment_request_deserializes_without_optional_reason() {
        let json = r#"{
            "patient_id": "550e8400-e29b-41d4-a716-446655440000",
            "doctor_id": "660e8400-e29b-41d4-a716-446655440000",
            "appointment_date": "2025-06-15",
            "appointment_time": "14:30:00"
        }"#;
        let req: CreateAppointmentRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.reason, None);
    }

    #[test]
    fn update_appointment_status_request_deserializes() {
        let json = r#"{"status": "Completed", "notes": "All good"}"#;
        let req: UpdateAppointmentStatusRequest = serde_json::from_str(json).unwrap();
        assert!(matches!(req.status, AppointmentStatus::Completed));
        assert_eq!(req.notes, Some("All good".to_string()));
    }

    #[test]
    fn send_message_request_deserializes() {
        let json = r#"{
            "recipient_id": "550e8400-e29b-41d4-a716-446655440000",
            "content": "Hello",
            "is_urgent": true
        }"#;
        let req: SendMessageRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.content, "Hello");
        assert_eq!(req.is_urgent, Some(true));
    }

    #[test]
    fn send_message_request_deserializes_without_optional_urgent() {
        let json = r#"{
            "recipient_id": "550e8400-e29b-41d4-a716-446655440000",
            "content": "Hello"
        }"#;
        let req: SendMessageRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.is_urgent, None);
    }

    #[test]
    fn dashboard_stats_serializes() {
        let stats = DashboardStats {
            appointments_today: AppointmentStats {
                total: 10, confirmed: 5, in_progress: 2,
                completed: 2, cancelled: 1, no_shows: 0,
            },
            triage_queue: TriageStats {
                waiting: 3, urgent_cases: 1, average_wait_time: Some(15.5),
            },
            staff_status: StaffStats {
                online: 8, in_consultation: 3, on_break: 2,
            },
            inventory_alerts: InventoryStats {
                low_stock_items: 5, expiring_soon: 2,
            },
            billing_summary: BillingStats {
                revenue_today: rust_decimal::Decimal::new(50000, 2),
                unpaid_invoices: 3, failed_payments: 0,
            },
            messages: MessageStats {
                unread_messages: 7, urgent_messages: 2,
            },
        };
        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("appointments_today"));
        assert!(json.contains("triage_queue"));
        assert!(json.contains("staff_status"));
    }

    #[test]
    fn user_role_serializes_correctly() {
        let role = UserRole::Doctor;
        let json = serde_json::to_string(&role).unwrap();
        assert_eq!(json, "\"Doctor\"");
    }

    #[test]
    fn appointment_status_serializes_correctly() {
        let status = AppointmentStatus::CheckedIn;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"CheckedIn\"");
    }

    #[test]
    fn priority_level_serializes_correctly() {
        let priority = PriorityLevel::Critical;
        let json = serde_json::to_string(&priority).unwrap();
        assert_eq!(json, "\"Critical\"");
    }

    #[test]
    fn triage_stats_with_none_average_wait_time() {
        let stats = TriageStats {
            waiting: 0,
            urgent_cases: 0,
            average_wait_time: None,
        };
        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("null"));
    }
}
