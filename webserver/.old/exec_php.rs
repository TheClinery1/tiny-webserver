use std::process::Command;

pub fn execute(address: String) -> String {
    let mut command = {
        Command::new("php")
        .arg("-f")
        .arg(address)
        .output()
        .unwrap()
    };
    let mut executed = command.stdout;
    let mut output = String::from_utf8(executed).unwrap();
    return output
}
