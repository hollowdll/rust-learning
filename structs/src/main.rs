#![allow(dead_code)]

// Derive debug trait to use debug formatting
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    height: u16,
    weight: u16,
}

// Methods
impl Person {
    fn age(&self) -> bool {
        self.age > 0
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_height_greater_than_weight(&self) -> bool {
        self.height > self.weight
    }

    fn is_older(&self, other: &Person) -> bool {
        self.age > other.age
    }
}

// Associated function
impl Person {
    fn new() -> Self {
        Self {
            name: String::from(""),
            age: 0,
            height: 0,
            weight: 0,
        }
    }
}

fn main() {
    let person_older = Person {
        name: String::from("Jesus"),
        age: 99,
        height: 180,
        weight: 80,
    };

    let person_younger = Person {
        name: String::from("Joseph"),
        age: 69,
        height: 205,
        weight: 90,
    };

    let person_empty = Person::new();

    println!(
        "\nIs {} older than {}? {}",
        person_older.name(),
        person_younger.name(),
        person_older.is_older(&person_younger)
    );

    println!("\nperson_empty is {:#?}", person_empty);
}
