use crate::{
    pkg::{handler::handle, server::HTTPServer},
    prelude::Result,
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "lets you start the application")]
struct Cmd {
    #[command(subcommand)]
    command: Option<SubcommandType>,
}

#[derive(Subcommand)]
enum SubcommandType {
    Listen,
}

pub async fn run() -> Result<()> {
    let args = Cmd::parse();

    match args.command {
        Some(SubcommandType::Listen) => {
            let mut server = HTTPServer::new();
            server.route("/api", handle)?;
            server.listen().await?;
        }
        None => {
            tracing::error!("no subcommand passed");
        }
    }

    Ok(())
}
