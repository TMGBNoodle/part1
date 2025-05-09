use std::fs::remove_file;
use std::env;

fn main() {
    let mut args = env::args();
    let _my_name = args.next();
    for arg in args {
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
