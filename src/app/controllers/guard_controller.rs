use gato_core::kernel::{Request, Response};
use crate::app::services::{guard_service, user_service};

pub struct GuardController { }

impl GuardController {

    pub fn can(request: &Request) -> Response {
        let headers = request.get_headers();
        if headers.contains_key("Authorization") {
            let token = headers["Authorization"].replace("Bearer ", "");
            let user = user_service::find_by_token(token.to_string());
            if user.is_some() {
                let permission = guard_service::find_permission(
                    request.get_param("permission").unwrap_or("".to_owned()),
                    user.unwrap().id.parse::<i32>().unwrap(),
                    request.get_param("project").unwrap_or("".to_owned()),
                );
                if permission {
                    return Response::new().json(serde_json::json!({
                        "success": "User can access endpoint"
                    }));
                }
            }
        }

        return Response::new().status(401).json(serde_json::json!({
            "error": "User has no access authorization"
        }));
    }

}
