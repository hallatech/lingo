use lingo::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lingo::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
