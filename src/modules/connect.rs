use std::process::Command;
use std::collections::HashMap;
use itertools::Itertools;
use std::io::{stdin, stdout, Write};


pub fn connect() {
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

    let mut my_hash: HashMap<usize, String> = HashMap::new();
    let list_split = list_output.split('\n').collect::<Vec<&str>>();
    let mut no_dupe: Vec<&str> = list_split.into_iter().unique().collect();
    no_dupe.retain(|&f| !f.trim().is_empty());

    for (id, value) in no_dupe.iter().enumerate() {
        my_hash.insert(id, value.to_string());
    }

    let mut s = String::new();
    println!("{:#?}", my_hash);
    println!("Select id from network you want to login");
    print!("> ");
    stdin().read_line(&mut s).expect("Did not enter correct string!");
    if let Some('\n') = s.chars().next_back() { s.pop(); }
    if let Some('\r') = s.chars().next_back() { s.pop(); }
    let user_id = match s.parse::<usize>() {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Could not get id: {}", e);
            return;
        }
    };
    
    let network_name = my_hash.get(&user_id).expect("Could not get network id");
    println!("Trying to connect to network..");
    let command = format!("nmcli device wifi connect {}", network_name);
    let connect_command = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Could not connect to Internet");
    let connect_output = match String::from_utf8(connect_command.stdout) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to convert stdout to string: {}", e);
            return;
        },
    };
    println!("connected output: {}", connect_output);
    if connect_output.contains("successfully") {
        println!("Connected!");
        return;
    } else if connect_output.contains("Secrets were required") {
        println!("Password required! Enter password:");
        let mut s = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter correct string!");
        if let Some('\n') = s.chars().next_back() { s.pop(); }
        if let Some('\r') = s.chars().next_back() { s.pop(); }
        let password_arg = format!("nmcli device wifi connect {} password {}", network_name, s);
        Command::new("bash")
            .arg("-c")
            .arg(password_arg)
            .output()
            .expect("Could not connect to network with password");

        let status_code = website_status_code();
        if status_code.to_string().starts_with('2') {
            println!("Network should be connected");
            return;
        } else if status_code.to_string().starts_with('3') {
            println!("You probably have to login!");
            return;
        }
        
    } else {
        println!("Some error happened. No ouput where provied (should not happen)");
        println!("Maybe try with adding a password to it");
        println!("Password:");
        let mut s = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter correct string!");
        if let Some('\n') = s.chars().next_back() { s.pop(); }
        if let Some('\r') = s.chars().next_back() { s.pop(); }
        let password_arg = format!("nmcli device wifi connect {} password {}", network_name, s);
        Command::new("bash")
            .arg("-c")
            .arg(password_arg)
            .output()
            .expect("Could not connect to network with password");

        let status_code = website_status_code();
        if status_code.to_string().starts_with('2') {
            println!("Network should be connected");
            return;
        } else if status_code.to_string().starts_with('3') {
            println!("You probably have to login!");
            return;
        }
    }
}


pub fn current_active_conn() -> String {
    let command = Command::new("bash")
        .arg("-c")
        .arg("nmcli -t -f name connection show --active")
        .output()
        .expect("Could not execute Command for showing active connection");

    let command_output: String = match String::from_utf8(command.stdout) {
        Ok(output) => return output,
        Err(e) => {
            eprintln!("Failed to convert stdout to string: {}", e);
            return "".to_string();
        }
    };

    command_output
}


fn website_status_code() -> isize {
    let command = Command::new("bash")
        .arg("-c")
        .arg("curl -o /dev/null -s -w %{http_code} http://example.com")
        .output()
        .expect("Command for checking response code failed");

    let status_code = match String::from_utf8(command.stdout) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to convert stdout to string: {}", e);
            return -1;
        },
    };

    status_code.parse().expect("Code is not a number!")
}


