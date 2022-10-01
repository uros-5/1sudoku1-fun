use serde::{Deserialize, Serialize};

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
    #[serde(default)]
    pub game_move: String,
}

#[derive(Clone)]
pub enum SendTo {
    Me,
    Players([String; 2]),
    All,
}
