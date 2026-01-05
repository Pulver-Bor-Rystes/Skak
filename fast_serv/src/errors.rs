use std::error::Error;
use std::fmt;

use crate::parsing::Json;

#[derive(Debug)]
pub enum RequestError {
    NotLoggedIn,
    RatingNotFound,
    TopicParsingFailed,
    RequestHadNoHandler,
    JsonParsingError,
    NoUserFound,
    WrongPassword,
}

impl Json for RequestError {
    fn to_json(&self) -> String {
        format!("{self}")
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            whatever => write!(f, "{:?}", whatever),
        }
    }
}

impl Error for RequestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}
