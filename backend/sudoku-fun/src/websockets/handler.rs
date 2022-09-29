use axum::{
    extract::ws::{Message, WebSocket},
    extract::WebSocketUpgrade,
    headers::UserAgent,
    response::IntoResponse,
    Extension, TypedHeader,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;

use crate::database::{session::UserSession, Database};

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
                    println!("{}", text);
                }
                Message::Close(_c) => {
                    break;
                }
                _ => (),
            }
        }
    });
}
