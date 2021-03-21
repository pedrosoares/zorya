use serde_json::json;
use gato_core::kernel::{ServiceProvider, Response};
use gato_simple_router::{SimpleRouterServiceProvider, SimpleRouter};
use crate::app::controllers::{GuardController, AuthController};
use crate::app::providers::ApplicationServiceProvider;

pub fn boot(service_provider: &mut ServiceProvider) -> () {
    service_provider.register_provider(ApplicationServiceProvider::new());
    service_provider.register_provider(SimpleRouterServiceProvider::new());

    SimpleRouter::get("/", &|_request| Response::new().json(json!({ "api": "0.0.0" })));

    SimpleRouter::post("/{project}/jwt/auth", &AuthController::login);
    SimpleRouter::post("/{project}/jwt/auth/register", &AuthController::register);
    SimpleRouter::post("/{project}/jwt/token/generate", &AuthController::token_generate);
    SimpleRouter::post("/{project}/jwt/logout", &AuthController::logout);
    SimpleRouter::get("/{project}/jwt/user", &AuthController::get_user);
    SimpleRouter::get("/{project}/jwt/auth/authenticated", &AuthController::is_authenticated);
    SimpleRouter::get("/{project}/guard/can/{permission}", &GuardController::can);
}
