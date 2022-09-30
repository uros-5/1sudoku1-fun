use axum::{
    extract::ws::{Message, WebSocket},
    extract::WebSocketUpgrade,
    headers::UserAgent,
    response::IntoResponse,
    Extension, TypedHeader,
};
use futures::{SinkExt, StreamExt};
use serde_json::Value;
use std::sync::Arc;

use crate::database::{session::UserSession, Database};

use super::messages::LiveCount;

/// Pass all app data to websocket handler.
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    _user_agent: Option<TypedHeader<UserAgent>>,
    Extension(db): Extension<Arc<Database>>,
    user: UserSession,
) -> impl IntoResponse {
    let headers = &user.headers();
    (
        headers.clone(),
        ws.on_upgrade(|socket| websocket(socket, db, user)),
    )
}

/// Handler for websocket messages.
async fn websocket(stream: WebSocket, db: Arc<Database>, user: UserSession) {
    let (mut sender, mut receiver) = stream.split();
    let mut socket_recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(value) = serde_json::from_str::<Value>(&text) {
                        let data_type = &value["t"];
                        match data_type {
                            serde_json::Value::String(t) => {
                                if t == "players_online" {
                                    
                                } else if t == "active_games" {
                                } else if t == "create_game" {
                                } else if t == "accept_game" {
                                } else if t == "resign" {
                                } else if t == "make_move" {
                                } else if t == "delete_one" {
                                } else if t == "delete_all" {
                                } else if t == "live_game" {
                                }
                            }
                            _ => (),
                        }
                    }
                }
                Message::Close(_c) => {
                    break;
                }
                _ => (),
            }
        }
    });
}
