pub mod config;

use std::io::Result;

use gerpeka::kill_process;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::get_args();

    for program in config.processes {
        kill_process(&program).await?
    }

    Ok(())
}
