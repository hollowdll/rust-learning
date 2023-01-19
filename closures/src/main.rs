#![allow(unused)]

fn closures() {
    let x_times_five = |x: i32| -> i32 {
        x * 5
    };

    let compact_closure = |x| x + 10;

    println!("{}", x_times_five(2_000_000));
    println!("{}", compact_closure(42));
}

fn capture_reference() {
    // Fibonacci sequence:
    // Every value is the sum of preceding 2 values
    let mut fibonacci = vec![1,1,2,3,5,8,13,21,34];

    println!("Fibonacci: {:?}", fibonacci);

    let mut mutable_reference = || fibonacci.push(
        &fibonacci[fibonacci.len() - 1] +
        &fibonacci[fibonacci.len() - 2]
    );
    mutable_reference();

    println!("Fibonacci after modification: {:?}", fibonacci);

    for val in fibonacci.iter() {
        println!("{}", val);
    }

    let total: u32 = fibonacci.iter().sum();
    println!("Sum of values: {}", total);

    let average = total / fibonacci.len() as u32;
    println!("Average: {}", average);
}

fn main() {
    closures();
    capture_reference();
}
