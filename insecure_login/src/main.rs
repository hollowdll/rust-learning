#![allow(unused)]

use insecure_login::*;

fn main() {
    // Store all users
    let mut users: Vec<User> = vec![];

    let user = signup("someuser42", "password1234");
    users.push(user);

    for user in users.iter() {
        println!("{:?}", user);
    }

    let login_result = login(&users, "someuser42", "password1234");
    println!("{}", login_result);
}
