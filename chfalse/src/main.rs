use std::env;

const EXIT_FAILURE: i32 = 1;
const VERSION: &str = "1.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" => {
                println!("Usage: chfalse [OPTIONS]");
                println!("Returns exit status of 1, always");
                println!("");
                println!("OPTIONS:");
                println!("--help\t\tDisplays help documentation");
                println!("");
                println!("--version\tDisplays current version");
            }
            "--version" => {
                println!("chfalse: {VERSION}");
            }
            _ => {
                eprintln!("Unexpected arguement -> {}", args[1]);
                eprintln!("Try 'true --help' for more information");
            }
        };
    }
    std::process::exit(EXIT_FAILURE);
}
