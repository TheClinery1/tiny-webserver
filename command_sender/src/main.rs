#![allow(unused)]

use std::net::{TcpStream};
use std::net::Shutdown::{self, Both};
use std::io::{self, Write};
use std::fs;

mod parse_ip;

fn main() {
    /*eprint!("Input IP address: ");
    let mut address = String::new();
    io::stdin().read_line(&mut address).unwrap(); address.pop();*/
    let mut address = parse_ip::read_config();
    eprintln!("The address: {}", address);
    eprint!("Input the command you want to run: ");

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap(); command.pop();
    let mut stream = TcpStream::connect(&address).unwrap();
    fs::write("/test-webserver/config/commands.txt", &command);
    stream.write(&command.as_bytes()).unwrap();
    stream.shutdown(Both);
}
