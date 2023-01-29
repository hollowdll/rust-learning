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

fn generate_rand_code(rng: &mut ThreadRng) -> u32 {
    println!("Generating random code...");

    let random_code = rng.gen_range(1..=1_000_000);

    println!("Random code: {}", random_code);

    random_code
}

fn crack_with_range(code: &u32) {
    println!("\x1b[93mCracking code...\x1b[0m");

    let start_time = Instant::now();
    
    for n in 0..=*code {
        print!("{}\r", n); 
    }

    let elapsed = start_time.elapsed();
    
    println!("Code cracked!");
    println!("\x1b[93mCracked code:\x1b[0m {}", code);
    println!("Time elapsed: {:?}", elapsed);
    println!("Seconds: {}", elapsed.as_secs());
    println!("Exact seconds: {}", elapsed.as_secs_f64());
    println!("Milliseconds: {}", elapsed.as_millis());
    println!("Microsecond: {}", elapsed.as_micros());
    println!("Nanosecond: {}", elapsed.as_nanos());
}

fn crack_with_loop() {
    // Todo
    // Use loop instead of range
}

fn main() {
    let mut rng = rand::thread_rng();
    let code = generate_rand_code(&mut rng);

    crack_with_range(&code);
}
