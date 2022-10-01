use std::sync::Arc;

use serde_json::Value;
use tokio::sync::broadcast::Sender;

use crate::database::{mongo::SudokuGame, session::UserSession, Database};

use super::{
    messages::{CreatingGame, GameMove, SendTo},
    state::WsState,
};

#[derive(Clone)]
pub struct MessageHandler<'a> {
    pub ws: &'a Arc<WsState>,
    pub tx: &'a Sender<ClientMessage>,
    pub db: &'a Arc<Database>,
    pub msg_sender: MsgSender,
}

impl<'a> MessageHandler<'a> {
    pub fn new(
        ws: &'a Arc<WsState>,
        tx: &'a Sender<ClientMessage>,
        db: &'a Arc<Database>,
        msg_sender: MsgSender,
    ) -> Self {
        Self {
            ws,
            tx,
            db,
            msg_sender,
        }
    }

    pub fn games_count(&self) {
        let count = self.ws.games.games_count();
        let res = serde_json::json!({"t": "games_count", "cnt": count});
        self.msg_sender.send_msg(res, SendTo::Me);
    }

    pub async fn create_game(&self, value: Value) {
        if let Ok(v) = serde_json::from_value::<CreatingGame>(value) {
            let username = &self.username();
            if !self.ws.games.is_playing(username) {
                if let Some(id) = self
                    .ws
                    .requests
                    .add(username, v.minute, &self.db.mongo.games)
                    .await
                {
                    let value = serde_json::json!({"t": "live_game_created", "game_id": id});
                    self.msg_sender.send_msg(value, SendTo::Me);
                }
            }
        }
    }

    pub async fn accept_game(&self, value: Value) {
        if let Ok(v) = serde_json::from_value::<GameMove>(value) {
            if let Some(mut g) = self.ws.requests.remove(&v.game_id, &self.username()) {
                g.other = String::from(&self.username());
                let players = self.ws.games.add_game(&g);
                let value = serde_json::json!({"t": "live_game_accepted", "game_id": &v.game_id});
                let to = SendTo::Players(players);
                self.msg_sender.send_msg(value, to);
            }
        }
    }

    pub fn resign(&self, value: Value) {
        if let Ok(v) = serde_json::from_value::<GameMove>(value) {
            if let Some((p, i)) = self.ws.games.resign(&v.game_id, &self.username()) {
                let value = serde_json::json!({"t": "live_game_resigned", "index": i});
                let to = SendTo::Players(p);
                self.msg_sender.send_msg(value, to);
            }
        }
    }

    pub fn make_move(&self, value: Value) {
        if let Ok(v) = serde_json::from_value::<GameMove>(value) {
            self.ws
                .games
                .make_move(&v.game_id, &self.username(), &v.game_move);
        }
    }

    pub fn live_game(&self, value: Value) {
        if let Ok(g) = serde_json::from_value::<GameMove>(value) {
            if let Some(g) = self.ws.games.live_game(&g.game_id, &self.username()) {
                let value = serde_json::json!({"t": "live_game", "game": g});
                self.msg_sender.send_msg(value, SendTo::Me);
            }
        }
    }

    pub fn live_game_line(&self, value: Value) {
        if let Ok(g) = serde_json::from_value::<GameMove>(value) {
            if let Some(line) = self.ws.games.live_game_line(&g.game_id, &self.username()) {
                let value = serde_json::json!({"t": "live_game_line", "line": line});
                self.msg_sender.send_msg(value, SendTo::Me);
            }
        }
    }

    pub fn get_username(&self) {
        let value = serde_json::json!({"t": "username", "username": &self.username()});
        self.msg_sender.send_msg(value, SendTo::Me);
    }

    fn username(&self) -> String {
        String::from(&self.msg_sender.user.username)
    }
}

#[derive(Clone)]
pub struct ClientMessage {
    pub username: String,
    pub msg: Value,
    pub to: SendTo,
}

impl ClientMessage {
    pub fn new(session: &UserSession, msg: Value, to: SendTo) -> Self {
        Self {
            username: String::from(&session.username),
            msg,
            to,
        }
    }
}

#[derive(Clone)]
pub struct MsgSender {
    user: UserSession,
    tx: Sender<ClientMessage>,
}

impl MsgSender {
    pub fn new(user: &UserSession, tx: &Sender<ClientMessage>) -> Self {
        Self {
            user: user.clone(),
            tx: tx.clone(),
        }
    }

    pub fn send_msg(&self, value: Value, to: SendTo) {
        let cm = ClientMessage::new(&self.user, value, to);
        let _ = self.tx.send(cm);
    }
}
