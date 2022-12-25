#![allow(dead_code)]

// Practice basics
// Uncomment if needed
/*
#[derive(Debug)]
enum Color {
    Red(String),
    Green(String),
    Blue(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", &self);
    }
}

fn print_color(color: Color) {
    println!("{:?}", color);
}

fn main() {
    let red = Color::Red(String::from("I am red!"));
    let blue = Color::Blue(String::from("I am blue!"));

    print_color(red);
    print_color(blue);

    let message = Message::Write(String::from("Hello world!"));
    message.call();
}
*/


// Practice match

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    /*
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));
    value_in_cents(Coin::Quarter(UsState::Alaska));
    */

    // Option enum
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
    println!("{:?}", five);
}
