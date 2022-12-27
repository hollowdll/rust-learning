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
    use std::collections::HashMap;

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 20);

    // Access a value
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(30);

    // Iterate
    for (key, value) in scores.iter() {
        println!("Key: {key}, Value: {value}");
    }

    let text = "hello hello hi hey";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn exercise_one() {
    // Only with an odd number of elements
    let mut list = vec![6,8,2,5,9,8,7,9,5];
    let median: usize;
    let _mode: i32;

    // sort the list
    list.sort();
    println!("{:?}", list);

    let length = list.len() as f64;
    median = (&length / 2.0).floor() as usize;

    println!("The median is {}", &list[median]);
}

fn main() {
    // vector_example();
    // string_example();
    // hashmap_example();
    exercise_one();
}
