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

}

fn hashmap_example() {

}

fn main() {
    vector_example();
}
