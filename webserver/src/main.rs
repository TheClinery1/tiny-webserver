#![allow(unused)]

extern crate chrono;

use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::io::{self, Write, Read};
use std::fs;
use std::fs::File;
use std::str;
use std::path::Path;
use std::fs::OpenOptions;
use chrono::{Utc, TimeZone};

mod misc;

fn main() {
    // DEBUG eprintln!("Server started");
    let mut config = Vec::<String>::new();
    config = misc::read_config();
    let listener = TcpListener::bind(format!("{}:{}", config[0], config[1]))
        .expect("An error occured at the TcpListener::bind");
    // DEBUG eprintln!("Going to try to open the command_file now...");
    let mut log_file = match OpenOptions::new().append(true).open(&config[4]) {
        Ok(x) => x,
        Err(_) => create_file("./commands.txt".to_string(), true)
    };
    let mut command_file = match OpenOptions::new().write(true).read(true).open("./commands.txt") {
        Ok(x) => x,
        Err(_) => create_file("./commands.txt".to_string(), false)
    };
    // DEBUG eprintln!("{:?}", config);
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 512];
        let mut len_stream:usize = stream.peek(&mut buffer).unwrap();
        let mut buffer = vec![0; len_stream];
        stream.read(&mut buffer);
        let mut string = String::from(String::from_utf8_lossy(&mut buffer));
        // DEBUG println!("{:?}", &stream.peer_addr());
        let peer_addr = stream.peer_addr().unwrap();
        // DEBUG println!("{}", &peer_addr);
        let mut get_response = [String::new(), String::new()];
        let mut tmp = String::new();
        command_file.read_to_string(&mut tmp);
        // DEBUG eprintln!("The contents of commands.txt: {}", tmp);
        let mut time = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9).format("%Y-%m-%d %H:%M:%S").to_string();
        // DEBUG eprintln!("Time: {}", time);
        if (string == "!stop" && tmp == "!stop") {
            log_file.write(&format!("Server stopped at: {}\n", time).as_bytes());
            fs::write("/test-webserver/config/commands.txt", "");
            break;
        } else if (string != "!stop") {
            if (string.contains("GET")) {
                string = string[4..].to_string();
                get_response = get(string.to_string(), stream, &config[2], &config[3], &config[5]);
            }
        }
        log_file.write(&format!("{} New connection from: {} requesting: {} response: {}\n", time, peer_addr, get_response[0], get_response[1]).as_bytes()).unwrap();
    }
}

fn create_file(file: String, append: bool) -> File {
    File::create(file);
    OpenOptions::new().append(append).write(true).read(true).open("./commands.txt").unwrap() // Returns the value when a ; isn't used at the end of the line.
}

fn get(request: String, mut stream: TcpStream, files_loc: &String, error_page_loc: &String, file_extension: &String) -> [String; 2] {
    // DEBUG println!("New request!\n{}", request);
    let mut response = misc::parse_request(request, files_loc.to_string(), error_page_loc.to_string(), file_extension.to_string()); // An array of 3 Strings
    // DEBUG eprintln!("This is the response: {}", response);
    stream.write(response[0].as_bytes());
    let return_value = [response[0].clone(), response[1].clone()];
    return return_value;
}
