use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    pub separator: char,
    pub lang: String,
    pub translation: String,
}

const DEFAULT_SEPARATOR: char = '=';

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
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

        Ok(Config {
            filename,
            separator,
            lang: lang.to_string(),
            translation: translation.to_string(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    for line in contents.lines() {
        if !line.contains(config.separator) {
            return Err(format!("line {} does not contain a valid separator", line).into());
        }
    }

    let result = load(&config, &contents);

    play(&config, result);

    Ok(())
}

pub fn load<'a>(config: &Config, contents: &'a str) -> HashMap<&'a str, &'a str> {
    let mut hm: HashMap<&str, &str> = HashMap::new();

    for line in contents.lines() {
        let v: Vec<&str> = line.split(config.separator).collect();
        hm.insert(v[0], v[1]);
    }

    hm
}

pub fn play(config: &Config, map: HashMap<&str, &str>) {
    println!("Play {} to {}:", config.lang, config.translation);
    for (k, v) in map.iter() {
        println!("{}:{}", k, v);
    }

    println!("\nPlay {} to {}:", config.translation, config.lang);
    for (k, v) in map.iter() {
        println!("{}:{}", v, k);
    }
}
