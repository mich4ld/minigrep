use std::{error::Error, fs, env::Args, io::{self, Read}};
use libc::STDOUT_FILENO;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const BOLD: &str = "\x1b[1m";

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut content = String::new();

    if config.filename.is_none() {
        io::stdin().read_to_string(&mut content)?;
    } else {
        content = fs::read_to_string(config.filename.unwrap())?;
    }

    let results = search(&config.query, &content, &config.isatty);

    for result in results {
        println!("{}", result);
    }

    return Ok(());
}

fn highlight(query: &String, line: &String) -> String {
    let colored_keyword = format!("{}{}{}{}", BOLD, RED, query, RESET);
    return line.replace(query, &colored_keyword);
}

fn search(query: &String, content: &String, isatty: &bool) -> Vec<String> {
    let result = content
        .lines()
        .filter(|line| line.contains(query));

    if !*isatty {
        return result.map(|line| line.to_string()).collect();
    }

    return result
        .map(|line| highlight(query, &line.to_string()))
        .collect();
}

pub struct Config {
    pub query: String,
    pub filename: Option<String>,
    pub isatty: bool,
}

impl Config {
    pub fn new(args: &mut Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = args.next();

        let isatty = unsafe { libc::isatty(STDOUT_FILENO) } != 0;
    
        return Ok(Config { query, filename, isatty, });
    }
}