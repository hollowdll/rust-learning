// Smart pointers are data structures.
// They are like pointers but have additional features.
// They implement Deref and Drop traits.
// Rust allows to create custom smart pointers.
// Most common smart pointers are Box<T>, Rc<T> and RefCell<T>

#![allow(unused)]

struct SomeSmartPointer {
    data: String,
}

impl Drop for SomeSmartPointer {
    fn drop(&mut self) {
        println!("Dropping SomeSmartPointer with data {}", self.data);
    }
}

// Box<T> stores and points to data on the heap memory
fn use_case_one() {
    let num: u8 = 42;
    let store_on_heap = Box::new(num);
    let store_text = Box::new("Hello world");

    println!("store_on_heap = {}", store_on_heap);
    println!("store_text = {}", store_text);
}

fn drop_trait_demonstration() {
    let x = SomeSmartPointer {
        data: String::from("example 1"),
    };
    let y = SomeSmartPointer {
        data: String::from("example 2"),
    };
    println!("Created SomeSmartPointers");
}

fn main() {
    drop_trait_demonstration();
}
