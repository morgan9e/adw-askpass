use std::process::Command;

use crate::Args;

pub fn send(args: &Args) {
    let _ = Command::new("notify-send")
        .arg("--icon")
        .arg(&args.icon)
        .arg("--app-name")
        .arg(&args.title)
        .arg(&args.message)
        .spawn();
}
