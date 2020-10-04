use std::env;
use postgres::{Connection, TlsMode};
use crate::app::entities::{User, Auth};
use gato_core::kernel::Logger;

fn get_connection() -> Option<Connection> {
    let postgres_uri = env::var("DATABASE_URI")
        .unwrap_or("postgresql://postgres:example@postgres_database:5432".to_owned());
    let res_conn = Connection::connect(postgres_uri, TlsMode::None);
    return match res_conn {
        Ok(conn) => Some(conn),
        Err(err) => {
            Logger::error(err.to_string().as_str());
            None
        }
    }
}

pub fn find_by_email(email: String) -> Option<User> {
    let res_conn = get_connection();
    match res_conn {
        Some(conn) => {
            let query = conn.query("SELECT id, name, email, password FROM users WHERE email=$1;", &[
                &email
            ]);
            if query.is_ok() {
                for row in &query.unwrap() {
                    let id: i32 = row.get(0);
                    return Some(User::new(
                        id.to_string(),
                        row.get(1),
                        row.get(2),
                        row.get(3)
                    ));
                }
            }
        },
        None => {}
    }
    return None;
}

pub fn find_by_token(token: String) -> Option<User> {
    let res_conn = get_connection();
    match res_conn {
        Some(conn) => {
            let sql = "SELECT u.id, u.name, u.email, u.password FROM users u \n\
              join tokens t2 on t2.email = u.email \n\
              where t2.\"token\" = $1;";

            let query = conn.query(sql, &[ &token ]);
            if query.is_ok() {
                for row in &query.unwrap() {
                    let id: i32 = row.get(0);
                    return Some(User::new(
                        id.to_string(),
                        row.get(1),
                        row.get(2),
                        row.get(3)
                    ));
                }
            }
        },
        None => {}
    }
    return None;
}

pub fn save_token(auth: &Auth) -> bool {
    let res_conn = get_connection();
    match res_conn {
        Some(conn) => {
            let exp = auth.exp as i64;
            let sql = "INSERT INTO tokens (expiration, email, \"token\") \n\
                VALUES($1, $2, $3) \n\
                ON CONFLICT (email) \n\
                DO \n\
                   UPDATE SET expiration = $1, \"token\" = $3;";
            return match conn.execute(sql, &[
                &exp, &auth.sub, &auth.token
            ]) {
                Ok(_e) => true,
                Err(err) => {
                    Logger::error(err.to_string().as_str());
                    false
                }
            }
        },
        None => {}
    }
    return false;
}
