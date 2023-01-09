// Lifetime parameters are needed with references.
// They tell the lifetimes of references.

#![allow(unused)]

use std::fmt;

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
    // Needed if struct holds a reference
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Uses lifetime elision rule number 1
    fn level(&self) -> i32 {
        5
    }

    // Lifetime elision rule number 3
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn call_important_excerpt() {
    let novel = String::from("Start. End.");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let instance = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", instance.part);
    instance.announce_and_return_part("Hello");
    instance.announce_and_return_part("aaaa");

    // novel and instance go out of scope here.
    // therefore this reference is valid.
}

// No lifetime parameter needed as compiler can infer
// the lifetimes of references here.
// These kind of rules are called lifetime elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("Whitespace found at index {}", i);
            return &s[0..i];
        }
    }

    return &s[..];
}

// Same but with lifetime annotation. Works the same as above.
/* 
fn first_word<'a>(s: &'a str) -> &'a str {
    // code
}
*/

// Static lifetime denotes that a reference can live
// for the entire duration of the program
fn static_lifetime() {
    let string_literal: &'static str= "String literals have static lifetime";
}

// Lifetimes with trait bounds and generics
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    announcement: T,
) -> &'a str
where
    T: fmt::Display,
{
    println!("Announcement! {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // lifetime_practice();
    // call_longest();
    call_important_excerpt();
    // println!("First word is {}", first_word("Hello world!"));
}