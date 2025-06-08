use crate::info;
use crate::std_format_msgs::content_templates;

use super::types::*;
use super::validate;


/// Hvis signup gik godt, får brugeren en cookie til at logge ind med fremadrettet
pub fn signup(payload: content_templates::Login) -> Result<String, SignupError> {
    let mut users = load();
    let content_templates::Login { username, password } = payload;

    validate::signup(&users, &username, &password)?;

    // new cookie
    let (cookie, cookie_value) = Cookie::new();

    // create user
    let user = User {
        username: username.to_string(),
        cookies: vec![Cookie::password(&password), cookie],
        rating: 1200,
    };

    // add user to list
    users.list.insert(username.to_string(), user);

    save(users);
    Ok(cookie_value)
}

pub fn login(payload: content_templates::Login) -> Result<LoginSuccess, LoginError> {
    let mut users = load();
    let content_templates::Login { username, password } = payload;

    let cookie = validate::login(&users, &username, &password)?;

    // inserts a new cookie if user logged in with password
    if cookie.is_password {
        let new_cookie = Cookie::new();

        let user = users.list.get_mut(&username).unwrap();

        user.cookies.retain(|cookie| cookie.is_password);
        user.cookies.push(new_cookie.0);

        save(users);
        return Ok(LoginSuccess::Cookie(new_cookie.1));
    }

    Ok(LoginSuccess::LoggedIn)
}

#[allow(dead_code)]
#[cfg(test)]
pub fn reset() {
    let users = Users::default();
    save(users);
}

pub fn load() -> Users {
    let user_content = std::fs::read(USERS_PATH);

    match user_content {
        Ok(content) => {
            let users: Result<Users, serde_json::Error> = serde_json::from_slice(&content);

            if users.is_ok() {
                return users.unwrap();
            } else {
                panic!("Could not load users.json\n{:?}", users.err());
            }
        }
        Err(_) => {
            let res = std::fs::write(
                "./users.json",
                serde_json::to_string(&Users::default()).unwrap(),
            );
            match res {
                Ok(_) => {
                    info!("File created");
                    return load();
                }
                Err(_e) => {
                    panic!("Failed to create file");
                }
            }
        }
    }
}

pub fn save(users: Users) {
    std::fs::write(USERS_PATH, serde_json::to_string_pretty(&users).unwrap())
        .expect("Failed to save database");
}
