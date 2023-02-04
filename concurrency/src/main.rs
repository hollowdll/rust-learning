// Concurrency allows to run code in multiple
// processes called threads.
// Running code simultaneously can improve
// performance, but makes the code more complex.

// Writing multi-threaded programs requires
// extra attention and details!
// Even a small mistake can introduce bugs
// in complex contexts.
// These examples are simple though.

#![allow(unused)]

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn thread_example() {
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

fn move_data() {
    let nums = vec![1,2,3,4,5];

    let handle = thread::spawn(move || {
        println!("Here's a vector in spawned thread: {:?}", nums);
    });

    handle.join().unwrap();
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let message = String::from("Yo!");
        tx.send(message).unwrap();
    });

    let received_message = rx.recv().unwrap();
    println!("Received message: {}", received_message);
}

fn main() {
    // thread_example();
    // move_data();
    message_passing();
}
