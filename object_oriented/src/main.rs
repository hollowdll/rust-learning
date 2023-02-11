// Small GUI structure with trait objects

#![allow(unused)]

use object_oriented::{
    Draw,
    Button,
    Screen,
};

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{:?}", self);
        // draw select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 50,
                height: 10,
                options: vec![
                    String::from("Dark"),
                    String::from("Light"),
                    String::from("System"),
                ]
            }),
            Box::new(Button {
                width: 30,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(Button {
                width: 40,
                height: 10,
                label: String::from("Cancel"),
            }),
        ],
    };

    screen.run();
}
