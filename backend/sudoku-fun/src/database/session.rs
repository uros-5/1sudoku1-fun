use std::sync::Arc;

use async_session::async_trait;
use axum::{
    extract::{Extension, FromRequest, RequestParts},
    headers::Cookie,
    http::HeaderValue,
    TypedHeader,
};

use hyper::{header::SET_COOKIE, HeaderMap, StatusCode};
use redis::aio::ConnectionManager;
use redis::AsyncCommands;

use super::queries::create_for_cookie;
use super::Database;

const AXUM_SESSION_COOKIE_NAME: &str = "axum-session";

#[derive(Clone)]
pub struct UserSession {
    pub username: String,
    pub session: String,
    pub is_new: bool,
}

impl UserSession {
    pub fn new(username: String, session: String, is_new: bool) -> Self {
        Self {
            username,
            session,
            is_new,
        }
    }

    pub fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if self.is_new {
            let cookie = format!("{}={}; Path=/", AXUM_SESSION_COOKIE_NAME, &self.session);
            headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());
        }
        headers
    }
}

#[derive(Clone)]
pub struct RedisCli {
    pub cli: ConnectionManager,
}

impl RedisCli {
    pub async fn new() -> Self {
        let cli = redis::Client::open("redis://127.0.0.1").unwrap();
        let cli = ConnectionManager::new(cli).await.unwrap();
        Self { cli }
    }

    pub async fn get(&mut self, id: &str) -> Option<UserSession> {
        let exist = self.cli.get::<&str, String>(id).await;
        if let Ok(username) = exist {
            return Some(UserSession::new(username, String::from(id), false));
        }
        None
    }

    pub async fn set(&mut self, key: &str, value: &str) -> UserSession {
        let _ = self
            .cli
            .set::<String, String, String>(String::from(key), String::from(value))
            .await;
        UserSession::new(String::from(value), String::from(key), true)
    }

    /*
    pub fn ttl(&self) -> u32 {
        60 * 60 * 24 * 365
    }
    */
    
}

#[async_trait]
impl<B> FromRequest<B> for UserSession
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(db) = Extension::<Arc<Database>>::from_request(req)
            .await
            .expect("db is missing");
        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();

        let mut m = db.redis.clone();

        let session_cookie = cookie
            .as_ref()
            .and_then(|cookie| cookie.get(AXUM_SESSION_COOKIE_NAME));
        if session_cookie.is_none() == false {
            if let Some(session) = m.get(&session_cookie.unwrap()).await {
                return Ok(session);
            }
        }
        Ok(create_for_cookie(&m, &db.mongo.players).await)
    }
}
