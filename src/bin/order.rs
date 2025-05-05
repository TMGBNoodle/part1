use std::{collections::btree_map::Values, env, fs::read_to_string};

fn main() {
    let mut args = env::args();
    let _my_name = args.next().unwrap();
    let first_arg = args.next().unwrap();
    let first_char = first_arg.chars().nth(0).unwrap();
    let mut rev = false;   
    if first_arg == "-r" {
        rev = true;
    } else {
        let file_text = read_to_string(first_arg.as_str());
        match file_text {
            Ok(file_str) => {
                println!("Sorting {first_arg} lines");
                sort_and_print(file_str, rev);
            }
            Err(e) => {
                println!("Failed to parse {first_arg} to string with error {e}")
            }
        }
    }
    for arg in args {
        
        let file_text = read_to_string(arg.as_str());
        match file_text {
            Ok(file_str) => {
                println!("Sorting {arg} lines");
                sort_and_print(file_str, rev);
            }
            Err(e) => {
                println!("Failed to parse {arg} to string with error {e}")
            }
        }
    }
}

fn sort_and_print(file_str: String, dir: bool) {
    let mut lin_vec: Vec<&str> = file_str.split("\n").collect();
    lin_vec.sort();
    if dir {
        lin_vec.reverse();
    }
    let mut line_num = 0;
    for line in lin_vec {
        println!("{line_num}: {line}");
        line_num += 1;
    }
}