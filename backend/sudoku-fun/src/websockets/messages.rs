use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LiveCount {
    pub t: String,
    pub cnt: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CreatingGame {
    pub t: String,
    pub minute: u8,
}

#[derive(Serialize, Deserialize)]
pub struct GameMove {
    pub t: String,
    pub game_id: String,
    pub game_move: String
}

pub enum PlayerMsg {
    ForMe,
    ForPlayers([String; 2]),
    ForAll
}