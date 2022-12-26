#![allow(dead_code)]

fn vector_example() {
    let _vector1: Vec<i32> = Vec::new();
    let vector2 = vec![1, 2, 3, 4, 5];

    // No need for type annotation
    // Rust can infer the type (i32 here)
    let mut vector3 = Vec::new();
    vector3.push(10);
    vector3.push(12);
    
    // Read elements
    let first: &i32 = &vector3[0];
    let second: Option<&i32> = vector3.get(1);

    println!("The first element is {}", first);

    match second {
        Some(num) => println!("The second element is {}", num),
        None => println!("No second element!"),
    }

    println!("The first element is {}", &vector3[0]);

    // Iterate
    for i in vector2.iter() {
        println!("{i}");
    }

    // Dereference and modify
    for i in vector3.iter_mut() {
        *i += 100;
    }

    println!("{:?}", vector3);
}

fn string_example() {
    let mut mutable_string = String::new();
    let _immutable_string = String::new();
    let string_literal = "Rust";
    let from_string_literal_1 = "Hello!".to_string();
    let from_string_literal_2 = String::from(" foo");

    mutable_string.push_str(string_literal);
    mutable_string.push('.');
    println!("{}", mutable_string);

    let combined_string = from_string_literal_1 + &from_string_literal_2;
    println!("{}", combined_string);

    let my_string_1 = String::from("This");
    let my_string_2 = String::from("string");
    let my_string_3 = String::from("is");
    let my_string_4 = String::from("blazingly");
    let my_string_5 = String::from("fast");

    // Combine with format! macro
    let clean_syntax = format!("{} {} {} {} {}!",
        my_string_1, my_string_2, my_string_3,
        my_string_4, my_string_5
    );
    println!("{}", clean_syntax);

    let string_slice = &clean_syntax[5..11];
    // Prints "string"
    println!("{}", string_slice);
}

fn hashmap_example() {

}

fn main() {
    //vector_example();
    string_example();
}
