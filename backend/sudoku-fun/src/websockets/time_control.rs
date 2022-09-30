use chrono::{DateTime, Duration, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::database::serde_helpers::{duration_i32, i32_duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeControl {
    pub last_click: DateTime<FixedOffset>,
    #[serde(serialize_with = "duration_i32")]
    #[serde(deserialize_with = "i32_duration")]
    pub clock: Duration,
}

impl TimeControl {
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
