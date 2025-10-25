use std::{fs::read, io::{stdin, stdout, Write}};

mod configs;
mod connection_manager;

fn main() {
    while true{
        let mut command = String::new();
        print!("nimpha> ");
        let _=stdout().flush();
        stdin().read_line(&mut command);
        print!("{}", command);
        let args = command.split(" ").map(|s| s.to_string()).collect();
        parse(args);
    }
}

fn parse(tokens: Vec<String>){
    for token in tokens{
        print!("{}\n", token);
    }
}