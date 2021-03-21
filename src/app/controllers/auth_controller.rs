use gato_core::kernel::{Request, Response};
use serde_json::json;
use crate::app::services::auth_service;
use crate::app::helpers::string_helper::s;


pub struct AuthController {}

impl AuthController {

    pub fn login(request: &Request) -> Response {
        let project = request.get_param("project").unwrap_or(s(""));
        let email = request.get("email").as_str().unwrap_or("").to_string();
        let password = request.get("password").as_str().unwrap_or("").to_string();

        return match auth_service::authenticate(project, email, password) {
            Some(token) => {
                Response::new().json(json!({
                    "expiration": token.exp,
                    "email": token.sub,
                    "token": token.token
                }))
            },
            None => {
                Response::new().status(401).json(json!({
                    "errors": [
                        "Email or Password are incorrect!"
                    ]
                }))
            }
        }
    }

    pub fn logout(request: &Request) -> Response {
        return match auth_service::get_user(request.get_headers()) {
            Some(user) => {
                if auth_service::logout(&user) {
                    return Response::new().status(200).json(json!({
                        "success": "Logout successfully"
                    }));
                }
                return Response::new().status(500).json(json!({
                    "errors": [
                        "Something went wrong"
                    ]
                }));
            },
            None => Response::new().status(401).json(json!({
                "errors": [
                    "User is not authenticated"
                ]
            }))
        };
    }

    pub fn is_authenticated(request: &Request) -> Response {
        let headers = request.get_headers();
        if headers.contains_key("Authorization") {
            let token =  headers["Authorization"].replace("Bearer ", "");
            if auth_service::validate(token.as_str()) {
                return Response::new().json(json!({
                    "success": "User is authenticated"
                }));
            }
        }
        return Response::new().status(401).json(json!({
            "errors": [
                "User is not authenticated"
            ]
        }));
    }

    pub fn get_user(request: &Request) -> Response {
        let headers = request.get_headers();
        if headers.contains_key("Authorization") {
            let token =  headers["Authorization"].replace("Bearer ", "");
            let user = auth_service::get_user_from_token(token.as_str());
            if user.is_ok() {
                return Response::new().json(json!({
                    "success": "User is authenticated",
                    "user": user.unwrap()
                }));
            }
        }
        return Response::new().status(401).json(json!({
            "errors": [
                "User is not authenticated"
            ]
        }));
    }

    pub fn register(request: &Request) -> Response {
        let password = request.get("password").as_str().unwrap_or("").to_string();
        return match auth_service::hash_password(password.as_str()) {
            Some(hashed) => Response::new().json(json!({ "hash": hashed })),
            None => Response::new().json(json!({ "error": "Password cannot be empty" }))
        }
    }

    /**
    *   Authenticate using  UNLIMITED token Lifetime, generally used API to API communication
    */
    pub fn token_generate(request: &Request) -> Response {
        let project = request.get_param("project").unwrap_or(s(""));
        let email = request.get("email").as_str().unwrap_or("").to_string();
        let password = request.get("password").as_str().unwrap_or("").to_string();

        return match auth_service::generate_token(project, email, password) {
            Some(token) => {
                Response::new().json(json!({
                    "email": token.sub,
                    "token": token.token
                }))
            },
            None => {
                return Response::new().status(401).json(json!({
                    "errors": [
                        "Email or Password are incorrect!"
                    ]
                }));
            }
        }
    }

}
