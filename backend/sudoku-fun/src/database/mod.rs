use self::mongo::Mongo;
use self::session::RedisCli;

pub mod helpers;
pub mod mongo;
pub mod queries;
pub mod session;
pub mod serde_helpers;

#[derive(Clone)]
pub struct Database {
    pub redis: RedisCli,
    pub mongo: Mongo,
}

impl Database {
    pub async fn new() -> Self {
        let mongo = Mongo::new().await;
        let redis = RedisCli::new().await;
        Self { mongo, redis }
    }
}
