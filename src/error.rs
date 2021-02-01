use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum UserBotErrorKind {
    MissingParameter
}

#[derive(Debug)]
pub struct UserBotError {
    kind: UserBotErrorKind,
    text: String
}

impl UserBotError {
    //Create new error
    pub fn new(kind: UserBotErrorKind, text: &str) -> UserBotError {
        UserBotError {
            kind: kind,
            text: text.to_owned()
        }
    }
}

//Display
impl fmt::Display for UserBotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserBot Error: {}", self.text)
    }
}

impl Error for UserBotError {}