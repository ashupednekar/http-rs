use cmd::run;

mod cmd;
pub mod pkg;
pub mod prelude;

use prelude::Result;

#[tokio::main]
async fn main() -> Result<()>{
    run().await?;
    Ok(())
}
