use bcrypt::{hash, verify};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Duration, Utc};
use gato_core::kernel::Logger;
use crate::app::services::user_service;
use crate::app::entities::Auth;

pub fn authenticate(email: String, password: String) -> Option<Auth> {
    if email != "" && password != "" {
        let user = user_service::find_by_email(email.to_string());
        match user {
            Some(user) => {
                let matches = verify(password, user.password.as_str()).unwrap_or(false);
                if matches {
                    let now = Utc::now() + Duration::hours(24);

                    let mut my_claims = Auth { sub: email.to_string(), exp: now.timestamp() as usize, token: "".to_owned() };
                    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap_or("".to_string());

                    my_claims.token = token.clone();

                    if user_service::save_token(&my_claims) {
                        return Some(my_claims);
                    }
                }
            },
            None => {}
        }
    }
    return None;
}

pub fn validate(token: &str) -> bool {
    let key = &DecodingKey::from_secret("secret".as_ref());
    let validation = Validation::default();
    let res_token = decode::<Auth>(&token, &key, &validation);

    if res_token.is_ok() {
        let user = user_service::find_by_token(token.to_string());
        return user.is_some();
    }

    match res_token.err() {
        Some(err) => Logger::error(err.to_string().as_str()),
        None => {}
    }

    return false;
}

pub fn hash_password(password: &str) -> Option<String> {
    if password != "" {
        let hashed = hash(password, 4).unwrap();
        return Some(hashed);
    }
    return None;
}

