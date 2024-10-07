#![allow(unused)]

use std::process::Command;
use itertools::Itertools;
use std::io::{stdin, stdout, Write};

mod modules;
use modules::{
    connect,
    disconnect,
    help, 
    command_help,
    list,
    login,
    status
};


fn main() {

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        if args[1] == "--help" {
            help();
            return;
        }
    }

    println!("Enter help for more information");

    let mut running = true;
    while running {

        let mut s = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter correct string!");
        if let Some('\n') = s.chars().next_back() { s.pop(); }
        if let Some('\r') = s.chars().next_back() { s.pop(); }

        let inputs: Vec<&str> = s.split(' ').collect::<Vec<&str>>();

        let command = inputs[0];
        match command as &str {
            "exit" => {
                println!("Exiting netman...");
                running = false;
            },
            "help" => command_help(),
            "list" => list(),
            "disconnect" => disconnect(),
            "login" => login(),
            "status" => status(),
            "connect" => connect(),
            _ => println!("Unkonwn Input: {}! Enter help for more information", command),
        };
    }
}


