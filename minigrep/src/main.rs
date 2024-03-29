// This program is a command line tool that
// searches for a given word in a given file
// and tells information about the results.

// WORK IN PROGRESS

#![allow(unused)]

use std::{
    env,
    process,
};

use minigrep::Config;

fn main() {
    // Ignore first argument, which is the program
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
