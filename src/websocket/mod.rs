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
    }

    pub async fn remove_connection(&self, user_id: &Uuid) {
        let mut connections = self.connections.write().await;
        connections.remove(user_id);
    }

    pub async fn broadcast_to_all(&self, message: WebSocketMessage) {
        let _ = self.global_sender.send(message);
    }

    pub async fn send_to_user(&self, user_id: &Uuid, message: WebSocketMessage) {
        let connections = self.connections.read().await;
        if let Some(sender) = connections.get(user_id) {
            let _ = sender.send(message);
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
                            let json = serde_json::to_string(&message).unwrap();
                            if sender.send(Message::Text(json)).await.is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
                // Handle global broadcasts
                msg = global_rx.recv() => {
                    match msg {
                        Ok(message) => {
                            let json = serde_json::to_string(&message).unwrap();
                            if sender.send(Message::Text(json)).await.is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
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
                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage>(&text) {
                    match ws_msg.message_type.as_str() {
                        "ping" => {
                            let pong = WebSocketMessage {
                                message_type: "pong".to_string(),
                                data: serde_json::json!({}),
                                timestamp: chrono::Utc::now(),
                            };
                            let _ = tx.send(pong);
                        }
                        "subscribe_dashboard" => {
                            // Send current dashboard data
                            if let Ok(stats) = crate::services::dashboard::get_dashboard_statistics(&state.db).await {
                                let message = WebSocketMessage {
                                    message_type: "dashboard_data".to_string(),
                                    data: serde_json::to_value(stats).unwrap(),
                                    timestamp: chrono::Utc::now(),
                                };
                                let _ = tx.send(message);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            _ => {}
        }
    }

    // Clean up
    tx_task.abort();
    state.websocket_state.remove_connection(&user_id).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn websocket_state_new_creates_empty_connections() {
        let state = WebSocketState::new();
        let connections = state.connections.read().await;
        assert!(connections.is_empty());
    }

    #[tokio::test]
    async fn add_and_remove_connection() {
        let state = WebSocketState::new();
        let user_id = Uuid::new_v4();
        let (tx, _rx) = broadcast::channel(10);

        state.add_connection(user_id, tx).await;
        {
            let connections = state.connections.read().await;
            assert_eq!(connections.len(), 1);
            assert!(connections.contains_key(&user_id));
        }

        state.remove_connection(&user_id).await;
        {
            let connections = state.connections.read().await;
            assert!(connections.is_empty());
        }
    }

    #[tokio::test]
    async fn remove_nonexistent_connection_is_noop() {
        let state = WebSocketState::new();
        let fake_id = Uuid::new_v4();
        state.remove_connection(&fake_id).await;
        let connections = state.connections.read().await;
        assert!(connections.is_empty());
    }

    #[tokio::test]
    async fn broadcast_to_all_delivers_message() {
        let state = WebSocketState::new();
        let mut rx = state.global_sender.subscribe();

        let msg = WebSocketMessage {
            message_type: "test".to_string(),
            data: serde_json::json!({"key": "value"}),
            timestamp: chrono::Utc::now(),
        };

        state.broadcast_to_all(msg.clone()).await;
        let received = rx.recv().await.unwrap();
        assert_eq!(received.message_type, "test");
    }

    #[tokio::test]
    async fn send_to_user_delivers_to_correct_user() {
        let state = WebSocketState::new();
        let user_id = Uuid::new_v4();
        let (tx, mut rx) = broadcast::channel(10);

        state.add_connection(user_id, tx).await;

        let msg = WebSocketMessage {
            message_type: "direct".to_string(),
            data: serde_json::json!({}),
            timestamp: chrono::Utc::now(),
        };

        state.send_to_user(&user_id, msg).await;
        let received = rx.recv().await.unwrap();
        assert_eq!(received.message_type, "direct");
    }

    #[tokio::test]
    async fn send_to_nonexistent_user_does_not_panic() {
        let state = WebSocketState::new();
        let msg = WebSocketMessage {
            message_type: "test".to_string(),
            data: serde_json::json!({}),
            timestamp: chrono::Utc::now(),
        };
        state.send_to_user(&Uuid::new_v4(), msg).await;
    }

    #[tokio::test]
    async fn broadcast_dashboard_update_uses_correct_type() {
        let state = WebSocketState::new();
        let mut rx = state.global_sender.subscribe();

        state.broadcast_dashboard_update(serde_json::json!({"stats": 42})).await;
        let received = rx.recv().await.unwrap();
        assert_eq!(received.message_type, "dashboard_update");
    }

    #[tokio::test]
    async fn broadcast_appointment_update_uses_correct_type() {
        let state = WebSocketState::new();
        let mut rx = state.global_sender.subscribe();

        state.broadcast_appointment_update(serde_json::json!({"id": 1})).await;
        let received = rx.recv().await.unwrap();
        assert_eq!(received.message_type, "appointment_update");
    }

    #[tokio::test]
    async fn broadcast_triage_update_uses_correct_type() {
        let state = WebSocketState::new();
        let mut rx = state.global_sender.subscribe();

        state.broadcast_triage_update(serde_json::json!({"priority": "high"})).await;
        let received = rx.recv().await.unwrap();
        assert_eq!(received.message_type, "triage_update");
    }

    #[tokio::test]
    async fn send_message_notification_uses_correct_type() {
        let state = WebSocketState::new();
        let user_id = Uuid::new_v4();
        let (tx, mut rx) = broadcast::channel(10);
        state.add_connection(user_id, tx).await;

        state.send_message_notification(&user_id, serde_json::json!({"msg": "hello"})).await;
        let received = rx.recv().await.unwrap();
        assert_eq!(received.message_type, "new_message");
    }

    #[test]
    fn websocket_message_serializes_correctly() {
        let msg = WebSocketMessage {
            message_type: "test".to_string(),
            data: serde_json::json!({"key": "value"}),
            timestamp: chrono::Utc::now(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"message_type\":\"test\""));
        assert!(json.contains("\"key\":\"value\""));
    }

    #[test]
    fn websocket_message_deserializes_correctly() {
        let json = r#"{"message_type":"ping","data":{},"timestamp":"2025-01-01T00:00:00Z"}"#;
        let msg: WebSocketMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.message_type, "ping");
    }

    #[tokio::test]
    async fn multiple_connections_tracked_independently() {
        let state = WebSocketState::new();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let (tx1, _rx1) = broadcast::channel(10);
        let (tx2, _rx2) = broadcast::channel(10);

        state.add_connection(id1, tx1).await;
        state.add_connection(id2, tx2).await;

        {
            let connections = state.connections.read().await;
            assert_eq!(connections.len(), 2);
        }

        state.remove_connection(&id1).await;
        {
            let connections = state.connections.read().await;
            assert_eq!(connections.len(), 1);
            assert!(connections.contains_key(&id2));
        }
    }
}
