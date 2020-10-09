use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub sub: String,
    pub exp: usize,
    pub token: String
}

impl Auth {
    pub fn new(email: &str, expiration: DateTime<Utc>, token: &str) -> Self {
        return Auth {
            sub: email.to_string(), exp: expiration.timestamp() as usize, token: token.to_owned()
        };
    }
}
