use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>().drain(1..).collect();
    if args.len() == 0 {
        println!("[ERROR] No arguments provided.");
        std::process::exit(1);
    }
}
