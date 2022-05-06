use std::env;
use std::fs;
use std::io::ErrorKind;

use korri::lexer;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>().drain(1..).collect();
    if args.is_empty() {
        println!("[ERROR] No arguments provided.");
        std::process::exit(1);
    }
    let filepath = &args[0];
    lex_file_from_path(filepath);
}

fn file_error(error: ErrorKind, filepath: &str) {
    match error {
        ErrorKind::NotFound => {
            println!("[ERROR] File '{}' not found.", filepath);
        }
        _ => {
            println!("[ERROR] File I/O error encountered: {:?}", error);
        }
    }

    std::process::exit(1);
}

fn lex_file_from_path(filepath: &str) {
    let file_result = fs::read_to_string(filepath);
    match file_result {
        Ok(v) => {
            println!("File found and read successfully!");
            lexer::lex_file(&v);
        }
        Err(e) => file_error(e.kind(), filepath),
    }
}
