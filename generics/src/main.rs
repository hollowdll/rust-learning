#![allow(unused)]

// Avoid code duplication
// No generics
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// No generics
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic version of above functions
// Accepts types that can be compared
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Call with generic function
/*
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'b', 'c'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
*/

// Generic struct
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Only on instances with f64 type
impl Point<f64, f64> {
    fn math_time(&self) -> f64 {
        self.x.powi(2) + self.y.powi(3)
    }
}

/*
fn main() {
    let integer = Point { x: 10, y: -5 };
    let float = Point { x: 2.0, y: 6.0 };
    let integer_and_float = Point { x: 8, y: 4.0 };
}
*/

// Option and Result enums use generics
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Generic type parameters can
// be different in methods
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    // other might have different generic types than self
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

/*
fn main() {
    let p1 = Point2 { x: 6, y: 12.7 };
    let p2 = Point2 { x: "Hi", y: 'f' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
*/



//--------//
// Traits //
//--------//

// From lib.rs
use generics::{
    Summary,
    Tweet,
    NewsArticle
};

fn main() {
    let tweet = Tweet {
        username: String::from("rustacean123"),
        content: String::from("Have a good day!"),
        reply: false,
        retweet: true,
    };

    let article = NewsArticle {
        headline: String::from("Nice article"),
        location: String::from("Toilet"),
        author: String::from("Unknown"),
        content: String::from("Have a nice day!"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", article.summarize());
    println!(
        "Author of tweet: {}\n\
        Author of article: {}",
        tweet.summarize_author(), article.summarize_author()
    );

    // Formatted output. Implements fmt::Display trait in lib.rs
    println!("tweet: {}", tweet);
}


