use std::fs::remove_file;
use std::env;

fn main() {
    for arg in env::args() {
        let file_path = arg.as_str();
        let res = remove_file(&file_path);
        match res {
            Ok(_empty) => {
                println!("Succesfully removed file at {file_path} with no issues")
            }
            Err(e) => {
                println!("Error: Delete on path {file_path} failed with error {e}");
            }
        }
    }
}
