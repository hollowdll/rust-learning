#![allow(unused)]

fn lifetime_practice() {
    let x = 10;
    let r = &x;

    println!("r is {}", r);
}

// Generic lifetimes parameters in function signature
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn call_longest() {
    let string1 = String::from("long string is long");
    let string2 = "short string";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// Lifetime annotation in struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn call_important_excerpt() {
    let novel = String::from("Start. End.");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let instance = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", instance.part);
    // novel and instance go out of scope here.
    // therefore this reference is valid.
}

fn main() {
    lifetime_practice();
    call_longest();
    call_important_excerpt();
}