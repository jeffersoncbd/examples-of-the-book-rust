use std::{env, process};

use minigrep::Configuration;

fn main() {
    let configuration = Configuration::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    if let Err(error) = minigrep::run(configuration) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}
