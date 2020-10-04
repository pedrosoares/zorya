use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub sub: String,
    pub exp: usize,
    pub token: String
}
