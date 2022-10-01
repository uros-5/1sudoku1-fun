use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    sync::{Arc, Mutex},
};

use mongodb::{options::Collation, Collection};

use crate::{
    arc2,
    database::{helpers::random_game, mongo::SudokuGame, queries::game_exist},
};

const MINUTES: [u8; 3] = [1, 2, 3];

pub struct GameRequest {
    pub id: String,
    pub minute: u8,
    pub caller: String,
    pub other: String
}
pub struct GameRequests {
    requests: Arc<Mutex<HashMap<String, GameRequest>>>,
    players: Arc<Mutex<HashSet<String>>>
}

impl Default for GameRequests {
    fn default() -> Self {
        Self {
            requests: arc2(HashMap::new()),
            players: arc2(HashSet::new())
        }
    }
}

impl GameRequests {

    pub fn user_check(&self, user: &String) -> bool {
        if let Some(_) = self.players.lock().unwrap().get(user) {
            return true;
        }
        false
    }

    pub fn id_check(&self, id: &String) -> bool {
        let games = self.requests.lock().unwrap();
        if let Some(_) = games.get(id) {
            return true;
        }
        false
    }

    pub async fn add(&self, username: &String, minute: u8, c: &Collection<SudokuGame>) -> Option<String> {
        if !self.user_check(&username) {
            loop {
                let game_id = random_game();
                if !self.id_check(&game_id) {
                    let id = String::from(&game_id);
                    if !game_exist(c, String::from(&id)).await {
                        if MINUTES.contains(&minute) {
                            let mut requests = self.requests.lock().unwrap();
                            let r_id = String::from(&game_id);
                            requests.insert(
                                game_id,
                                GameRequest {
                                    id,
                                    minute,
                                    caller: String::from(username),
                                    other: String::from("")
                                },
                            );
                            let mut players = self.players.lock().unwrap();
                            players.insert(String::from(username));
                            return Some(r_id);
                        }
                        return None;
                    }
                }
            }
        }
        None
    }

    pub fn remove(&self, id: &String, other: &String) -> Option<GameRequest> {
        let mut rqs = self.requests.lock().unwrap();
        if let Some(r) = rqs.get(id) {
            if &r.caller != other {
                let mut ps = self.players.lock().unwrap();
                ps.remove(&r.caller);
                return rqs.remove(id);
            }
        }
        None
    }

    pub fn waiting_count(&self) -> usize {
        let p = self.players.lock().unwrap();
        p.len()
    }

}
