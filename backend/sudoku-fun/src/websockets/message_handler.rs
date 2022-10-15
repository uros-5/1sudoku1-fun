use std::sync::Arc;

use serde_json::Value;
use tokio::sync::broadcast::Sender;

use crate::{
    arc2,
    database::{
        mongo::SudokuGame,
        queries::{add_game, update_game},
        session::UserSession,
        Database,
    },
};

use super::{
    messages::{CreatingGame, GameMove, SendTo},
    state::WsState,
    time_control::{MsgClock, TimeCheck},
};

#[derive(Clone)]
pub struct MessageHandler<'a> {
    pub ws: &'a Arc<WsState>,
    pub tx: &'a Sender<ClientMessage>,
    pub clock_tx: &'a Sender<MsgClock>,
    pub db: &'a Arc<Database>,
    pub msg_sender: MsgSender,
}

impl<'a> MessageHandler<'a> {
    pub fn new(
        ws: &'a Arc<WsState>,
        tx: &'a Sender<ClientMessage>,
        db: &'a Arc<Database>,
        clock_tx: &'a Sender<MsgClock>,
        msg_sender: MsgSender,
    ) -> Self {
        Self {
            ws,
            tx,
            clock_tx,
            db,
            msg_sender,
        }
    }

    pub fn games_count(&self, to: SendTo) {
        let count = self.ws.games.games_count();
        let res = serde_json::json!({"t": "games_count", "cnt": count});
        self.msg_sender.send_msg(res, to);
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
                add_game(&self.db.mongo.games, SudokuGame::from(&g)).await;

                let mut clock_rv = self.clock_tx.subscribe();
                let clock_tx = self.clock_tx.clone();
                let id = String::from(&v.game_id);
                let ws = self.ws.clone();
                self.games_count(SendTo::All);
                let _clock_task = tokio::spawn({
                    let msg_sender = self.msg_sender.clone();
                    let id = String::from(&id);
                    let c = self.db.mongo.games.clone();
                    async move {
                        while let Ok(msg) = clock_rv.recv().await {
                            match &msg {
                                MsgClock::LostOnTime(t) => {
                                    if let Some(res) = ws.games.lost_on_time(&id) {
                                        if let Some(game) = res.game {
                                            t.lock().unwrap().finished();
                                            let value = serde_json::json!({"t": "game_finished", "score":res.score});
                                            msg_sender.send_msg(
                                                value,
                                                SendTo::Players(game.game.players.clone()),
                                            );
                                            update_game(&c, game).await;
                                            break;
                                        }
                                        continue;
                                    }
                                    t.lock().unwrap().finished();
                                    break;
                                }
                            }
                        }
                    }
                });

                let _clock_loop = tokio::spawn(async move {
                    let clock = arc2(TimeCheck::new());
                    loop {
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        let t = clock.lock().unwrap();
                        if t.is_finished() {
                            break;
                        }
                        if let Ok(_) = clock_tx.send(MsgClock::LostOnTime(clock.clone())) {}
                    }
                });
            }
        }
    }

    pub async fn resign(&self, value: Value) {
        if let Ok(game_move) = serde_json::from_value::<GameMove>(value) {
            if let Some(game_result) = self.ws.games.resign(&game_move.game_id, &self.username()) {
                let value = serde_json::json!({"t": "live_game_resigned", "player": game_result.player, "score": game_result.score});
                let to = SendTo::Players(game_result.players);
                self.msg_sender.send_msg(value, to);
                let c = self.db.mongo.games.clone();
                update_game(&c, game_result.game.unwrap()).await;
            }
        }
    }

    pub async fn make_move(&self, value: Value) {
        if let Ok(v) = serde_json::from_value::<GameMove>(value) {
            if let Some(game_result) =
                self.ws
                    .games
                    .make_move(&v.game_id, &self.username(), &v.game_move)
            {
                let value = serde_json::json!({"t":"live_game_winner", "player": game_result.player, "score": game_result.score});
                let to = SendTo::Players(game_result.players);
                self.msg_sender.send_msg(value, to);
                let c = self.db.mongo.games.clone();
                update_game(&c, game_result.game.unwrap()).await;
            }
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

    pub fn request_url(&self) {
        if let Some(url) = self.ws.requests.get_request_url(&self.username()) {
            let value = serde_json::json!({"t": "request_url", "url": url});
            self.msg_sender.send_msg(value, SendTo::Me);
        }
    }

    pub fn game_url(&self) {
        if let Some(url) = self.ws.games.get_game_url(&self.username()) {
            let value = serde_json::json!({"t": "game_url", "url": url});
            self.msg_sender.send_msg(value, SendTo::Me);
        }
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
