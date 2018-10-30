use std::fs::File;
use std::io::{self, Read};

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
    for elem in config_file.split_terminator('\n'){
        if !elem.starts_with('#') {
            parsed_config.push(elem.to_string());
        }
        println!("{}", elem);
    }
    x = 0;
    // Remove any newline chars or semicolons.
    while x < parsed_config.len() {
        while parsed_config[x].contains('\n') {
            parsed_config[x].pop();
        }
        while parsed_config[x].contains(';') {
            parsed_config[x].pop();
        }
    }
    return parsed_config
}
