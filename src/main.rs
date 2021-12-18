use std::{env, process};
use minigrep::Config;

fn main() {
    let mut args = env::args();
    let config = Config::new(&mut args).unwrap_or_else(|msg| {
        eprintln!("{}", msg);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error: {}", e);

        process::exit(1);
    }
}