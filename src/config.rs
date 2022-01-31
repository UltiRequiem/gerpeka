use clap::Parser;

#[derive(Parser)]
#[clap(author = "UltiRequiem", version = "0.1.0")]
pub struct Args {
    pub processes: Vec<String>,
}

pub fn get_args() -> Args {
    Args::parse()
}
