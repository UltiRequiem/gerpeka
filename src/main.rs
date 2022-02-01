use gerpeka::get_killer;
use std::io;

pub mod cli;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = cli::get_args();

    for program in config.programs {
        let mut killer = get_killer(&program);
        killer.spawn()?;
        println!("{} was killed.", program);
    }

    Ok(())
}
