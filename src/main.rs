use clap::Parser;
use lingo::Args;
use log::{debug, error};
use std::process;

fn main() {
    env_logger::init();
    let args = Args::parse();

    debug!("Args:{:?}", args);

    if let Err(e) = lingo::run(args) {
        error!("Application error: {}", e);
        process::exit(1);
    }
}
