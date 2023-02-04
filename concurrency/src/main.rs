// Concurrency allows to run code in multiple
// processes called threads.
// Running code simultaneously can improve
// performance, but makes the code more complex.

// Writing multi-threaded programs requires
// extra attention and details!
// Even a small mistake can introduce bugs
// and possibly vulnerabilities!

#![allow(unused)]

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for i in 1..5 {
        println!("Hello number {i} from the main thread!");
        thread::sleep(Duration::from_millis(100));
    }
    
    handle.join().unwrap();
}
