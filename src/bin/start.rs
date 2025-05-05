use std::{env, u32};
use std::io::Error;
use std::fs::read_to_string;
//use std::fs::OpenOptions;
fn main() {
    let mut args = env::args();
    //println!("{args}");
    let my_name = args.next().unwrap();
    let first_arg = &args.nth(0).unwrap();
    let mut line_count = 10;
    let first_char = first_arg.chars().nth(0).unwrap();
    println!("{my_name}");
    println!("{first_arg}");
    if first_char == '-' {
        //args.next();
        println!("First char denotes num");
        let slice = &first_arg[1..];
        let res= slice.parse::<u32>();
        match res {
            Ok(res) =>{
                line_count = res
            }
            Err(e) => {
                println!("Failed to parse '-' identified line count into a U32 integer with error {e}")
            }
        }
    } else {
        println!("Checking a file");
        let input: Result<String, Error> = read_to_string(first_arg.as_str());
        match input {
            Ok(file_str) => {
                println!("Contents of {first_arg}:");
                for (line_num, line) in file_str.lines().enumerate().take(line_count.try_into().unwrap()) {
                    println!("{}: {line}", line_num + 1)
                }
            }   
            Err(e) => {
                println!("{my_name}: Process failed with error {e}");
            }
        }
    }
    for arg in args {
        println!("Checking a file");
        let input: Result<String, Error> = read_to_string(arg.as_str());
        match input {
            Ok(file_str) => {
                println!("Contents of {arg}:");
                for (line_num, line) in file_str.lines().enumerate().take(line_count.try_into().unwrap()) {
                    println!("{}: {line}", line_num + 1)
                }
            }   
            Err(e) => {
                println!("{my_name}: Process failed with error {e}");
            }
        }
    }
}