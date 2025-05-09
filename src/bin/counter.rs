use std::{env, u32};
use std::io::Error;
use std::fs::read_to_string;
//use std::fs::OpenOptions;
fn main() {
    let mut args = env::args();
    //println!("{args}");
    let my_name = args.next().unwrap();
    let first_arg = &args.nth(0).unwrap();
    let first_char = first_arg.chars().nth(0).unwrap();
    let mut single = ' ';
    let mut valid = true;
    if first_char == '-' {
        let slice = &first_arg[1..];
        let res= slice.parse::<char>();
        match res {
            Ok(res) =>{
                if res == 'w' || res == 'l' || res == 'c' {
                    single = res;
                }
                else {
                    valid = false;
                    println!("Please use 'w', 'l', or 'c' after '-' to indicate what count to display.")
                }
            }
            Err(e) => {
                println!("Token after '-' indicator not valid with error {e}. Please use 'w', 'l', or 'c' to indicate what count to display.")
            }
        }
    } else {
        if valid {
        let input: Result<String, Error> = read_to_string(first_arg.as_str());
        match input {
            Ok(file_str) => {
                println!("{first_arg} counts:");
                let line_count = file_str.split_whitespace().count();
                let char_count = file_str.chars().count();
                let word_count = file_str.lines().count();
                // for (line_num, line) in file_str.lines().enumerate() {
                //     line_count += 1;
                //     for char in line.chars() {
                //         char_count += 1;
                //         if char == ' ' {
                //             word_count += 1;
                //         }
                //     }
                //     word_count += 1;
                // }
                if single == ' ' {
                    println!("Lines: {line_count}");
                    println!("Words: {word_count}");
                    println!("Characters: {char_count}")
                } else {
                    if single == 'w' {
                        println!("Words: {word_count}");
                    }
                    if single == 'l' {
                        println!("Lines: {line_count}");
                    }
                    if single == 'c' {
                    println!("Characters: {char_count}")
                    }
                }
            }   
            Err(e) => {
                println!("{my_name}: Process failed with error {e}");
            }
        }
    }
    }
    if valid {
        for arg in args {
            let input: Result<String, Error> = read_to_string(arg.as_str());
            match input {
                Ok(file_str) => {
                    println!("{arg} counts:");
                    let mut line_count = 0;
                    let mut char_count = 0;
                    let mut word_count = 0;
                    for (line_num, line) in file_str.lines().enumerate() {
                        line_count += 1;
                        for char in line.chars() {
                            char_count += 1;
                            if char == ' ' {
                                word_count += 1;
                            }
                        }
                        word_count += 1;
                    }
                    if single == ' ' {
                        println!("Lines: {line_count}");
                        println!("Words: {word_count}");
                        println!("Characters: {char_count}")
                    } else {
                        if single == 'w' {
                            println!("Words: {word_count}");
                        }
                        if single == 'l' {
                            println!("Lines: {line_count}");
                        }
                        if single == 'c' {
                        println!("Characters: {char_count}")
                        }
                    }
                }   
                Err(e) => {
                    println!("{my_name}: Process failed with error {e}");
                }
            }
        }
    }
}