// This program is a command line tool.
// It generates random number codes
// and tries to crack them
// It also measures time taken to crack them
// And shows information about it

// This program can be computationally heavy
// and expensive on weak machines!

// Note! The code isn't very organized
// and is a big mess right now!

// There is a lot of code duplication
// that could be fixed!

// #![allow(unused)]

use rand::{
    Rng,
    rngs::ThreadRng
};

use std::time::Instant;
use std::io;

fn gen_code(rng: &mut ThreadRng) -> u32 {
    println!("Generating random code...");

    let random_code = rng.gen_range(0..=1_000_000);

    println!("Random code: {}", random_code);

    random_code
}

fn gen_seven_digit_code(rng: &mut ThreadRng) -> String {
    println!("Generating random seven digit code...");

    // code format is "0 000 000"

    let code = rng.gen_range(0..=9_999_999);
    let mut final_code: String;

    final_code = match code {
        0..=9 => format!("000000{code}"),
        10..=99 => format!("00000{code}"),
        100..=999 => format!("0000{code}"),
        1_000..=9_999 => format!("000{code}"),
        10_000..=99_999 => format!("00{code}"),
        100_000..=999_999 => format!("0{code}"),
        1_000_000..=9_999_999 => format!("{code}"),
        _ => {
            eprintln!("Invalid code!");
            String::from("0000000")
        }
    };

    final_code.insert(1, ' ');
    final_code.insert(5, ' ');

    println!("{final_code}");

    final_code
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

fn crack_seven_digit_code(code: &str) {
    println!("\nCracking seven digit code...");

    let mut attempt: u32 = 0;
    let mut attempt_text: String;
    let start_time = Instant::now();

    loop {
        attempt_text = match attempt {
            0..=9 => format!("000000{attempt}"),
            10..=99 => format!("00000{attempt}"),
            100..=999 => format!("0000{attempt}"),
            1_000..=9_999 => format!("000{attempt}"),
            10_000..=99_999 => format!("00{attempt}"),
            100_000..=999_999 => format!("0{attempt}"),
            1_000_000..=9_999_999 => format!("{attempt}"),
            _ => {
                eprintln!("Invalid code!");
                String::from("0000000")
            }
        };
        attempt_text.insert(1, ' ');
        attempt_text.insert(5, ' ');

        print!("{attempt_text}\r");
        
        if attempt_text == code {
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

fn init_with_text_code() {
    let mut rng = rand::thread_rng();
    let code = gen_seven_digit_code(&mut rng);

    // Loop every possible value
    // until code is cracked

    crack_seven_digit_code(&code);
}

fn init_code_cracker() {
    let mut rng = rand::thread_rng();
    let code = gen_code(&mut rng);
    
    println!("Choose cracking method by typing its number:");
    println!("1. Range");
    println!("2. Loop");
    println!("3. Range and loop");
    println!("4. Text code");
    
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
        4 => {
            init_with_text_code();
        },
        _ => panic!("Invalid input!"),
    }
}

fn main() {
    init_code_cracker();
}
