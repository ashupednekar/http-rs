use crate::{conf::settings, pkg::{handler::handle, request::Request}, prelude::Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub async fn listen() -> Result<()> {
    let addr = format!("0.0.0.0:{}", &settings.listen_port);
    let ln = TcpListener::bind(&addr).await?;
    tracing::info!("listening at {}", &addr);
    loop {
        let (socket, _) = ln.accept().await?;
        tokio::spawn(async move {
            if handle_connection(socket).await.is_err() {
                tracing::error!("error handling connection");
            };
        });
    }
}

pub async fn handle_connection(mut socket: TcpStream) -> Result<()> {
    let mut buf = vec![0; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read from socket");
        if n == 0 {
            return Ok(());
        }
        let request = Request::parse(buf[..n].to_vec())?;
        let response = handle(request)?; 
        if let Err(e) = socket
            .write_all(&response.to_bytes())
            .await
        {
            tracing::error!("error writing to stream: {}", &e);
        }
    }
}
