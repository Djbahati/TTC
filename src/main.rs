use axum::{
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    middleware,
    response::Json,
    routing::{get, post},
    Router,
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, AllowOrigin};
use tracing::{info, error};

mod models;
mod handlers;
mod services;
mod websocket;
mod auth;
mod error;

use handlers::{auth as auth_handlers, appointments, dashboard, messages};
use websocket::websocket_handler;
use auth::auth_middleware;

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

    // Database connection - require DATABASE_URL to be set explicitly
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

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

    // Configure CORS with explicit allowed origins
    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:3001".to_string());
    let origins: Vec<HeaderValue> = allowed_origins
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .allow_credentials(true);

    // Protected routes (require authentication)
    let protected_routes = Router::new()
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
        // Auth info route
        .route("/api/auth/me", get(auth_handlers::me))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/api/auth/login", post(auth_handlers::login))
        .route("/api/auth/register", post(auth_handlers::register))
        .route("/ws", get(websocket_handler))
        .route("/health", get(health_check));

    // Combine all routes
    let app = Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .layer(cors)
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
