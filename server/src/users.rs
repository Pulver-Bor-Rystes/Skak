use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::security;

const USERS_PATH: &str = "./users.json";


#[derive(Debug, PartialEq)]
pub enum SignupError {
    UsernameTaken,
    UsernameTooShort,
    UsernameTooLong,
    PasswordTooShort,
    PasswordTooLong,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    cookies: Vec<Cookie>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Cookie {
    hash: String,
    is_password: bool,
}


impl Cookie {
    // makes a new cookie with a default value
    pub fn new() -> (Cookie, String) {
        let str = "cookie".to_string();
        
        return (
            Cookie::password(&str),
            str
        )
    }

    pub fn password(password: &str) -> Cookie {
        Cookie {
            hash: security::hash(password),
            is_password: true,
        }
    }
}




#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Users {
    list: HashMap<String, User>
}


#[cfg(test)]
pub fn reset() {
    let users = Users::default();
    save(users);
}


fn load() -> Users {
    let user_content = std::fs::read(USERS_PATH);

    match user_content {
        Ok(content) => {
            let users: Result<Users, serde_json::Error> = serde_json::from_slice(&content);
            
            if users.is_ok() {
                return users.unwrap();
            }
            else {
                return Users::default();
            }

            // Copy every element from users to self
        },
        Err(_) => {
            let res = std::fs::write("./users.json", serde_json::to_string(&Users::default()).unwrap());
            match res {
                Ok(_) => {
                    println!("File created");
                    return load();
                },
                Err(e) => {
                    panic!("Failed to create file");
                }
            }
        }
    }
}


fn save(users: Users) {
    std::fs::write(USERS_PATH, serde_json::to_string(&users).unwrap())
        .expect("Failed to save database");
}


pub fn signup(usr: &str, password: &str) -> Result<String, SignupError> {
    let mut users = load();

    // check whether username is already taken
    if users.list.contains_key(usr) {
        return Err(SignupError::UsernameTaken);
    }

    // check whether password is too short
    if password.len() < 8 {
        return Err(SignupError::PasswordTooShort);
    }

    // check whether password is too long
    if password.len() > 64 {
        return Err(SignupError::PasswordTooLong);
    }

    // check whether username is too short
    if usr.len() < 3 {
        return Err(SignupError::UsernameTooShort);
    }

    // check whether username is too long
    if usr.len() > 32 {
        return Err(SignupError::UsernameTooLong);
    }

    // new cookie
    let (cookie, cookie_value) = Cookie::new();

    // create user
    let user = User {
        username: usr.to_string(),
        cookies: vec![Cookie::password(&password), cookie],
    };

    // add user to list
    users.list.insert(usr.to_string(), user);

    save(users);
    Ok(cookie_value)
}


#[derive(Debug, PartialEq)]
pub enum LoginError {
    UsernameNotFound,
    WrongPassword,
}

#[derive(Debug, PartialEq)]
pub enum LoginSuccess {
    Cookie(String),
    LoggedIn,
}

pub fn login(usr: &str, key: &str) -> Result<LoginSuccess, LoginError> {
    let mut users = load();

    // check whether username exists
    if !users.list.contains_key(usr) {
        return Err(LoginError::UsernameNotFound);
    }

    // check whether password is correct
    let user = users.list.get(usr).unwrap();
    let cookie_list = user.cookies.iter().find(|cookie| {
        if !security::verify(key, &cookie.hash) {
            return false;
        }
        
        return true;
    });

    if let Some(cookie) = cookie_list {
        if cookie.is_password {
            let new_cookie = Cookie::new();

            users.list.get_mut(usr).unwrap().cookies.push(new_cookie.0);

            return Ok(LoginSuccess::Cookie(new_cookie.1))
        }
        else {
            return Ok(LoginSuccess::LoggedIn);
        }
    }

    Err(LoginError::WrongPassword)
}
