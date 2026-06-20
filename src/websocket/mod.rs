use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, error, warn};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct WebSocketState {
    pub connections: Arc<RwLock<HashMap<Uuid, broadcast::Sender<WebSocketMessage>>>>,
    pub global_sender: broadcast::Sender<WebSocketMessage>,
}

impl WebSocketState {
    pub fn new() -> Self {
        let (global_sender, _) = broadcast::channel(1000);

        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            global_sender,
        }
    }

    pub async fn add_connection(&self, user_id: Uuid, sender: broadcast::Sender<WebSocketMessage>) {
        let mut connections = self.connections.write().await;
        connections.insert(user_id, sender);
        debug!("WebSocket connection added for user {user_id}");
    }

    pub async fn remove_connection(&self, user_id: &Uuid) {
        let mut connections = self.connections.write().await;
        connections.remove(user_id);
        debug!("WebSocket connection removed for user {user_id}");
    }

    pub async fn broadcast_to_all(&self, message: WebSocketMessage) {
        if let Err(e) = self.global_sender.send(message) {
            debug!("No active WebSocket subscribers for broadcast: {e}");
        }
    }

    pub async fn send_to_user(&self, user_id: &Uuid, message: WebSocketMessage) {
        let connections = self.connections.read().await;
        if let Some(sender) = connections.get(user_id) {
            if let Err(e) = sender.send(message) {
                warn!("Failed to send WebSocket message to user {user_id}: {e}");
            }
        } else {
            debug!("No active WebSocket connection for user {user_id}");
        }
    }

    pub async fn broadcast_dashboard_update(&self, data: serde_json::Value) {
        let message = WebSocketMessage {
            message_type: "dashboard_update".to_string(),
            data,
            timestamp: chrono::Utc::now(),
        };
        self.broadcast_to_all(message).await;
    }

    pub async fn broadcast_appointment_update(&self, appointment_data: serde_json::Value) {
        let message = WebSocketMessage {
            message_type: "appointment_update".to_string(),
            data: appointment_data,
            timestamp: chrono::Utc::now(),
        };
        self.broadcast_to_all(message).await;
    }

    pub async fn broadcast_triage_update(&self, triage_data: serde_json::Value) {
        let message = WebSocketMessage {
            message_type: "triage_update".to_string(),
            data: triage_data,
            timestamp: chrono::Utc::now(),
        };
        self.broadcast_to_all(message).await;
    }

    pub async fn send_message_notification(&self, recipient_id: &Uuid, message_data: serde_json::Value) {
        let message = WebSocketMessage {
            message_type: "new_message".to_string(),
            data: message_data,
            timestamp: chrono::Utc::now(),
        };
        self.send_to_user(recipient_id, message).await;
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

async fn websocket_connection(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let user_id = Uuid::new_v4(); // In real implementation, extract from JWT token

    let (tx, mut rx) = broadcast::channel(100);

    // Add connection to state
    state.websocket_state.add_connection(user_id, tx.clone()).await;

    // Subscribe to global broadcasts
    let mut global_rx = state.websocket_state.global_sender.subscribe();

    // Spawn task to handle outgoing messages
    let tx_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                // Handle user-specific messages
                msg = rx.recv() => {
                    match msg {
                        Ok(message) => {
                            let json = match serde_json::to_string(&message) {
                                Ok(j) => j,
                                Err(e) => {
                                    error!("Failed to serialize WebSocket message: {e}");
                                    continue;
                                }
                            };
                            if sender.send(Message::Text(json)).await.is_err() {
                                break;
                            }
                        }
                        Err(broadcast::error::RecvError::Lagged(n)) => {
                            warn!("WebSocket receiver lagged, skipped {n} messages");
                            continue;
                        }
                        Err(broadcast::error::RecvError::Closed) => break,
                    }
                }
                // Handle global broadcasts
                msg = global_rx.recv() => {
                    match msg {
                        Ok(message) => {
                            let json = match serde_json::to_string(&message) {
                                Ok(j) => j,
                                Err(e) => {
                                    error!("Failed to serialize global WebSocket message: {e}");
                                    continue;
                                }
                            };
                            if sender.send(Message::Text(json)).await.is_err() {
                                break;
                            }
                        }
                        Err(broadcast::error::RecvError::Lagged(n)) => {
                            warn!("Global WebSocket receiver lagged, skipped {n} messages");
                            continue;
                        }
                        Err(broadcast::error::RecvError::Closed) => break,
                    }
                }
            }
        }
    });

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Handle incoming WebSocket messages
                match serde_json::from_str::<WebSocketMessage>(&text) {
                    Ok(ws_msg) => {
                        match ws_msg.message_type.as_str() {
                            "ping" => {
                                let pong = WebSocketMessage {
                                    message_type: "pong".to_string(),
                                    data: serde_json::json!({}),
                                    timestamp: chrono::Utc::now(),
                                };
                                if let Err(e) = tx.send(pong) {
                                    warn!("Failed to send pong response: {e}");
                                }
                            }
                            "subscribe_dashboard" => {
                                match crate::services::dashboard::get_dashboard_statistics(&state.db).await {
                                    Ok(stats) => {
                                        match serde_json::to_value(&stats) {
                                            Ok(data) => {
                                                let message = WebSocketMessage {
                                                    message_type: "dashboard_data".to_string(),
                                                    data,
                                                    timestamp: chrono::Utc::now(),
                                                };
                                                if let Err(e) = tx.send(message) {
                                                    warn!("Failed to send dashboard data: {e}");
                                                }
                                            }
                                            Err(e) => {
                                                error!("Failed to serialize dashboard stats: {e}");
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to fetch dashboard stats for WebSocket subscription: {e}");
                                    }
                                }
                            }
                            other => {
                                debug!("Received unknown WebSocket message type: {other}");
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse incoming WebSocket message: {e}");
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                warn!("WebSocket receive error for user {user_id}: {e}");
                break;
            }
            _ => {}
        }
    }

    // Clean up
    tx_task.abort();
    state.websocket_state.remove_connection(&user_id).await;
}
