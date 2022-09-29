use self::mongo::Mongo;
use self::session::MemcachedCli;

pub mod helpers;
pub mod mongo;
pub mod queries;
pub mod session;

#[derive(Clone)]
pub struct Database {
    pub memcached: MemcachedCli,
    pub mongo: Mongo,
}

impl Database {
    pub async fn new() -> Self {
        let mongo = Mongo::new().await;
        let memcached = MemcachedCli::new();
        Self { mongo, memcached }
    }
}
