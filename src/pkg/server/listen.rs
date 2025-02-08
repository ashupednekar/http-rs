use crate::{
    pkg::request::Request,
    prelude::Result,
};
use matchit::Router;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use super::{router::route, HTTPServer, Handler};

impl HTTPServer {
    pub async fn listen(&self) -> Result<()> {
        let ln = TcpListener::bind(&self.addr).await?;
        tracing::info!("listening at {}", &self.addr);
        loop {
            let (socket, _) = ln.accept().await?;
            let routes = self.routes.clone();
            tokio::spawn(async move {
                if handle_connection(socket, routes).await.is_err() {
                    tracing::error!("error handling connection");
                };
            });
        }
    }
}

pub async fn handle_connection(mut socket: TcpStream, routes: Router<Handler>) -> Result<()> {
    let mut buf = vec![0; 1024];
    loop {
        let n = socket.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }
        let body = buf[..n].to_vec(); 

        let request = Request::parse(body)?;
        let res = route(request, routes.clone()).await?;
        socket.write_all(&res).await?
    }
}
