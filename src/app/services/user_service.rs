use crate::app::entities::{User, Auth};
use gato_core::kernel::Logger;
use crate::app::helpers::postgres_helper;

pub fn insert_user(name: String, project: String, email: String, password: String) -> Option<User> {
    return match postgres_helper::get_connection() {
        Some(conn) => {
            let sql = "INSERT INTO zorya.users (name, email, password, project) VALUES ($1, $2, $3, $4) RETURNING *;";
            let query = conn.query(sql, &[
                &name, &email, &password, &project
            ]);
            if query.is_ok() {
                for row in &query.unwrap() {
                    return Some(User::new(
                        row.get(0),
                        row.get(1),
                        row.get(2),
                        row.get(3)
                    ));
                }
            }
            return None;
        },
        None => None
    };
}

pub fn find_by_email(project: String, email: String) -> Option<User> {
    let res_conn = postgres_helper::get_connection();
    match res_conn {
        Some(conn) => {
            let sql = "SELECT id, name, email, password FROM zorya.users WHERE email=$1 and project=$2;";
            let query = conn.query(sql, &[
                &email, &project
            ]);
            if query.is_ok() {
                for row in &query.unwrap() {
                    return Some(User::new(
                        row.get(0),
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
    let sql = "select  \n\
            coalesce(u.id,a.id), coalesce(u.name, a.name), coalesce(u.email, a.email), coalesce(u.password, a.password)  \n\
        from zorya.tokens t  \n\
        left join zorya.users u on t.email = u.email \n\
        left join zorya.apis a on t.email = a.email \n\
        where t.\"token\" = $1;";
    let tokens = postgres_helper::select(sql, &[ &token ]);
    if tokens.is_some() {
        for row in &tokens.unwrap() {
            return Some(User::new(
                row.get(0),
                row.get(1),
                row.get(2),
                row.get(3)
            ));
        }
    }
    return None;
}

pub fn save_token(auth: &Auth) -> bool {
    let res_conn = postgres_helper::get_connection();
    match res_conn {
        Some(conn) => {
            let exp = auth.exp as i64;
            let sql = "INSERT INTO zorya.tokens (expiration, email, \"token\") \n\
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

pub fn delete_by_user(user: &User) -> bool {
    let res_conn = postgres_helper::get_connection();
    return match res_conn {
        Some(conn) => {
            let sql = "DELETE FROM zorya.tokens WHERE email = $1;";
            return match conn.execute(sql, &[ &user.email ]) {
                Ok(_e) => true,
                Err(err) => {
                    Logger::error(err.to_string().as_str());
                    false
                }
            }
        },
        None => false
    };
}
