use std::process::Command;

pub fn get_killer(process: &str) -> Command {
    if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.args(["taskkill", "/IM", process, "/F"]);
        cmd
    } else {
        let mut cmd = Command::new("pkill");
        cmd.args([process]);
        cmd
    }
}
