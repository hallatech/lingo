use std::collections::HashMap;
use std::error::Error;
use std::fs;

mod config;
pub use crate::config::*;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.filename)?;

    for line in contents.lines() {
        if !line.contains(args.separator) {
            return Err(format!("line {} does not contain a valid separator", line).into());
        }
    }

    let result = load(&args, &contents);

    list(&args, result);

    Ok(())
}

pub fn play(_args: &Args, _map: HashMap<&str, &str>) {}
