// This program is under development!

//#![allow(unused)]

use data_structure_visualizer::{
    DataStructure,
};

fn main() {
    let data_structure = DataStructure {
        name: String::from("Data structure name"),
        field_1: String::from("Some text"),
        field_2: String::from("Some text"),
        field_3: String::from("Some text"),
        field_4: String::from("Some text"),
    };

    // Debug print
    println!("{:?}", data_structure);
    data_structure.draw();

    // Call draw/visualizer function
}
