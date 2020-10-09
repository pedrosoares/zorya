use gato_core::kernel::{Request, Response};

pub struct GuardController { }

impl GuardController {

    pub fn can(_request: &Request) -> Response {
        return Response::new().raw("");
    }

}
