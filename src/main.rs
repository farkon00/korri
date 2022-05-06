use std::env;
use std::fs;
use std::io::ErrorKind;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>().drain(1..).collect();
    if args.is_empty() {
        println!("[ERROR] No arguments provided.");
        std::process::exit(1);
    }
    let filepath = &args[0];
    debug_print_file(filepath);
}

fn file_error(error: ErrorKind, filepath: &str) {
    match error {
        ErrorKind::NotFound => {
            println!("[ERROR] File '{}' not found.", filepath); 
        },
        _ => {
            println!("[ERROR] File I/O error encountered: {:?}", error);
        }
    }

    std::process::exit(1);
}

fn debug_print_file(filepath: &str) {
    let contents = fs::read_to_string(filepath);
    match contents {
        Ok(_) => println!("File found and read successfully!"),
        Err(e) => file_error(e.kind(), filepath),
    }
}
