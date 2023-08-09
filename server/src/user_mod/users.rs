use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::security;
use super::types::*;
use super::validate;



pub fn signup(usr: &str, password: &str) -> Result<String, SignupError> {
    let mut users = load();

    validate::signup(&users, usr, password)?;

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




pub fn login(usr: &str, key: &str) -> Result<LoginSuccess, LoginError> {
    let mut users = load();

    let cookie = validate::login(&users, usr, key)?;

    // inserts a new cookie if user logged in with password
    if cookie.is_password {
        let new_cookie = Cookie::new();

        users.list.get_mut(usr).unwrap().cookies.push(new_cookie.0);

        return Ok(LoginSuccess::Cookie(new_cookie.1))
    }

    Ok(LoginSuccess::LoggedIn)
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