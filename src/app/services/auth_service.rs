use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use gato_core::kernel::Logger;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::app::entities::Auth;
use crate::app::helpers::string_helper::jwt;
use crate::app::services::{application_service, user_service};

pub fn generate_token(project: String, email: String, password: String) -> Option<Auth> {
    if project != "" && email != "" && password != "" {
        let user = application_service::find_by_email(&project, &email);
        return match user {
            Some(user) => {
                let matches = verify(password, user.password.as_str()).unwrap_or(false);
                if matches {
                    let now = Utc::now() + Duration::days(99999);
                    let mut auth = Auth::new(email.as_str(), now, "");
                    let token = jwt(&auth);

                    auth.token = token.clone();

                    if user_service::save_token(&auth) {
                        return Some(auth);
                    }
                }
                return None;
            },
            None => None
        }
    }
    return None;
}

pub fn authenticate(project: String, email: String, password: String) -> Option<Auth> {
    if project != "" && email != "" && password != "" {
        let user = user_service::find_by_email(project, email.clone());
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

    res_token.err().and_then(|err| Some(Logger::error(err.to_string().as_str())));

    return false;
}

pub fn hash_password(password: &str) -> Option<String> {
    if password != "" {
        let hashed = hash(password, 4).unwrap();
        return Some(hashed);
    }
    return None;
}

