use::std::fs::rename;
//use::std::io;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Program exited due to improper usage. Please use only 2 arguments.");
        // for arg in args {
        //     let val = arg.as_str();
        //     println!("{val}");
        // }
        return;
    }
    let file_path = args[0].as_str();
    let new_name = args[1].as_str();
    let result = rename(file_path, new_name);
    match result {
        Ok(_) =>{
            println!("Succesfully renamed {} to {}", file_path, new_name);
        }
        Err(e) => {
            println!("Failed to rename {file_path} to {new_name} with error {e}")
        }
    }
}
