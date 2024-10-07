use std::process::Command;
use itertools::Itertools;

pub fn list() {
    let list_command = Command::new("bash")
        .arg("-c")
        .arg("nmcli -t -f SSID dev wifi")
        .output()
        .expect("Could not execute Command for listing nmcli");

    let list_output = match String::from_utf8(list_command.stdout) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to convert stdout to string: {}", e);
            return;
        },
    };

    let list_split = list_output.split('\n').collect::<Vec<&str>>();
    let mut no_dupe: Vec<&str> = list_split.into_iter().unique().collect();
    no_dupe.retain(|&f| !f.trim().is_empty());

    println!("{:#?}", no_dupe);
}


