use std::env;
use postgres::{Connection, TlsMode};
use gato_core::kernel::Logger;
use postgres::rows::Rows;
use postgres::types::ToSql;

pub fn get_connection() -> Option<Connection> {
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

pub fn select(sql: &str, params: &[&dyn ToSql]) -> Option<Rows> {
    let res_conn = get_connection();
    return match res_conn {
        Some(conn) => {
            let query = conn.query(sql, params);
            if query.is_ok() {
                return Some(query.unwrap());
            }
            return None
        },
        None => None
    }
}
