use super::types::{Cookie, LoginError, SignupError, Users};
use pwhash::bcrypt;

pub fn signup(users: &Users, usr: &str, password: &str) -> Result<(), SignupError> {
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

    Ok(())
}

pub fn login(users: &Users, usr: &str, key: &str) -> Result<Cookie, LoginError> {
    // check whether username exists
    if !users.list.contains_key(usr) {
        return Err(LoginError::UsernameNotFound);
    }

    // check whether password is correct
    let user = users.list.get(usr).unwrap();
    let cookie_list = user.cookies.iter().find(|&cookie| {
        if !bcrypt::verify(key, &cookie.hash) {
            return false;
        }

        return true;
    });

    match cookie_list {
        Some(cookie) => Ok(Cookie {
            hash: cookie.hash.clone(),
            is_password: cookie.is_password,
        }),
        None => Err(LoginError::WrongPassword),
    }
}
