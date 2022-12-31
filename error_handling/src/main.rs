#![allow(unused)]

use std::fs::File;
use std::io::{
    self,
    ErrorKind,
    Read
};

fn main() {
    // Error handling with match
    /*
    let file_result = File::open("hello.txt");

    let _text_file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created_file) => created_file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    */

    // Same but another way with closures
    let text_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    

    // Shortcut methods on Result<T, E> for panic
    /*
    let _file_result = File::open("hello.txt").unwrap();
    let _file_result = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
    */

    // Propagate error and return value to the calling code
    /*
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
    
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    
        let mut username = String::new();
    
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    */

    // Shortcut with the ? operator
    /*
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    */


}

// Main with return type
// Returns any kind of error
/*
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
*/
