use std::{env, fs::read_to_string};

fn main() {
    let mut args = env::args();
    let _my_name = args.next().unwrap();
    let pattern = &args.next().unwrap();
    print!("{pattern}");
    for arg in args {
        let file_text = read_to_string(arg.as_str());
        match file_text {
            Ok(file_str) => {
                println!("Scanning {arg} for lines with {pattern}");
                for (line_num, line) in file_str.lines().enumerate() {
                    if line.contains(pattern) {
                        println!("Pattern found at line {line_num} : {line}")
                    }
                }
            }
            Err(e) => {
                println!("Failed to parse {arg} to string with error {e}")
            }
        }
    }
}