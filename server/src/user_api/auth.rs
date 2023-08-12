use serde::Deserialize;

use super::types::*;
use super::validate;

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    username: String,
    password: String,
}

impl LoginPayload {
    pub fn new(username: impl ToString, password: impl ToString) -> LoginPayload {
        LoginPayload {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

pub fn signup(payload: LoginPayload) -> Result<String, SignupError> {
    let mut users = load();
    let LoginPayload { username, password } = payload;

    validate::signup(&users, &username, &password)?;

    // new cookie
    let (cookie, cookie_value) = Cookie::new();

    // create user
    let user = User {
        username: username.to_string(),
        cookies: vec![Cookie::password(&password), cookie],
    };

    // add user to list
    users.list.insert(username.to_string(), user);

    save(users);
    Ok(cookie_value)
}

pub fn login(payload: LoginPayload) -> Result<LoginSuccess, LoginError> {
    let mut users = load();
    let LoginPayload { username, password } = payload;

    let cookie = validate::login(&users, &username, &password)?;

    // inserts a new cookie if user logged in with password
    if cookie.is_password {
        let new_cookie = Cookie::new();

        users
            .list
            .get_mut(&username)
            .unwrap()
            .cookies
            .push(new_cookie.0);

        save(users);
        return Ok(LoginSuccess::Cookie(new_cookie.1));
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
            } else {
                return Users::default();
            }

            // Copy every element from users to self
        }
        Err(_) => {
            let res = std::fs::write(
                "./users.json",
                serde_json::to_string(&Users::default()).unwrap(),
            );
            match res {
                Ok(_) => {
                    println!("File created");
                    return load();
                }
                Err(e) => {
                    panic!("Failed to create file");
                }
            }
        }
    }
}

fn save(users: Users) {
    std::fs::write(USERS_PATH, serde_json::to_string_pretty(&users).unwrap())
        .expect("Failed to save database");
}
