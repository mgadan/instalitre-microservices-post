use std::fmt;
use bcrypt::BcryptError;
use diesel::result;

pub enum PostError {
    HashError(BcryptError),
    DBError(result::Error),
    PasswordNotMatch(String),
    WrongPassword(String)
}

impl From<BcryptError> for PostError {
    fn from(error: BcryptError) -> Self {
        PostError::HashError(error)
    }
}

impl From<result::Error> for PostError {
    fn from(error: result::Error) -> Self {
        PostError::DBError(error)
    }
}

impl fmt::Display for PostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PostError::HashError(error) => write!(f, "{}", error),
            PostError::DBError(error) => write!(f, "{}", error),
            PostError::PasswordNotMatch(error) => write!(f, "{}", error),
            PostError::WrongPassword(error) => write!(f, "{}", error)
        }
    }
}