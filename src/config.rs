use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "translations.txt")]
    pub filename: String,
    #[arg(short, long, default_value = "=")]
    pub separator: char,
    #[arg(short, long, default_value = "Nederlands")]
    pub lang: String,
    #[arg(short, long, default_value = "French")]
    pub translation: String,
    #[arg(short = 'k', long)]
    pub values_to_keys: bool,
    #[arg(short, long, default_value_t = 10)]
    pub attempts: u16,
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,
}

pub fn load<'a>(args: &Args, contents: &'a str) -> HashMap<&'a str, &'a str> {
    let mut hm: HashMap<&str, &str> = HashMap::new();

    for line in contents.lines() {
        let v: Vec<&str> = line.split(args.separator).collect();
        if args.values_to_keys {
            hm.insert(v[1], v[0]);
        } else {
            hm.insert(v[0], v[1]);
        }
    }

    hm
}

pub fn list(args: &Args, map: HashMap<&str, &str>) {
    println!("List {} to {}:", args.lang, args.translation);
    for (k, v) in map.iter() {
        println!("{}:{}", k, v);
    }

    println!("List {} to {}:", args.translation, args.lang);
    for (k, v) in map.iter() {
        println!("{}:{}", v, k);
    }
}
