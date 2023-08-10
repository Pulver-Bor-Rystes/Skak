const USR: &str = "rasmus";
const PSW: &str = "kodeord1234";


#[cfg(test)]
mod signup {
    use crate::tests::users_test::{USR, PSW};
    use crate::user_api::auth::LoginPayload;
    use crate::user_api::types::*;
    use crate::user_api::*;

    #[test]
    fn username_taken() {
        auth::reset();
        let _ = auth::signup(LoginPayload::new(USR, "kodeord123"));
        let res = auth::signup(LoginPayload::new(USR, PSW));
        assert_eq!(res, Err(SignupError::UsernameTaken));
    }

    #[test]
    fn signup() {
        auth::reset();
        let res = auth::signup(LoginPayload::new(USR, PSW));
        assert_eq!(res.is_ok(), true)
    }

    #[test]
    fn username_too_short() {
        auth::reset();
        let res = auth::signup(LoginPayload::new("ra", "kodeord123"));
        assert_eq!(res, Err(SignupError::UsernameTooShort))
    }

    #[test]
    fn username_too_long() {
        auth::reset();
        let res = auth::signup(LoginPayload::new("rasmusrasmusrasmusrasmusrasmusrasmus123", "kodeord123"));
        assert_eq!(res, Err(SignupError::UsernameTooLong))
    }

    #[test]
    fn psw_too_short() {
        auth::reset();
        let res = auth::signup(LoginPayload::new(USR, "1231"));
        assert_eq!(res, Err(SignupError::PasswordTooShort))
    }

    #[test]
    fn psw_too_long() {
        auth::reset();
        let res = auth::signup(LoginPayload::new(USR, "kodeord1kodeord1kodeord1kodeord123kodeord1kodeord1kodeord1kodeord123kodeord1kodeord1kodeord1kodeord123"));
        assert_eq!(res, Err(SignupError::PasswordTooLong))
    }
}


#[cfg(test)]
mod login {
    use crate::user_api::auth::LoginPayload;
    use crate::user_api::types::*;
    use crate::user_api::*;

    use super::{USR, PSW};

    #[test]
    fn with_psw() {
        auth::reset();
        auth::signup(LoginPayload::new(USR, PSW))
            .expect("Failed to signup");

        let res = auth::login(LoginPayload::new(USR, PSW));
        
        if res.is_err() {
            assert_eq!(res, Ok(LoginSuccess::LoggedIn))
        }
        else {
            assert!(res.is_ok())
        }
    }

    #[test]
    fn with_cookie() {
        auth::reset();
        let cookie = auth::signup(LoginPayload::new(USR, PSW))
            .expect("Failed to signup");

        let res = auth::login(LoginPayload::new(USR, &cookie));
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn no_user() {
        auth::reset();
        let res = auth::login(LoginPayload::new(USR, PSW));

        assert_eq!(res, Err(LoginError::UsernameNotFound))
    }
    #[test]
    fn wrong_psw() {
        auth::reset();
        let _ = auth::signup(LoginPayload::new(USR, PSW))
            .expect("Failed to signup");

        let res = auth::login(LoginPayload::new(USR, "asdbsd"));
        assert_eq!(res, Err(LoginError::WrongPassword))
    }
}