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
    /* OLD
    eprint!("Input IP address: ");
    let mut address = String::new();
    io::stdin().read_line(&mut address).unwrap(); address.pop();
    */
    let mut config = Vec::<String>::new();
    config = misc::read_config();
    let listener = TcpListener::bind(format!("{}:{}", config[0], config[1]))
        .expect("An error occured at the TcpListener::bind");
    // DEBUG eprintln!("Going to try to open the command_file now...");
    let mut log_file = OpenOptions::new().append(true).open("/test-webserver/logs/connections.log").unwrap();
    let mut command_file = OpenOptions::new().write(true).read(true).open("/test-webserver/config/commands.txt").unwrap();
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
                get_response = get(string.to_string(), stream, &config[2], &config[3]);
            }
        }
        log_file.write(&format!("{} New connection from: {} requesting: {} response: {}\n", time, peer_addr, get_response[0], get_response[1]).as_bytes()).unwrap();
    }
}

fn get(request: String, mut stream: TcpStream, files_loc: &String, error_page_loc: &String) -> [String; 2] {
    let mut return_value = [String::new(), String::new()];
    let mut response = String::new();
    let error_ok = String::from("HTTP/1.1 200 OK\r\n\r\n");
    let error_not_found = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");
    // DEBUG println!("New request!\n{}", request);
    let mut address = String::new();
    let mut x = 0usize;
    let mut is_dir = false;
    if request.contains(".html") {
        x = request.rfind(".html").unwrap() + 5;
        return_value[0] = request[..x].to_string();
    } else if request[..].contains("/ ") {
        is_dir = true;
        return_value[0] = request[..x].to_string();
    }
    // DEBUG eprintln!("Just another DEBUG for the address variable: {}", address);
    address.push_str(files_loc);
    if is_dir {
        for iter in request.chars() {
            if iter != ' ' {
                address.push_str(&iter.to_string())
            } else if iter == ' ' {
                address.push_str("index.html");
                break;
            }
        }
    } else if !is_dir {
        address.push_str(&request[..x]);
    }

    if address == files_loc.to_string() {
        address.push_str("index.html");
    }
    let mut contents = String::new();
    if Path::new(&address).exists() {
        let mut file = File::open(&address);
        let mut file = {
            match file {
                Ok(file) => file,
                Err(_) => File::open(format!("{}/404.html", error_page_loc)).unwrap(),
            }
        };
        let mut tmp = {
            match file.read_to_string(&mut contents) {
                Ok(_) => "!200",
                Err(_) => "!404"
            }
        };
        if (tmp == "!404" && tmp != "!200") { // Returns a 404 to the browser
            return_value[1] = "404".to_string();
            // DEBUG eprintln!("The !404 part is running now.");
            // DEBUG eprintln!("The contents before read file: {}", contents);
            contents.clear();
            file = File::open(format!("{}/404.html", error_page_loc)).unwrap();
            // DEBUG eprintln!("The contents after read file: {}", contents);
            file.read_to_string(&mut contents);
            response.push_str(&error_not_found);
            response.push_str(&contents);
        } else { // Returns a 200 to the browser
            return_value[1] = "200".to_string();
            // DEBUG eprintln!("The !good part is running now.");
            // DEBUG eprintln!("The contents before read file: {}", contents);
            contents.clear();
            file = File::open(address).unwrap();
            file.read_to_string(&mut contents);
            // DEBUG eprintln!("The contents after read file: {}", contents);
            response.push_str(&error_ok);
            response.push_str(&contents);
        }
    } else { // Returns a 404 to the browser
        return_value[1] = "404".to_string();
        // DEBUG eprintln!("The master 'else' block is running!");
        let mut file = File::open(format!("{}/404.html", error_page_loc)).unwrap();
        file.read_to_string(&mut contents);
        response.push_str(&error_ok);
        response.push_str(&contents);
    }
    // DEBUG eprintln!("This is the response: {}", response);
    stream.write(&response.as_bytes());
    return return_value;
}
