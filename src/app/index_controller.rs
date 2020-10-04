use serde_json::json;
use gato_core::kernel::{Request, Response};

pub struct IndexController {}
impl IndexController {
    pub fn index(_request: &Request) -> Response {
        return Response::json(json!({
            "api": "0.0.0"
        }));
    }
}
