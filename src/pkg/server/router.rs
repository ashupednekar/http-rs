use matchit::Router;

use crate::{
    pkg::{request::Request, response::StatusCode},
    prelude::Result,
};

use super::{HTTPServer, Handler};

impl HTTPServer {
    pub fn route(&mut self, path: &str, handler: Handler) -> Result<()> {
        self.routes.insert(path, handler)?;
        Ok(())
    }
}

pub async fn route(request: Request, routes: Router<Handler>) -> Result<Vec<u8>> {
    tracing::info!("routing to: {}", &request.path);
    let res = match routes.at(&request.path) {
        Ok(matched) => {
            let handler = matched.value;
            let response = handler(request)?;
            response.to_bytes()
        }
        Err(_) => {
            let mut status = StatusCode::NotFound.as_http().into_bytes();
            status.extend_from_slice("Content-Length: 0\r\n\r\n".as_bytes());
            status
        }
    };
    Ok(res)
}
