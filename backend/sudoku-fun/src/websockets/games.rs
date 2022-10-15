use bson::DateTime;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::Range,
    sync::{Arc, Mutex},
};
use sudoku::Sudoku;

use crate::{arc2, database::mongo::SudokuGame};

use super::{messages::GameResult, requests::GameRequest, time_control::TimeControl};

const TEMP_CHARS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub struct SudokuGames {
    pub games: Arc<Mutex<HashMap<String, SudokuGame>>>,
}

impl SudokuGames {
    pub fn new() -> Self {
        let games = arc2(HashMap::new());
        Self { games }
    }

    pub fn is_playing(&self, username: &String) -> bool {
        let games = self.games.lock().unwrap();
        for game in games.iter() {
            if game.1.game.player_index(username).is_some() {
                return true;
            }
        }
        false
    }

    pub fn add_game(&self, g: &GameRequest) -> [String; 2] {
        let mut games = self.games.lock().unwrap();
        let sudoku_game = SudokuGame::from(g);
        let players = sudoku_game.game.players.clone();
        games.insert(String::from(&g.id), sudoku_game);
        players
    }

    pub fn games_count(&self) -> usize {
        let games = self.games.lock().unwrap();
        games.len()
    }

    pub fn resign(&self, id: &String, username: &String) -> Option<GameResult> {
        let mut games = self.games.lock().unwrap();
        if let Some(g) = games.get_mut(id) {
            if let Some(mut r) = g.game.resign(username) {
                let g = games.remove(id);
                r.update_game(g);
                return Some(r);
            }
        }
        None
    }

    pub fn make_move(&self, id: &String, user: &String, game_move: &String) -> Option<GameResult> {
        let mut games = self.games.lock().unwrap();
        if let Some(g) = games.get_mut(id) {
            g.game.make_move(user, game_move);
            if let Some(finished) = g.game.finished(user) {
                return Some(finished);
            }
        }
        None
    }

    pub fn live_game(&self, id: &String, user: &String) -> Option<SudokuGame> {
        let games = self.games.lock().unwrap();
        if let Some(g) = games.get(id) {
            if let Some(_) = g.game.player_index(user) {
                return Some(g.clone());
            }
        }
        None
    }

    pub fn live_game_line(&self, id: &String, user: &String) -> Option<String> {
        let games = self.games.lock().unwrap();
        if let Some(g) = games.get(id) {
            return g.game.get_current(user);
        }
        None
    }

    pub fn get_game_url(&self, username: &String) -> Option<String> {
        let games = self.games.lock().unwrap();
        for i in games.iter() {
            if i.1.game.is_playing(username) {
                return Some(String::from(i.0));
            }
        }
        None
    }

    pub fn lost_on_time(&self, id: &String) -> Option<GameResult> {
        let mut games = self.games.lock().unwrap();
        if let Some(g) = games.get_mut(id) {
            let result = g.game.lost_on_time();
            if result.0 {
                let sudoku_game = games.remove(id);
                drop(games);
                let result = GameResult::new(result.2, result.1, 0, sudoku_game);
                return Some(result);
            }
            return Some(GameResult::empty());
        }
        None
    }
}

impl From<&GameRequest> for SudokuGame {
    fn from(g: &GameRequest) -> Self {
        Self {
            _id: String::from(&g.id),
            game: SudokuGen::new(g),
        }
    }
}

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
    pub started_with_bytes: Vec<u8>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub solution: String,
    pub players: [String; 2],
    pub result: [u8; 2],
    pub status: u8,
}

impl SudokuGen {
    pub fn new(g: &GameRequest) -> Self {
        let sudoku = Sudoku::generate();
        let started_with = sudoku.to_str_line().to_string();
        let started_with = String::from(&started_with);
        let started_with_bytes = String::from(&started_with).into_bytes();
        let solution = sudoku.solution().unwrap().to_string();
        let clock = TimeControl::new(g.minute);
        let score = [0, 0];
        let current = Arc::new(Mutex::new([
            String::from(&started_with),
            String::from(&started_with),
        ]));
        let min = g.minute;
        let date_created = DateTime::now();
        let players = [String::from(&g.caller), String::from(&g.other)];
        Self {
            clock,
            score,
            current,
            min,
            date_created,
            started_with,
            started_with_bytes,
            solution,
            players,
            result: [0, 0],
            status: 3,
        }
    }

    pub fn get_current(&self, user: &String) -> Option<String> {
        if let Some(index) = self.player_index(user) {
            let current = self.current.lock().unwrap();
            let c = String::from(&current[index]);
            drop(current);
            return Some(c);
        }
        None
    }

    pub fn make_move(&self, user: &String, game_move: &String) {
        if let Some(_) = self.clock.current_duration() {
            let m = SudokuGameMove::from(game_move);
            self.make_sudoku_move(user, m);
        }
    }

    pub fn make_sudoku_move(&self, user: &String, game_move: SudokuGameMove) {
        if let Some(index) = self.player_index(user) {
            let mut current = self.current.lock().unwrap();
            if let Some(s) = current.get_mut(index) {
                match game_move {
                    SudokuGameMove::NormalMove { position, number } => {
                        let bytes = String::from(&s.clone());
                        let mut bytes = bytes.into_bytes();
                        if self.started_with_bytes[position as usize] == '.' as u8 {
                            bytes[position as usize] = TEMP_CHARS[(number - 1) as usize] as u8;
                            if let Ok(bytes) = String::from_utf8(bytes) {
                                *s = bytes;
                            }
                        }
                    }
                    SudokuGameMove::DeleteMove { position } => {
                        let bytes = String::from(&s.clone());
                        let mut bytes = bytes.into_bytes();
                        bytes[position as usize] = '.' as u8;
                        if self.started_with_bytes[position as usize] == '.' as u8 {
                            if let Ok(bytes) = String::from_utf8(bytes) {
                                *s = bytes;
                            }
                        }
                    }
                    SudokuGameMove::DeleteAll => {
                        *s = String::from(&self.started_with);
                    }
                }
            }
        }
    }

    pub fn resign(&mut self, user: &String) -> Option<GameResult> {
        if let Some(index) = self.player_index(user) {
            if let Some(_) = self.clock.current_duration() {
                let score = self.final_score();
                self.status = index as u8 + 4;
                self.result[index] = 0;
                self.result[self.other_index(index)] = 1;
                let result =
                    GameResult::new(self.players.clone(), score.1, self.other_index(index), None);
                return Some(result);
            }
        }
        None
    }

    pub fn lost_on_time(&mut self) -> (bool, [u8; 2], [String; 2]) {
        if let Some(_) = self.clock.current_duration() {
            return (false, [0, 0], [String::from(""), String::from("")]);
        }
        let final_score = self.final_score();
        self.status = final_score.0;
        (true, final_score.1, self.players.clone())
    }

    pub fn finished(&mut self, user: &String) -> Option<GameResult> {
        if let Some(i) = self.get_current(user) {
            if i == self.solution {
                let score = self.final_score();
                self.status = score.0;
                let result = GameResult::new(
                    self.players.clone(),
                    score.1,
                    self.player_index(user).unwrap(),
                    None,
                );
                return Some(result);
            }
        }
        None
    }

    pub fn final_score(&mut self) -> (u8, [u8; 2]) {
        let started_with = self.started_with_bytes.clone();
        let solution = self.solution.as_bytes();
        let current_m = self.current.lock().unwrap();
        let current = current_m.clone();
        let current = [current[0].as_bytes(), current[1].as_bytes()];
        drop(current_m);
        let empty_field = '.' as u8;
        let empty_fields: Vec<usize> = started_with
            .iter()
            .enumerate()
            .filter_map(|i| {
                if i.1 == &empty_field {
                    return Some(i.0);
                }
                None
            })
            .collect();

        for player in [0, 1] {
            for empty in &empty_fields {
                if current[player][*empty] == solution[*empty] {
                    self.score[player] += 1;
                }
            }
        }
        self.status_from_score(&self.score)
    }

    pub fn is_playing(&self, username: &String) -> bool {
        if let Some(_) = self.player_index(username) {
            return true;
        }
        false
    }

    fn status_from_score(&self, score: &[u8; 2]) -> (u8, [u8; 2]) {
        if score[0] == score[1] {
            return (2, score.clone());
        } else if score[0] > score[1] {
            return (0, score.clone());
        } else if score[1] > score[0] {
            return (1, score.clone());
        }
        return (3, [0, 0]);
    }

    fn player_index(&self, user: &String) -> Option<usize> {
        if let Some(index) = self.players.iter().position(|x| x == user) {
            return Some(index);
        }
        None
    }

    /// Opposite index of specified.
    fn other_index(&self, index: usize) -> usize {
        let b: bool = index != 0;
        usize::from(!b)
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
