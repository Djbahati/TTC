use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::{info, error};

mod models;
mod handlers;
mod services;
mod websocket;
mod auth;
mod error;

use handlers::{auth as auth_handlers, appointments, dashboard, messages};
use websocket::websocket_handler;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub websocket_state: Arc<websocket::WebSocketState>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost/hospital_management".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Initialize WebSocket state
    let websocket_state = Arc::new(websocket::WebSocketState::new());

    let app_state = AppState {
        db: pool,
        websocket_state,
    };

    // Build our application with routes
    let app = Router::new()
        // Authentication routes
        .route("/api/auth/login", post(auth_handlers::login))
        .route("/api/auth/register", post(auth_handlers::register))
        .route("/api/auth/me", get(auth_handlers::me))
        
        // Appointment routes
        .route("/api/appointments", get(appointments::list_appointments))
        .route("/api/appointments", post(appointments::create_appointment))
        .route("/api/appointments/:id", get(appointments::get_appointment))
        .route("/api/appointments/:id/status", post(appointments::update_appointment_status))
        
        // Dashboard routes
        .route("/api/dashboard/stats", get(dashboard::get_dashboard_stats))
        .route("/api/dashboard/live", get(dashboard::get_live_data))
        
        // Message routes
        .route("/api/messages", get(messages::list_messages))
        .route("/api/messages", post(messages::send_message))
        .route("/api/messages/:id/read", post(messages::mark_as_read))
        
        // WebSocket endpoint
        .route("/ws", get(websocket_handler))
        
        // Health check
        .route("/health", get(health_check))
        
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("Hospital Management System running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    })))
}
