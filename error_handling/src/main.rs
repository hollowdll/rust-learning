use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");

    let _text_file = match file_result {
        Ok(file) => file,
        // Panic if not found
        Err(error) => panic!("Problem opening the file: {}", error),
    };
}
