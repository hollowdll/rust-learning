// Rust concept examples

#![allow(dead_code)]

fn variable_example(num: u8) {
    let immutable_var: u8 = 10;
    let mut mutable_var: u8 = 15;
    const ONE_DAY_IN_SECONDS: u32 = 60 * 60 * 24;

    // Have to be of same type (u8 in this case)
    mutable_var += immutable_var + num;

    println!("\nmutable_var after modification is {}", mutable_var);
    println!("One day in seconds is {}", ONE_DAY_IN_SECONDS);
}

fn tuple_example() {
    let my_tuple: (u32, f64, i16) = (1234, 5.3, -7);
    
    // destructure
    let (a, b, c) = my_tuple;
    println!("({a}, {b}, {c})");
}

fn array_example() {
    let my_array: [u32; 3] = [1, 2, 3];

    println!("Last element of my_array is {}\n", my_array[2]);

    for num in my_array {
        println!("{num}");
    }
    println!("End");
}

fn main() {
    // variable_example(8);
    // tuple_example();
    array_example();
}
