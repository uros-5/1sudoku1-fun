use async_session::chrono::Duration;
use serde::{Deserialize, Deserializer, Serializer};
use std::time::Duration as StdD;

/// Serializing from Duration to u64 
pub fn duration_i32<S>(x: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let duration = x.num_milliseconds() as u64;
    s.serialize_u64(duration)
}


/// Deserializing from u64 to Duration
pub fn i32_duration<'de, D>(data: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s: u64 = Deserialize::deserialize(data)?;
    let d2 = StdD::from_millis(s);
    if let Ok(d2) = Duration::from_std(d2) {
        return Ok(d2);
    }
    println!("error while deserializing");
    Ok(Duration::minutes(1))
}