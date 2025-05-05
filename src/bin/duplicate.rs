use std::fs::{copy, OpenOptions};
//use::std::io;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Program exited due to improper usage. Please use only 2 arguments.");
        return;
    }
    let file_path = args[0].as_str();
    let new_name = args[1].as_str();
    let create_result = OpenOptions::new().append(true).create(true).open(new_name);
    match create_result {
        Ok(_) =>{
            let copy_result = copy(file_path, new_name);
            match copy_result {
                Ok(_) => {
                    println!("Succesfully created a clone of {} at {}", file_path, new_name);
                }
                Err(e) => {
                    println!("Failed to create a clone of {file_path} at {new_name} with error {e}")
                }
            }
        }
        Err(e) => {
            println!("Failed to create a clone of {file_path} at {new_name} with error {e}")
        }
    }
}
