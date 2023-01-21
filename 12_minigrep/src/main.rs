use std::{env, process};

use minigrep::Configuration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let configuration = Configuration::from(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    if let Err(error) = minigrep::run(configuration) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}
