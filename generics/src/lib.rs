// Traits are similar to interfaces in other languages.
// They are abstract, but can have default behaviour.
// When a type implements a trait, methods in the trait
// will have to be defined with the exact same signature.

pub trait Summary {
    fn summarize_author(&self) -> String;

    // Default implementation
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    // Omit to get default implementation of Summary trait
    /*
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    */
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

use std::fmt;

// Implement custom display format
impl fmt::Display for Tweet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {}, {}, {})",
            self.username, self.content, self.reply, self.retweet)
    }
}


// Traits as function parameters //

// Impl trait syntax
pub fn say_something(item1: &impl Summary, _item2: &(impl Summary + fmt::Display)) {
    println!("Something: {}", item1.summarize());
}

// Trait bound syntax
// Type T implements Summary and fmt::Display
pub fn notify<T: Summary + fmt::Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound with where clause
pub fn trait_bound_with_where<T, U>(_item1: &T, _item2: &U) -> String
where
    T: fmt::Display + Summary,
    U: Clone + fmt::Debug,
{
    String::from("no")
}

