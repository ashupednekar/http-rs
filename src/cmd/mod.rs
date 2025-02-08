use crate::{pkg::server::listen::listen, prelude::Result};
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
    Cli,
}

pub async fn run() -> Result<()> {
    let args = Cmd::parse();

    match args.command {
        Some(SubcommandType::Listen) => {
            listen().await?;
        }
        Some(SubcommandType::Cli) => {}
        None => {
            tracing::error!("no subcommand passed");
        }
    }

    Ok(())
}
