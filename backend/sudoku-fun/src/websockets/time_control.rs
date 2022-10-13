use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    arc2,
    database::serde_helpers::{duration_i32, i32_duration},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeControl {
    pub last_click: DateTime<FixedOffset>,
    #[serde(serialize_with = "duration_i32")]
    #[serde(deserialize_with = "i32_duration")]
    pub clock: Duration,
}

impl TimeControl {
    pub fn new(minute: u8) -> Self {
        let clock = Duration::seconds((minute as i64 * 60 as i64 + 4) as i64);
        let last_click = Utc::now().into();
        Self { clock, last_click }
    }

    pub fn current_duration(&self) -> Option<Duration> {
        let elapsed = self.elapsed();
        if let Some(duration) = self.clock.checked_sub(&elapsed) {
            if duration.num_seconds() < 0 {
                return None;
            }
            return Some(duration);
        }
        None
    }

    /// Elapsed time since last click.
    fn elapsed(&self) -> Duration {
        let now: DateTime<FixedOffset> = Utc::now().into();
        now - self.last_click
    }
}

#[derive(Clone)]
pub enum MsgClock {
    LostOnTime(Arc<Mutex<TimeCheck>>),
}


#[derive(Clone)]
pub struct CurrentClock {
    pub finished: bool,
    pub score: [u8; 2],
    pub players: [String; 2],
}

pub struct TimeCheck {
    finished: Arc<Mutex<bool>>,
}

impl TimeCheck {
    pub fn new() -> Self {
        let finished = arc2(false);
        Self { finished }
    }

    pub fn is_finished(&self) -> bool {
        let f = self.finished.lock().unwrap();
        let v = *f;
        drop(f);
        return v;
    }

    pub fn finished(&self) {
        let mut f = self.finished.lock().unwrap();
        *f = true;
    }
}
