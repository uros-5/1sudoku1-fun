use base64::encode;
use rand::Rng;

macro_rules! generate_id {
    ($s: expr, $len: expr) => {
        format!(
            "{}{}",
            $s,
            encode(rand::thread_rng().gen::<[u8; $len]>())
                .replace("+", "")
                .replace("/", "")
                .replace("=", "")
        )
    };
}

pub fn random_username() -> String {
    generate_id!("Anon-", 7)
}

pub fn random_session() -> String {
    String::from(async_session::Session::new().id())
}
