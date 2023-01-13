// This program is a command line tool that
// searches for a given word in a given file
// and tells information about the results.

// WORK IN PROGRESS

use std::{
    env,
    fs,
};

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // If not enough arguments
        if args.len() < 3 {
            panic!(
                "Error: Not enough arguments! \
                [query] and [file_path] required!"
            );
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

fn main() {
    // Collect arguments passed into the program
    let args: Vec<String> = env::args().collect();
    
    // Ignore first argument, which is the program
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Error reading the file!");

    println!("With text:\n\n{}", contents);
}
