use cmd::run;

mod cmd;
pub mod conf;
pub mod pkg;
pub mod prelude;

use prelude::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    run().await?;
    Ok(())
}
