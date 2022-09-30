use bson::DateTime;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::Range,
    sync::{Arc, Mutex},
};

use super::{messages::GameMove, time_control::TimeControl};
pub struct SudokuGames {}

#[derive(Serialize, Deserialize, Clone)]
pub struct SudokuGen {
    pub clock: TimeControl,
    pub score: [u8; 2],
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub current: Arc<Mutex<[String; 2]>>,
    pub min: u8,
    pub date_created: DateTime,
    pub started_with: String,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub solution: String,
    pub players: [String; 2],
}

impl SudokuGen {
    pub fn get_current(&self, user: &String) -> Option<String> {
        if let Some(index) = self.player_index(user) {
            let current = self.current.lock().unwrap();
            let c = String::from(&current[index]);
            drop(current);
            return Some(c);
        }
        None
    }

    pub fn make_move(&self, user: &String, m: &String) {
        if let Ok(msg) = serde_json::from_str::<GameMove>(&m) {
            if let Some(_) = self.clock.current_duration() {
                let m = SudokuGameMove::from(m);
                self.make_sudoku_move(user, m);
            }
        }
    }

    pub fn make_sudoku_move(&self, user: &String, m: SudokuGameMove) {
        if let Some(index) = self.player_index(user) {
            let mut current = self.current.lock().unwrap();
            if let Some(s) = current.get_mut(index) {
                match m {
                    SudokuGameMove::NormalMove { position, number } => {
                        let bytes = String::from(&s.clone());
                        let mut bytes = bytes.into_bytes();
                        bytes[position as usize] = number;
                        if let Ok(bytes) = String::from_utf8(bytes) {
                            *s = bytes;
                        }
                    }
                    SudokuGameMove::DeleteMove { position } => {
                        let bytes = String::from(&s.clone());
                        let mut bytes = bytes.into_bytes();
                        bytes[position as usize] = '.' as u8;
                        if let Ok(bytes) = String::from_utf8(bytes) {
                            *s = bytes;
                        }
                    }
                    SudokuGameMove::DeleteAll => {
                        *s = String::from(&self.started_with);
                    }
                }
            }
        }
    }

    fn player_index(&self, user: &String) -> Option<usize> {
        if let Some(index) = self.players.iter().position(|x| x == user) {
            return Some(index);
        }
        None
    }
}

const FIELDS: Range<u8> = Range { start: 0, end: 81 };
const FIELD_VALUES: Range<u8> = Range { start: 0, end: 9 };
pub enum SudokuGameMove {
    NormalMove { position: u8, number: u8 },
    DeleteMove { position: u8 },
    DeleteAll,
}

impl From<&String> for SudokuGameMove {
    fn from(s: &String) -> Self {
        let mut s2 = s.split("_");
        if let Some(pos) = s2.next() {
            if let Ok(position) = pos.parse::<u8>() {
                if FIELDS.contains(&position) {
                    if let Some(value) = s2.next() {
                        if let Ok(number) = value.parse::<u8>() {
                            if FIELD_VALUES.contains(&number) {
                                return Self::NormalMove { position, number };
                            }
                        } else if value == "x" {
                            return Self::DeleteMove { position };
                        }
                    }
                }
            }
        }
        Self::DeleteAll
    }
}
