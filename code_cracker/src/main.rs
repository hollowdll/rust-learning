// This program is a command line tool.
// It generates random number codes
// and tries to crack them
// It also measures time taken to crack them
// And shows information about it

#![allow(unused)]

use rand::{
    Rng,
    rngs::ThreadRng
};

use std::time::Instant;
use std::io;

fn generate_rand_code(rng: &mut ThreadRng) -> u32 {
    println!("Generating random code...");

    let random_code = rng.gen_range(1..=1_000_000);

    println!("Random code: {}", random_code);

    random_code
}

fn crack_with_range(code: &u32) {
    println!("\nCracking code with range...");

    let start_time = Instant::now();
    
    for n in 0..=*code {
        print!("{}\r", n); 
    }

    let elapsed = start_time.elapsed();
    
    println!("\x1b[93mCode cracked!\x1b[0m");
    println!("\x1b[93mCracked code:\x1b[0m {}", code);
    println!("Time elapsed: {:?}", elapsed);
    println!("Seconds: {}", elapsed.as_secs());
    println!("Exact seconds: {}", elapsed.as_secs_f64());
    println!("Milliseconds: {}", elapsed.as_millis());
    println!("Microsecond: {}", elapsed.as_micros());
    println!("Nanosecond: {}", elapsed.as_nanos());
}

fn crack_with_loop(code: &u32) {
    println!("\nCracking code with loop...");

    let mut attempt: u32 = 0;
    let start_time = Instant::now();

    loop {    
        print!("{}\r", attempt);

        if attempt == *code {
            println!("\x1b[93mCode cracked!\x1b[0m");
            break;
        }
        
        attempt += 1;
    }

    let elapsed = start_time.elapsed();

    println!("\x1b[93mCracked code:\x1b[0m {}", code);
    println!("Time elapsed: {:?}", elapsed);
    println!("Seconds: {}", elapsed.as_secs());
    println!("Exact seconds: {}", elapsed.as_secs_f64());
    println!("Milliseconds: {}", elapsed.as_millis());
    println!("Microsecond: {}", elapsed.as_micros());
    println!("Nanosecond: {}", elapsed.as_nanos());
}

fn main() {
    let mut rng = rand::thread_rng();
    let code = generate_rand_code(&mut rng);

    println!("Choose cracking method by typing its number:");
    println!("1. Range");
    println!("2. Loop");
    println!("3. Range and loop");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line!");
        
    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse!"),
    };
    
    match option {
        1 => {
            crack_with_range(&code);
        },
        2 => {
            crack_with_loop(&code);
        },
        3 => {
            println!("Using range and loop...");
            crack_with_range(&code);
            crack_with_loop(&code);
        },
        _ => panic!("Invalid input!"),
    }
}
