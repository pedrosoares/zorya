use jsonwebtoken::{encode, Header, EncodingKey};
use crate::app::entities::Auth;
use bcrypt::hash;

pub fn s(t: impl Into<String>) -> String {
    return t.into();
}

pub fn jwt(auth: &Auth) -> String {
    return encode(
        &Header::default(),
        &auth,
        &EncodingKey::from_secret("secret".as_ref())
    ).unwrap_or(s(""));
}

pub fn bcrypt(t: impl Into<String>) -> String {
    let password = t.into();
    if password != "" {
        return hash(password, 4).unwrap();
    }
    return "".to_owned();
}
