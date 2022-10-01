use tokio::sync::broadcast;

use super::{games::SudokuGames, message_handler::ClientMessage, requests::GameRequests};

pub struct WsState {
    pub requests: GameRequests,
    pub games: SudokuGames,
    pub tx: broadcast::Sender<ClientMessage>,
}

impl Default for WsState {
    fn default() -> Self {
        let requests = GameRequests::default();
        let games = SudokuGames::new();
        let tx = broadcast::channel(100);
        Self {
            requests,
            games,
            tx: tx.0,
        }
    }
}
