use gato_core::kernel::Provider;
use gato_apache_cgi::ApacheServiceProvider;
use gato_lambda::LambdaServiceProvider;
use gato_stdout_logger::StdoutLoggerServiceProvider;

pub struct ApplicationServiceProvider { }

impl ApplicationServiceProvider {
    pub fn new() -> Box<Self> {
        return Box::new(ApplicationServiceProvider {  });
    }
}

impl Provider for ApplicationServiceProvider {
    fn boot(&self) {
        if std::env::var("IS_AWS_LAMBDA").unwrap_or("false".to_owned()) == "true" {
            LambdaServiceProvider::new().boot();
            StdoutLoggerServiceProvider::new().boot();
        } else {
            ApacheServiceProvider::new().boot();
        }
    }
}