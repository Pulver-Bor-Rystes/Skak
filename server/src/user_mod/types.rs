use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::security;


pub const USERS_PATH: &str = "./users.json";


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Cookie {
    pub hash: String,
    pub is_password: bool,
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
    pub list: HashMap<String, User>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub cookies: Vec<Cookie>,
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

#[derive(Debug, PartialEq)]
pub enum SignupError {
    UsernameTaken,
    UsernameTooShort,
    UsernameTooLong,
    PasswordTooShort,
    PasswordTooLong,
}