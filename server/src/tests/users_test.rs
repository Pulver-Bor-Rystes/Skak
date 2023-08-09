const USR: &str = "rasmus";
const PSW: &str = "kodeord1234";


#[cfg(test)]
mod signup {
    use crate::{users, tests::users_test::{USR, PSW}, user_mod::types::*};

    #[test]
    fn username_taken() {
        users::reset();
        let _ = users::signup(USR, "kodeord123");
        let res = users::signup(USR, PSW);
        assert_eq!(res, Err(SignupError::UsernameTaken));
    }

    #[test]
    fn signup() {
        users::reset();
        let res = users::signup(USR, PSW);
        assert_eq!(res.is_ok(), true)
    }

    #[test]
    fn username_too_short() {
        users::reset();
        let res = users::signup("ra", "kodeord123");
        assert_eq!(res, Err(SignupError::UsernameTooShort))
    }

    #[test]
    fn username_too_long() {
        users::reset();
        let res = users::signup("rasmusrasmusrasmusrasmusrasmusrasmus123", "kodeord123");
        assert_eq!(res, Err(SignupError::UsernameTooLong))
    }

    #[test]
    fn psw_too_short() {
        users::reset();
        let res = users::signup(USR, "1231");
        assert_eq!(res, Err(SignupError::PasswordTooShort))
    }

    #[test]
    fn psw_too_long() {
        users::reset();
        let res = users::signup(USR, "kodeord1kodeord1kodeord1kodeord123kodeord1kodeord1kodeord1kodeord123kodeord1kodeord1kodeord1kodeord123");
        assert_eq!(res, Err(SignupError::PasswordTooLong))
    }
}


#[cfg(test)]
mod login {
    use crate::{users, tests::users_test::{PSW, USR}, user_mod::types::*};

    #[test]
    fn with_psw() {
        users::reset();
        users::signup(USR, PSW)
            .expect("Failed to signup");

        let res = users::login(USR, PSW);
        
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn with_cookie() {
        users::reset();
        let cookie = users::signup(USR, PSW)
            .expect("Failed to signup");

        let res = users::login(USR, &cookie);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn no_user() {
        users::reset();
        let res = users::login(USR, PSW);

        assert_eq!(res, Err(LoginError::UsernameNotFound))
    }
    #[test]
    fn wrong_psw() {
        users::reset();
        let cookie = users::signup(USR, PSW)
            .expect("Failed to signup");

        let res = users::login(USR, "asdbsd");
        assert_eq!(res, Err(LoginError::WrongPassword))
    }
}