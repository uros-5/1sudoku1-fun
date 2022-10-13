use std::fs::File;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PKey {
    pub prod: bool,
}

impl PKey {
    pub fn new() -> Self {
        let file_path = Path::new("src/database/p_key.json");

        if let Ok(file) = File::open(file_path) {
            if let Ok(key) = serde_json::from_reader::<File, PKey>(file) {
                return key;
            }
        }
        Self { prod: false }
    }

    pub fn addr(&self) -> (&'static str, &'static str) {
        // (backend, frontend)
        if self.prod {
            ("https://lishuuro.org/w", "https://lishuuro.org")
        } else {
            ("http://localhost:9000", "http://localhost:3000")
        }
    }
}
