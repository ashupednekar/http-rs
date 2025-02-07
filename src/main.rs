use cmd::run;

mod cmd;
pub mod pkg;
pub mod conf;
pub mod prelude;

use prelude::Result;

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt::init();
    run().await?;
    Ok(())
}
