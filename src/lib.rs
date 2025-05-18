use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "translations.txt")]
    pub filename: String,
    #[arg(short, long, default_value = "=")]
    pub separator: char,
    #[arg(short, long, default_value = "english")]
    pub lang: String,
    #[arg(short, long)]
    pub translation: String,
}

const DEFAULT_SEPARATOR: char = '=';

impl Args {
    pub fn new(mut args: std::env::Args) -> Result<Args, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No input filename given"),
        };

        let latr = match args.next() {
            Some(arg) => {
                if !arg.contains(':') {
                    return Err("language:translation pairing must contain the ':' separator");
                }
                arg
            }
            None => return Err("language:translation pairing required, e.g. english:french"),
        };
        let (lang, translation) = latr.split_once(':').unwrap();

        let separator = match args.next() {
            Some(arg) => {
                if arg.len() != 1 {
                    return Err("Invalid separator, length must be 1");
                }
                arg.chars().nth(0).unwrap()
            }
            None => DEFAULT_SEPARATOR,
        };

        Ok(Args {
            filename,
            separator,
            lang: lang.to_string(),
            translation: translation.to_string(),
        })
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.filename)?;

    for line in contents.lines() {
        if !line.contains(args.separator) {
            return Err(format!("line {} does not contain a valid separator", line).into());
        }
    }

    let result = load(&args, &contents);

    play(&args, result);

    Ok(())
}

pub fn load<'a>(args: &Args, contents: &'a str) -> HashMap<&'a str, &'a str> {
    let mut hm: HashMap<&str, &str> = HashMap::new();

    for line in contents.lines() {
        let v: Vec<&str> = line.split(args.separator).collect();
        hm.insert(v[0], v[1]);
    }

    hm
}

pub fn play(args: &Args, map: HashMap<&str, &str>) {
    println!("Play {} to {}:", args.lang, args.translation);
    for (k, v) in map.iter() {
        println!("{}:{}", k, v);
    }

    println!("\nPlay {} to {}:", args.translation, args.lang);
    for (k, v) in map.iter() {
        println!("{}:{}", v, k);
    }
}
