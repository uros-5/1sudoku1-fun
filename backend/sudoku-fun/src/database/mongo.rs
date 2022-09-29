use bson::DateTime;
use mongodb::{options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};

/// Player struct.
#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub _id: String,
    pub date_created: DateTime,
    pub password: String,
}

///All games by Players
#[derive(Serialize, Deserialize, Clone)]
pub struct SudokuGame {
    pub _id: String,
    pub players: [String; 2],
    pub clocks: [u16; 2],
    pub score: [u8; 2],
    pub status: u8,
    pub minutes: u8,
}

#[derive(Clone)]
pub struct Mongo {
    pub players: Collection<Player>,
    pub games: Collection<SudokuGame>,
}

impl Mongo {
    pub async fn new() -> Self {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017")
            .await
            .expect("No client available");
        client_options.app_name = Some("sudokuFun".to_string());
        let client = Client::with_options(client_options).expect("client not found");
        let db = client.database("sudokuFun");
        let players = db.collection("players");
        let games = db.collection("games");
        Self { players, games }
    }
}
