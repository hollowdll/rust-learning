// Integration tests make sure library code's public APIs
// are working properly and everything works together
// the same way, if someone else uses the code outside the project,
// or if used by binary crates. If the code works, it will work
// in binary crates as well, so binary crates won't be needed to test.

use test_practice;

// Declare common directory as module
mod common;

#[test]
fn returns_hello() {
    assert_eq!(common::RETURNS_HELLO_EXPECTED_VALUE, test_practice::greeting("text"));
}