use crate::security;
use random_string::generate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const USERS_PATH: &str = "./users.json";
const COOKIE_LENGTH: usize = 32;
const COOKIE_CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub struct LoadedUser {
    username: String,
}

impl LoadedUser {
    pub fn username(&self) -> &str {
        &self.username
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Cookie {
    pub hash: String,
    pub is_password: bool,
}

impl Cookie {
    // makes a new cookie with a default value
    pub fn new() -> (Cookie, String) {
        let str = generate(COOKIE_LENGTH, COOKIE_CHARSET); // TODO! make this random

        let mut cookie = Cookie::password(&str);
        cookie.is_password = false;

        return (cookie, str);
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
    pub list: HashMap<String, User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub cookies: Vec<Cookie>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum LoginError {
    UsernameNotFound,
    WrongPassword,
}

#[derive(Debug, PartialEq)]
pub enum LoginSuccess {
    Cookie(String),
    LoggedIn,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum SignupError {
    UsernameTaken,
    UsernameTooShort,
    UsernameTooLong,
    PasswordTooShort,
    PasswordTooLong,
}
