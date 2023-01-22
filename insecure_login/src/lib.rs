// Small login and signup demonstration.
// This is not secure at all
// and should not be used in real life.
// This doesn't hash passwords which is bad practice.

// Prefer using real cryptography algorithms
// such as Argon2, sha256, sha512 etc.

#![allow(unused)]

#[derive(Debug, PartialEq)]
pub struct User {
    username: String,
    password: String,
}

impl User {
    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }
}

pub fn signup(username: &str, password: &str) -> User {
    User {
        username: String::from(username),
        password: String::from(password),
    }
}

pub fn login<'a>(users: &'a [User], username: &'a str, password: &'a str) -> &'a str {
    // Find user with given username
    for v in users.iter() {
        if v.username() == username {
            let user = v;
            let user_password = user.password();

            match user_password == password {
                true => return "Login success!",
                _ => return "Login failed!",
            }
        }
    }

    return "No user found!";
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signup_user() {
        let user = User {
            username: String::from("rustacean42"),
            password: String::from("password123"),
        };

        assert_eq!(user, signup(user.username(), user.password()));
    }

    #[test]
    fn successful_login() {
        let mut users: Vec<User> = vec![];

        let user = User {
            username: String::from("rustacean42"),
            password: String::from("password123"),
        };

        users.push(user);

        assert_eq!("Login success!", login(&users, "rustacean42", "password123"));
    }
}