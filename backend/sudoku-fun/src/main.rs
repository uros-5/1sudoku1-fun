use axum::{http::HeaderValue, routing::get, Extension, Router};
use websockets::{handler::websocket_handler, state::WsState};

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::Mutex as Mutex2;
use tower_http::cors::CorsLayer;

mod database;
mod websockets;

use crate::database::Database;

#[tokio::main]
async fn main() {
    let db = Database::new().await;
    let cors_layer = cors();
    let db = Arc::new(db);
    let ws = Arc::new(WsState::default());
    let app = Router::new()
        .route("/ws/", get(websocket_handler))
        .layer(Extension(db))
        .layer(Extension(ws))
        .layer(cors_layer);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn cors() -> CorsLayer {
    let addr = "http://localhost:5173";
    let cors = CorsLayer::new();
    cors.allow_origin(addr.parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
}

pub fn arc2<T>(data: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(data))
}

pub fn arc3<T>(data: T) -> Arc<Mutex2<T>> {
    Arc::new(Mutex2::new(data))
}
