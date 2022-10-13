use self::mongo::Mongo;
use self::p_key::PKey;
use self::session::RedisCli;

pub mod helpers;
pub mod mongo;
pub mod p_key;
pub mod queries;
pub mod serde_helpers;
pub mod session;

#[derive(Clone)]
pub struct Database {
    pub redis: RedisCli,
    pub mongo: Mongo,
    pub p_key: PKey,
}

impl Database {
    pub async fn new() -> Self {
        let mongo = Mongo::new().await;
        let redis = RedisCli::new().await;
        let p_key = PKey::new();
        Self {
            mongo,
            redis,
            p_key,
        }
    }
}
