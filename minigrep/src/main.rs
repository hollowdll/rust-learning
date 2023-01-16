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
    // Collect arguments passed into the program
    let args: Vec<String> = env::args().collect();
    
    // Ignore first argument, which is the program
    let config = Config::build(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        process::exit(1);
    }
}
