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
use tokio::sync::broadcast;

use crate::database::{session::UserSession, Database};

use super::{
    message_handler::{MessageHandler, MsgSender},
    messages::SendTo,
    state::WsState,
};

macro_rules! send_or_break {
    ($sender: expr, $msg: expr, $username: expr) => {
        if $sender
            .send(Message::Text($msg.msg.to_string()))
            .await
            .is_err()
        {
            break;
        }
    };
}

/// Pass all app data to websocket handler.
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    _user_agent: Option<TypedHeader<UserAgent>>,
    Extension(db): Extension<Arc<Database>>,
    Extension(wss): Extension<Arc<WsState>>,
    user: UserSession,
) -> impl IntoResponse {
    let headers = &user.headers();
    (
        headers.clone(),
        ws.on_upgrade(|socket| websocket(socket, db, wss, user)),
    )
}

/// Handler for websocket messages.
async fn websocket(stream: WebSocket, db: Arc<Database>, ws: Arc<WsState>, user: UserSession) {
    let (mut sender, mut receiver) = stream.split();
    let mut rx = ws.tx.subscribe();
    let username = String::from(&user.username);
    let socket_send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            match &msg.to {
                SendTo::Me => {
                    if &msg.username == &username {
                        send_or_break!(&mut sender, msg, &username);
                    }
                }
                SendTo::All => {
                    send_or_break!(&mut sender, msg, &username);
                }
                SendTo::Players(players) => {
                    if players.contains(&username) {
                        send_or_break!(&mut sender, msg, &username);
                    }
                }
            }
        }
    });

    let tx = ws.tx.clone();
    let (clock_tx, _) = broadcast::channel(100);

    let _ = tokio::spawn(async move {
        let msg_sender = MsgSender::new(&user, &tx);
        let handler = MessageHandler::new(&ws, &tx, &db, &clock_tx, msg_sender);
        handler.games_count(SendTo::Me);
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(value) = serde_json::from_str::<Value>(&text) {
                        let data_type = &value["t"];
                        match data_type {
                            serde_json::Value::String(t) => {
                                if t == "username" {
                                    handler.get_username();
                                } else if t == "games_count" {
                                    handler.games_count(SendTo::Me);
                                } else if t == "create_game" {
                                    handler.create_game(value).await;
                                } else if t == "accept_game" {
                                    handler.accept_game(value).await;
                                } else if t == "resign" {
                                    handler.resign(value).await;
                                } else if t == "make_move" {
                                    handler.make_move(value).await;
                                } else if t == "delete_one" {
                                    handler.make_move(value).await;
                                } else if t == "delete_all" {
                                    handler.make_move(value).await;
                                } else if t == "live_game" {
                                    handler.live_game(value);
                                } else if t == "live_game_line" {
                                    handler.live_game_line(value);
                                } else if t == "request_url" {
                                    handler.request_url();
                                } else if t == "game_url" {
                                    handler.game_url();
                                }
                            }
                            _ => (),
                        }
                    }
                }
                Message::Close(_c) => {
                    socket_send_task.abort();
                    break;
                }
                _ => (),
            }
        }
    });
}
