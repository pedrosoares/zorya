use crate::app::helpers::postgres_helper;
use uuid::Uuid;

pub fn find_permission(permission: String, user_id: Uuid, project: String) -> bool {
    let query = postgres_helper::select(
        "SELECT 1 FROM guard WHERE permission = $1 and user_id = $2 and project = $3", &[
        &permission, &user_id, &project
    ]);
    return query.is_some() && query.unwrap().len() > 0;
}
