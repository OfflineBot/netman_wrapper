
use std::process::Command;

pub fn status() {
    let status = Command::new("bash")
        .arg("-c")
        .arg("nmcli device show wlan0")
        .output()
        .expect("Could not execute Command for status");


    let status_out = match String::from_utf8(status.stdout) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to convert stdout to string: {}", e);
            return;
        },
    };

    let status_collection = status_out.split('\n').collect::<Vec<&str>>();
    println!("{:#?}", status_collection);
}


