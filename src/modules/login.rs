use std::process::Command;

pub fn login() {
    //curl -o /dev/null -s -w %{http_code}\n http://example.com
    let status = Command::new("bash")
        .arg("-c")
        .arg("curl -o /dev/null -s -w %{http_code}\n http://example.com")
        .output()
        .expect("Could not execute response code scripts");

    let status_out = match String::from_utf8(status.stdout) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to convert stdout to string: {}", e);
            return;
        },
    };

    if status_out.starts_with('2') {
        println!("Already Connected");
        return;
    }
    if status_out.starts_with('3') {
        println!("Getting IP4 to connect to login server...");
    }

    let ipv4_gateway = Command::new("bash")
        .arg("-c")
        .arg("nmcli device show wlan0 | grep \"IP4.GATEWAY\" | awk '{print $2}'")
        .output()
        .expect("Could not execute ipv4 code script");

    let ip4_out = match String::from_utf8(ipv4_gateway.stdout) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to convert stdout to string: {}", e);
            return;
        },
    };

    println!("Found {}. Redirecting via firefox..", ip4_out);
    Command::new("firefox")
        .arg(ip4_out)
        .spawn();
}


