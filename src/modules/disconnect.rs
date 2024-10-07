use std::process::Command;
use crate::modules::connect::current_active_conn;

pub fn disconnect() {
    let active = current_active_conn();
    let disconn = format!("nmcli connection down {}", active);
    if active.is_empty() { println!("No connection found!"); return }
    Command::new("bash")
        .arg("-c")
        .arg(disconn)
        .output()
        .expect("Could not execute Command for disconnection");
}



