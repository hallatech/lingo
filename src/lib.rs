use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;

mod config;
pub use crate::config::*;

const YES: char = char::from_u32(0x2705).unwrap();
const NO: char = char::from_u32(0x274C).unwrap();

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.filename)?;

    for line in contents.lines() {
        if !line.contains(args.separator) {
            return Err(format!("line {} does not contain a valid separator", line).into());
        }
    }

    let result = load(&args, &contents);

    random_play(&args, result);

    Ok(())
}

pub fn random_play(args: &Args, map: HashMap<&str, &str>) {
    let from = if args.values_to_keys {
        &args.translation
    } else {
        &args.lang
    };

    let to = if args.values_to_keys {
        &args.lang
    } else {
        &args.translation
    };

    println!("Translate the given {} word or phrase to {}.", from, to);

    let mut attempts = 0;
    let mut correct = 0;

    for i in 1..(args.attempts + 1) {
        let rk = fastrand::choice(map.keys()).unwrap();
        println!("{:?} - Translate: {}", i, rk);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to input line");

        attempts += 1;

        if args.verbose {
            println!(
                "Checking translation: required:[{}] entered:[{}]",
                *map.get(rk).unwrap(),
                guess.trim()
            );
        }

        if guess.trim() == *map.get(rk).unwrap() {
            println!("Yes! You got it right. {}", YES);
            correct += 1;
        } else {
            println!("Sorry, not quite right. {}", NO);
        }
    }

    println!("\nYour stats:");
    println!("Attempted: {}", attempts);
    println!("Correct: {}", correct);
    let p = (correct as f64 / attempts as f64) * 100.0;
    println!("Percentage correct: {}%", p.round());
}
