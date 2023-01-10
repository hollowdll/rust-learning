#![allow(unused)]

// PartialEq trait is needed to do == and != operations
// PartialOrd is needed for comparison, like > and < operations
#[derive(Debug, PartialEq, PartialOrd)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_five(num: i32) -> i32 {
    num + 5
}

// This function's test is intended to fail
fn greeting(name: &str) -> String {
    return format!("Hello");
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }

        Guess { value }
    }
}



// Tests for this library crate
#[cfg(test)]
mod tests {
    // Include everything from parent module
    // super keyword = parent module
    use super::*;

    #[test]
    fn exploration() {
        let num1 = 2;
        let num2 = 3;
        let result = num1 * num2;
        assert_eq!(result, 6);
    }

    // This test always fails
    /*
    #[test]
    fn second_test() {
        panic!("This test fails");
    }
    */

    #[test]
    fn test_string() {
        let some_string = "Almost friday";
        assert_eq!(some_string, "Almost friday");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 12,
        };
        let smaller = Rectangle {
            width: 8,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 6,
            height: 8,
        };
        let smaller = Rectangle {
            width: 5,
            height: 7,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_add_five() {
        assert_eq!(15, add_five(10));
    }

    // Edit greeting in crate root so this this fails
    #[test]
    fn greeting_contains_name() {
        let name = "Alexander";
        let result = greeting(name);
        // Add a custom error message if this test fails
        assert!(
            result.contains(name),
            "Greeting did not contain name, value was \"{}\"",
            result
        );
    }

    // Should panic attribute with expected parameter.
    // Panic message must contain this value in order to pass the test
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Tests can also use Result<T, E> when they fail, not just panic
    #[test]
    fn it_works() -> Result<(), String> {
        if 3 * 3 == 9 {
            Ok(())
        } else {
            Err(String::from("3 times 3 does not equal 9"))
        }
    }
}

