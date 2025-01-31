use std::env;
use std::io::Error;
use std::fs::read_to_string;
//use std::fs::OpenOptions;
fn main() {
    let mut args = env::args();
    let my_name = args.next().unwrap();
    for arg in args {
        let input: Result<String, Error> = read_to_string(arg.as_str());
        match input {
            Ok(file_str) => {
                println!("Contents of {arg}:");
                for (line_num, line) in file_str.lines().enumerate() {
                    println!("{}: {line}", line_num + 1)
                }
            }   
            Err(e) => {
                println!("{my_name}: Process failed with error {e}");
            }
        }
    }
}