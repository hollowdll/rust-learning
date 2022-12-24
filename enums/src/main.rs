#![allow(dead_code)]

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
