use matchit::Router;

use crate::{conf::settings, prelude::Result};

use super::{request::Request, response::Response};

pub mod listen;
pub mod router;

pub type Handler = fn(req: Request) -> Result<Response>;

pub struct HTTPServer {
    pub addr: String,
    pub routes: Router<Handler>,
}

impl HTTPServer {
    pub fn new() -> Self {
        let addr = format!("0.0.0.0:{}", &settings.listen_port);
        let routes = Router::new();
        Self { addr, routes }
    }
}
