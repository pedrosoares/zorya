use crate::app::entities::User;
use crate::app::helpers::postgres_helper;

pub fn find_by_email(project: &String, email: &String) -> Option<User> {
    let sql = "SELECT id, name, email, password FROM apis WHERE email=$1 and project=$2;";
    let apis = postgres_helper::select(sql, &[ &email, &project ]);
    if apis.is_some() {
        for row in &apis.unwrap() {
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
