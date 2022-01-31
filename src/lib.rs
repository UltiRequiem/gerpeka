use std::{io, process::Command};

pub async fn kill_process(process: &str) -> io::Result<()> {
    Command::new("pkill").args([process]).output()?;
    Ok(())
}
