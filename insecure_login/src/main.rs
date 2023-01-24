#![allow(unused)]

use std::io;
use insecure_login::*;

// Ask user to enter data
fn handle_signup() -> (String, String) {
    let mut username = String::new();
    let mut password = String::new();
    
    println!("Sign up by giving a username and password");
    println!("Give username:");
    
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    
    println!("Give password:");
    
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    (username, password)
}

fn handle_login(users: &Vec<User>, username: &str, password: &str) {
    println!("Logging in...");
    
    let login_result = login(&users, username, password);
    println!("{}", login_result);
}

fn main() {
    // Store all users
    let mut users: Vec<User> = vec![];

    let (username, password) = handle_signup();
    let username = username.trim();
    let password = password.trim();

    let user = signup(username, password);

    users.push(user);

    for user in users.iter() {
        println!("{:?}", user);
    }

    handle_login(&users, username, password);
}
