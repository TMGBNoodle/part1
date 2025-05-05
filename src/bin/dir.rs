// use std::env;
// use std::env::args;
use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();
    println!("Listing files in current directory:");
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}
