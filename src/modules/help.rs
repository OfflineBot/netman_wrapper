
pub fn help() {
    println!("This is a simplified NetworkManager working as a nmcli wrapper.");
    println!("By default the settings are made for wlan0");
    println!("How to use:");
    println!("netman --help               For this info text");
    println!("");
    println!("netman                    Launch the Application");
    println!("");
    println!("  help                    Command overview");
    println!("  list                    List possible connections");
    println!("                          Returns <id> <name>");
    println!("  connect <id>            Connect to network with id (id got from list)");
    println!("                          <id> got from netman list");
    println!("  connect <id> <password> Connect to network with id and required password");
    println!("                          <id> got from netman list");
    println!("  disconnect              Disconnects from the current connection");
    println!("  login                   Opens firefox with the login page (if there is some)");
    println!("  status                  Shows some status information");
    println!("  exit                    Exists application");
}


pub fn command_help() {
    println!("");
    println!("  help                    This Command overview");
    println!("  list                    List possible connections");
    println!("                          Returns <id> <name>");
    println!("  connect <id>            Connect to network with id (id got from list)");
    println!("                          <id> got from netman list");
    println!("  connect <id> <password> Connect to network with id and required password");
    println!("                          <id> got from netman list");
    println!("  disconnect              Disconnects from the current connection");
    println!("  login                   Opens firefox with the login page (if there is some)");
    println!("  status                  Shows some status information");
    println!("  exit                    Properly exists application");
    println!("");
}
