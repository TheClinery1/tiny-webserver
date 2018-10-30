use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn read_config() -> Vec<String> {
    let mut config_file_location = File::open("./config.conf")
        .expect("An error occured when the config was read.");
    let mut config_file = String::new();
    let mut newline = false;
    config_file_location.read_to_string(&mut config_file);
    if config_file.contains("#!newline") {
        newline = true;
    } else if config_file.contains("#!semicolon") {
        newline = false;
    } else if !config_file.contains("#!semicolon") && !config_file.contains("#!newline") {
        config_file = format!("#!newline\n{}", config_file); // Create the config for newlines.
    }
    let mut parsed_config = Vec::<String>::new();
    let mut parsed_config_locations = Vec::<usize>::new();
    let mut x = 0usize;
    // DEBUG eprintln!("Splitting the config.");
    for elem in config_file.split_terminator('\n'){
        if !elem.starts_with('#') {
            parsed_config.push(elem.to_string());
            // DEBUG eprintln!("Elem: {}", elem);
        }
    }
    x = 0;
    // Remove any newline chars or semicolons.
    // DEBUG eprintln!("Remove newline and semicolons.");
    while x < parsed_config.len() {
        while parsed_config[x].contains('\n') {
            parsed_config[x].pop();
        }
        while parsed_config[x].contains(';') {
            parsed_config[x].pop();
        }
        x = x + 1;
    }
    //DEBUG eprintln!("Finished removing newlines and semicolons. The length parsed_config is: {}", parsed_config.len());
    return parsed_config
}

pub fn parse_request(request: String, files_loc: String, error_loc: String, index_extension: String) -> [String; 2] {
    // DEBUG eprintln!("Parseing the request with files_loc: {}, error_loc: {}, index_extension: {}", files_loc, error_loc, index_extension);
    let mut address = String::new();
    let mut return_value = [String::new(), String::new()];
    let mut response = String::new();
    let error_200 = String::from("HTTP/1.1 200 OK\r\n\r\n");
    let error_404 = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");
    let error_500 = String::from("HTTP/1.1 500 SERVER ERROR\r\n\r\n");
    let is_dir = match request.rfind(".html") {Some(x) => false, None => true};
    let mut end_request = 0usize;
    let mut contents = String::new();
    let mut files_exists = true;
    let mut error_exists = true;
    if !is_dir {
        end_request = request.rfind(".html").unwrap()+5;
        address = request[..end_request].to_string();
    } else if is_dir {
        for elem in request.chars() {
            if elem != ' ' {
                end_request = end_request + 1;
            } else if elem == ' ' {
                break
            }
        }
        address = format!("{}index{}", request[..end_request].to_string(), index_extension);
    }
    match File::open(format!("{}{}", files_loc, &address)) {
        Ok(_) => files_exists = true,
        Err(_) => files_exists = false
    }
    match File::open(format!("{}/404{}", error_loc, index_extension)) {
        Ok(_) => error_exists = true,
        Err(_) => error_exists = false
    }
    if files_exists {
        File::open(format!("{}{}", files_loc, address)).expect("An error occured during the reading of the requested page.").read_to_string(&mut contents);
        return_value[0] = format!("{}{}", error_200, contents);
        return_value[1] = "200".to_string();
    } else if !Path::new(&files_loc).exists() && Path::new(&error_loc).exists() {
        return_value[0] = format!("{}<!DOCTYPE html><html><body><h1>If your seeing this then report it to the server owner.</h1><br><h2>There are no website files.</h2></body></html>", error_500);
        return_value[1] = "500".to_string()
    } else if error_exists {
        File::open(format!("{}/404{}", error_loc, index_extension)).expect("An error occured during the reading of the 404 page.").read_to_string(&mut contents);
        return_value[0] = format!("{}{}", error_404, contents);
        return_value[1] = "404".to_string();
    } else {
        return_value[0] = format!("{}<!DOCTYPE html><html><body><h1>If your seeing this then report it to the server owner.</h1><br><h2>There are no website files nor are there error pages.</body></html>", error_500);
        return_value[1] = "500".to_string()
    }
    // DEBUG eprintln!("return_value: {:?}", return_value);
    return return_value
}
