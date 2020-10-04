use gato_core::kernel::ServiceProvider;
use crate::app::{IndexController, AuthController};
use gato_apache_cgi::ApacheServiceProvider;
use gato_simple_router::{SimpleRouterServiceProvider, SimpleRouter};

pub fn boot(service_provider: &mut ServiceProvider) -> () {
    service_provider.register_provider(ApacheServiceProvider::new());
    service_provider.register_provider(SimpleRouterServiceProvider::new());

    SimpleRouter::get("/", &IndexController::index);

    SimpleRouter::post("/auth", &AuthController::login);
    SimpleRouter::get("/auth/is-authenticated", &AuthController::is_authenticated);
}
