use gato_core::kernel::{Request, Response};
use serde_json::json;
use crate::app::services::auth_service;


pub struct AuthController {}

impl AuthController {

    pub fn login(request: &Request) -> Response {
        let project = request.get_param("project", "");
        let email = request.get("email".to_owned()).as_str().unwrap_or("").to_string();
        let password = request.get("password".to_owned()).as_str().unwrap_or("").to_string();

        return match auth_service::authenticate(project, email, password) {
            Some(token) => {
                Response::json(json!({
                    "expiration": token.exp,
                    "email": token.sub,
                    "token": token.token
                }))
            },
            None => {
                let mut response = Response::json(json!({
                    "errors": [
                        "Email or Password are incorrect!"
                    ]
                }));
                response.set_code(401);
                response
            }
        }
    }

    pub fn is_authenticated(request: &Request) -> Response {
        let headers = request.get_headers();
        if headers.contains_key("Authorization") {
            let token =  headers["Authorization"].replace("Bearer ", "");
            if auth_service::validate(token.as_str()) {
                return Response::json(json!({
                    "success": "User is authenticated"
                }));
            }
        }
        let mut response = Response::json(json!({
            "errors": [
                "User is not authenticated"
            ]
        }));
        response.set_code(401);
        return response;
    }

    pub fn register(request: &Request) -> Response {
        let payload = request.json();
        let password = payload["password"].as_str().unwrap_or("");
        return match auth_service::hash_password(password) {
            Some(hashed) => Response::json(json!({ "hash": hashed })),
            None => Response::json(json!({ "error": "Password cannot be empty" }))
        }
    }

    pub fn token_generate(_request: &Request) -> Response {
        return Response::new("");
    }

}
