#[allow(unused)]
const USR: &str = "rasmus";
const PSW: &str = "kodeord1234";

#[cfg(test)]
mod signup {
    use crate::std_format_msgs::content_templates;
    use crate::tests::users_test::{PSW, USR};
    use crate::socket_user_api::types::*;
    use crate::socket_user_api::*;

    #[test]
    fn username_taken() {
        auth::reset();
        let _ = auth::signup(content_templates::Login::new(USR, "kodeord123"));
        let res = auth::signup(content_templates::Login::new(USR, PSW));
        assert_eq!(res, Err(SignupError::UsernameTaken));
    }

    #[test]
    fn signup() {
        auth::reset();
        let res = auth::signup(content_templates::Login::new(USR, PSW));
        assert_eq!(res.is_ok(), true)
    }

    #[test]
    fn username_too_short() {
        auth::reset();
        let res = auth::signup(content_templates::Login::new("ra", "kodeord123"));
        assert_eq!(res, Err(SignupError::UsernameTooShort))
    }

    #[test]
    fn username_too_long() {
        auth::reset();
        let res = auth::signup(content_templates::Login::new(
            "rasmusrasmusrasmusrasmusrasmusrasmus123",
            "kodeord123",
        ));
        assert_eq!(res, Err(SignupError::UsernameTooLong))
    }

    #[test]
    fn psw_too_short() {
        auth::reset();
        let res = auth::signup(content_templates::Login::new(USR, "1231"));
        assert_eq!(res, Err(SignupError::PasswordTooShort))
    }

    #[test]
    fn psw_too_long() {
        auth::reset();
        let res = auth::signup(content_templates::Login::new(USR, "kodeord1kodeord1kodeord1kodeord123kodeord1kodeord1kodeord1kodeord123kodeord1kodeord1kodeord1kodeord123"));
        assert_eq!(res, Err(SignupError::PasswordTooLong))
    }
}

#[cfg(test)]
mod login {
    use crate::{std_format_msgs::content_templates, socket_user_api::types::*};
    use crate::socket_user_api::*;

    use super::{PSW, USR};

    #[test]
    fn with_psw() {
        auth::reset();
        auth::signup(content_templates::Login::new(USR, PSW)).expect("Failed to signup");

        let res = auth::login(content_templates::Login::new(USR, PSW));

        if res.is_err() {
            assert_eq!(res, Ok(LoginSuccess::LoggedIn))
        } else {
            assert!(res.is_ok())
        }
    }

    #[test]
    fn with_cookie() {
        auth::reset();
        let cookie = auth::signup(content_templates::Login::new(USR, PSW)).expect("Failed to signup");

        let res = auth::login(content_templates::Login::new(USR, &cookie));
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn no_user() {
        auth::reset();
        let res = auth::login(content_templates::Login::new(USR, PSW));

        assert_eq!(res, Err(LoginError::UsernameNotFound))
    }
    #[test]
    fn wrong_psw() {
        auth::reset();
        let _ = auth::signup(content_templates::Login::new(USR, PSW)).expect("Failed to signup");

        let res = auth::login(content_templates::Login::new(USR, "asdbsd"));
        assert_eq!(res, Err(LoginError::WrongPassword))
    }
}
