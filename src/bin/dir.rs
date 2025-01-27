use std::fs;

fn main() {
    for arg in std::env::args() {
        println!("{arg}");
    }
}