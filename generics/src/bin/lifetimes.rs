#![allow(unused)]

fn lifetime_practice() {
    let x = 10;
    let r = &x;

    println!("r is {}", r);
}

fn main() {
    lifetime_practice();
}