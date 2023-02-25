// Check lib.rs to customize!

//#![allow(unused)]

use data_structure_visualizer::{
    DataStructure,
};
use std::io;

fn init() {
    let mut name = String::new();
    let mut field_1 = String::new();
    let mut field_2 = String::new();
    let mut field_3 = String::new();
    let mut field_4 = String::new();

    let stdin = io::stdin();

    println!("Enter name:");
    stdin
        .read_line(&mut name)
        .expect("Failed to read name");

    println!("Enter field_1:");
    stdin
        .read_line(&mut field_1)
        .expect("Failed to read field_1");

    println!("Enter field_2:");
    stdin
        .read_line(&mut field_2)
        .expect("Failed to read field_2");

    println!("Enter field_3:");
    stdin
        .read_line(&mut field_3)
        .expect("Failed to read field_3");

    println!("Enter field_4:");
    stdin
        .read_line(&mut field_4)
        .expect("Failed to read field_4");

    let data_structure = DataStructure {
        name: name.trim().to_string(),
        field_1: field_1.trim().to_string(),
        field_2: field_2.trim().to_string(),
        field_3: field_3.trim().to_string(),
        field_4: field_4.trim().to_string(),
    };

    println!("\n{:?}", data_structure);

    data_structure.draw();
    DataStructure::draw_structure();
}

fn main() {
    init();
}
