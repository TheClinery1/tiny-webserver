use std::fs::File;
use std::io::{self, Read};

pub fn read_config() -> Vec<String> {
    let mut config_file_location = File::open("/test-webserver/config/config.conf")
        .expect("An error occured when the config was read.");
    let mut config_file = String::new();
    config_file_location.read_to_string(&mut config_file);
    let mut parsed_config = Vec::<String>::new();
    let mut parsed_config_locations = Vec::<usize>::new();
    let mut x = 0usize;
    let mut y = 0usize;
    for char in config_file.chars() {
        if (char == ';') {
            parsed_config_locations.push(x);
            y = y + 1;
            if (parsed_config_locations.len() % 2 == 0 || parsed_config_locations.len() % 2 == 1 && parsed_config_locations.len() != 0 && parsed_config_locations.len() != 1) {
                let y2 = y - 1;
                let tmp1 = parsed_config_locations[y2-1]+2;
                let tmp2 = parsed_config_locations[y-1]+2;
                parsed_config.push(config_file[tmp1..tmp2].to_string());
            } else if (parsed_config_locations.len() == 1) {
                parsed_config.push(config_file[..x+2].to_string());
            }
        }
        x = x + 1;
    }
    x = 0;
    let mut config = Vec::<String>::new();
    for iter in &parsed_config {
        if (iter.contains('#') != true) {
            config.push(iter.to_string());
        }
    }
    let mut tmp = Vec::<String>::new();
    for iter in &config {
        let mut iter2 = iter.to_string();
        if iter.contains('\n') {
            while iter2.contains('\n') {
                iter2.pop();
            }
            if iter.contains(';') {
                iter2.pop();
            }
        }
        tmp.push(iter2);
    }
    config = tmp;
    return config
}
