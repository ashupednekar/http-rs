use crate::{conf::settings, prelude::Result};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

pub async fn listen() -> Result<()>{
    let addr = format!("0.0.0.0:{}", &settings.listen_port);
    let ln = TcpListener::bind(&addr).await?;
    tracing::info!("listening at {}", &addr);
    loop {
        let (mut socket, _) = ln.accept().await?;
        tokio::spawn(async move{
            let mut buf = vec![0; 1024];
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read from socket"); 
                if n == 0{
                    return;
                }
                if let Ok(msg) = String::from_utf8(buf[..n].to_vec()) {
                    tracing::info!("{}", msg);
                    let res = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                        msg.len(),
                        msg
                    );
                    if let Err(e) = socket.write_all(res.as_bytes()).await{
                        tracing::error!("error writing to stream: {}", &e);
                    }
                } else {
                    tracing::warn!("received non-UTF8 data");
                }
            }
        });
    }
}
