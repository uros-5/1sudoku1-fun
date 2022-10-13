use serde::{Deserialize, Serialize};

use crate::database::mongo::SudokuGame;

#[derive(Serialize, Deserialize)]
pub struct LiveCount {
    pub t: String,
    pub cnt: u32,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Clone)]
pub struct GameResult {
    pub players: [String; 2],
    pub score: [u8; 2],
    pub player: usize,
    pub game: Option<SudokuGame>,
}

impl GameResult {
    pub fn new(
        players: [String; 2],
        score: [u8; 2],
        player: usize,
        game: Option<SudokuGame>,
    ) -> Self {
        Self {
            players,
            score,
            player,
            game,
        }
    }

    pub fn empty() -> Self {
        Self {
            players: [String::from(""), String::from("")],
            score: [0, 0],
            player: 0,
            game: None,
        }
    }
}
