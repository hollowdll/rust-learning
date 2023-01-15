// This program is a command line tool that
// searches for a given word in a given file
// and tells information about the results.

// WORK IN PROGRESS

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
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
