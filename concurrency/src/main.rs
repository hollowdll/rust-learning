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

fn simple_message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let message = String::from("Yo!");
        tx.send(message).unwrap();
    });

    let received_message = rx.recv().unwrap();
    println!("Received message: {}", received_message);
}

// Send multiple messages via one transmitter
fn multiple_messages() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread!"),
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Iterate over received messages
    for received in rx {
        // thread::sleep(Duration::from_secs(1));
        println!("Got {}", received);
    }
}

// Send messages from multiple threads
// to one receiver
fn multiple_threads() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let messages = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread!"),
        ];

        for message in messages {
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("Yo"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("spawned"),
            String::from("thread!"),
            String::from("that"),
            String::from("has"),
            String::from("a longer"),
            String::from("message"),
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        // thread::sleep(Duration::from_secs(1));
        println!("Got {}", received);
    }
}

fn main() {
    // thread_example();
    // move_data();
    // simple_message_passing();
    // multiple_messages();
    multiple_threads();
}
