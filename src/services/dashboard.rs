use sqlx::PgPool;
use crate::models::*;
use anyhow::Result;

pub async fn get_dashboard_statistics(pool: &PgPool) -> Result<DashboardStats> {
    // Get appointment statistics for today
    let appointment_stats = get_appointment_stats(pool).await?;
    
    // Get triage queue statistics
    let triage_stats = get_triage_stats(pool).await?;
    
    // Get staff status
    let staff_stats = get_staff_stats(pool).await?;
    
    // Get inventory alerts
    let inventory_stats = get_inventory_stats(pool).await?;
    
    // Get billing summary
    let billing_stats = get_billing_stats(pool).await?;
    
    // Get message statistics
    let message_stats = get_message_stats(pool).await?;

    Ok(DashboardStats {
        appointments_today: appointment_stats,
        triage_queue: triage_stats,
        staff_status: staff_stats,
        inventory_alerts: inventory_stats,
        billing_summary: billing_stats,
        messages: message_stats,
    })
}

async fn get_appointment_stats(pool: &PgPool) -> Result<AppointmentStats> {
    let today = chrono::Utc::now().date_naive();
    
    let total: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM appointments WHERE appointment_date = $1",
        today
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    let confirmed: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM appointments WHERE appointment_date = $1 AND status = 'confirmed'",
        today
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    let in_progress: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM appointments WHERE appointment_date = $1 AND status IN ('checked_in', 'in_consult')",
        today
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    let completed: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM appointments WHERE appointment_date = $1 AND status = 'completed'",
        today
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    let cancelled: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM appointments WHERE appointment_date = $1 AND status = 'cancelled'",
        today
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    let no_shows: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM appointments WHERE appointment_date = $1 AND status = 'no_show'",
        today
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    Ok(AppointmentStats {
        total,
        confirmed,
        in_progress,
        completed,
        cancelled,
        no_shows,
    })
}

async fn get_triage_stats(pool: &PgPool) -> Result<TriageStats> {
    let waiting: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM triage_queue WHERE status = 'waiting'"
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    let urgent_cases: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM triage_queue WHERE priority IN ('high', 'critical') AND status = 'waiting'"
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    let average_wait_time: Option<f64> = sqlx::query_scalar!(
        "SELECT AVG(estimated_wait_time) FROM triage_queue WHERE status = 'waiting'"
    )
    .fetch_one(pool)
    .await?;

    Ok(TriageStats {
        waiting,
        urgent_cases,
        average_wait_time,
    })
}

async fn get_staff_stats(pool: &PgPool) -> Result<StaffStats> {
    let online: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM staff s JOIN users u ON s.user_id = u.id WHERE s.is_on_duty = true AND u.is_active = true"
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    let in_consultation: i64 = sqlx::query_scalar!(
        "SELECT COUNT(DISTINCT d.id) FROM doctors d 
         JOIN appointments a ON d.id = a.doctor_id 
         WHERE a.status = 'in_consult' AND a.appointment_date = CURRENT_DATE"
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    Ok(StaffStats {
        online,
        in_consultation,
        on_break: 0, // This would need additional tracking
    })
}

async fn get_inventory_stats(pool: &PgPool) -> Result<InventoryStats> {
    let low_stock_items: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM inventory WHERE current_stock <= minimum_stock"
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    let expiring_soon: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM inventory WHERE expiry_date <= CURRENT_DATE + INTERVAL '30 days'"
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    Ok(InventoryStats {
        low_stock_items,
        expiring_soon,
    })
}

async fn get_billing_stats(pool: &PgPool) -> Result<BillingStats> {
    let today = chrono::Utc::now().date_naive();
    
    let revenue_today: Option<rust_decimal::Decimal> = sqlx::query_scalar!(
        "SELECT SUM(paid_amount) FROM invoices WHERE DATE(created_at) = $1",
        today
    )
    .fetch_one(pool)
    .await?;

    let unpaid_invoices: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM invoices WHERE status = 'pending'"
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    Ok(BillingStats {
        revenue_today: revenue_today.unwrap_or_else(|| rust_decimal::Decimal::new(0, 0)),
        unpaid_invoices,
        failed_payments: 0, // This would need payment gateway integration
    })
}

async fn get_message_stats(pool: &PgPool) -> Result<MessageStats> {
    let unread_messages: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM messages WHERE is_read = false"
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    let urgent_messages: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM messages WHERE is_urgent = true AND is_read = false"
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    Ok(MessageStats {
        unread_messages,
        urgent_messages,
    })
}

pub async fn get_live_dashboard_data(pool: &PgPool) -> Result<serde_json::Value> {
    let stats = get_dashboard_statistics(pool).await?;
    
    Ok(serde_json::json!({
        "timestamp": chrono::Utc::now(),
        "stats": stats,
        "alerts": {
            "critical_patients": stats.triage_queue.urgent_cases,
            "low_stock": stats.inventory_alerts.low_stock_items,
            "urgent_messages": stats.messages.urgent_messages
        }
    }))
}
