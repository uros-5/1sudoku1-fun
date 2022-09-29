use bson::{document, DateTime};
use mongodb::Collection;

use super::helpers::{random_session, random_username};
use super::mongo::Player;
use super::session::{MemcachedCli, UserSession};

pub async fn create_session(m: &MemcachedCli) -> String {
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
        let filter = bson::doc! {"_id": &username};
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

pub async fn create_for_cookie(m: &MemcachedCli, c: &Collection<Player>) -> UserSession {
    let session = create_session(m).await;
    let player = create_player(c).await;
    let session = m.clone().set(&session, &player._id).await;
    let _ = c.insert_one(player, None).await;
    session
}
