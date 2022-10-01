use bson::{doc, DateTime};
use mongodb::Collection;

use super::helpers::{random_session, random_username};
use super::mongo::{Player, SudokuGame};
use super::session::{RedisCli, UserSession};

pub async fn create_session(m: &RedisCli) -> String {
    let mut m = m.clone();
    loop {
        let session = random_session();
        if !m.get(&session).await.is_some() {
            return session;
        }
    }
}

pub async fn create_player(m: &Collection<Player>) -> Player {
    loop {
        let username = random_username();
        let filter = doc! {"_id": &username};
        if let Ok(r) = m.find_one(filter, None).await {
            if !r.is_some() {
                return Player {
                    _id: username,
                    password: String::from(""),
                    date_created: DateTime::now(),
                };
            }
        }
    }
}

pub async fn create_for_cookie(m: &RedisCli, c: &Collection<Player>) -> UserSession {
    let session = create_session(m).await;
    let player = create_player(c).await;
    let session = m.clone().set(&session, &player._id).await;
    let _ = c.insert_one(player, None).await;
    session
}

pub async fn game_exist(c: &Collection<SudokuGame>, id: String) -> bool {
    if let Ok(r) = c.find_one(doc! {"_id": id}, None).await {
        if let Some(r) = r {
            return true;
        }
    }
    false
}
