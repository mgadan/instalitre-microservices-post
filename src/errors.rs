use std::fmt;
use bcrypt::BcryptError;
use diesel::result;

#[derive(Debug)]
pub enum PostError {
    HashError(BcryptError),
    DBError(result::Error),
    S3PutError(rusoto_core::RusotoError<rusoto_s3::PutObjectError>),
    S3GetError(rusoto_core::RusotoError<rusoto_s3::GetObjectError>),
    S3DeleteError(rusoto_core::RusotoError<rusoto_s3::DeleteObjectError>),
    ValidatorInvalid(validator::ValidationErrors),
    InvalidReadFile(std::io::Error),
    InvalidMultipart(actix_multipart::MultipartError),
    InvalidEnv(std::env::VarError),
    PGConnectionError
}

impl From<actix_multipart::MultipartError> for PostError {
    fn from(error: actix_multipart::MultipartError) -> Self {
        PostError::InvalidMultipart(error)
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::DeleteObjectError>> for PostError {
    fn from(error: rusoto_core::RusotoError<rusoto_s3::DeleteObjectError>) -> Self {
        PostError::S3DeleteError(error)
    }
}

impl From<std::env::VarError> for PostError {
    fn from(error: std::env::VarError) -> Self {
        PostError::InvalidEnv(error)
    }
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

impl From<validator::ValidationErrors> for PostError {
    fn from(error: validator::ValidationErrors) -> Self {
        PostError::ValidatorInvalid(error)
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::PutObjectError>> for PostError {
    fn from(error: rusoto_core::RusotoError<rusoto_s3::PutObjectError>) -> Self {
        PostError::S3PutError(error)
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::GetObjectError>> for PostError {
    fn from(error: rusoto_core::RusotoError<rusoto_s3::GetObjectError>) -> Self {
        PostError::S3GetError(error)
    }
}

impl From<std::io::Error> for PostError {
    fn from(error: std::io::Error) -> Self {
        PostError::InvalidReadFile(error)
    }
}

impl fmt::Display for PostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PostError::HashError(error) => write!(f, "{}", error),
            PostError::DBError(error) => write!(f, "{}", error),
            PostError::ValidatorInvalid(error) => write!(f, "{}", error),
            PostError::S3PutError(error) => write!(f, "{}", error),
            PostError::S3GetError(error) => write!(f, "{}", error),
            PostError::S3DeleteError(error) => write!(f, "{}", error),
            PostError::InvalidReadFile(error) => write!(f, "{}", error),
            PostError::InvalidEnv(error) => write!(f, "{}", error),
            PostError::InvalidMultipart(error) => write!(f, "{}", error),
            PostError::PGConnectionError => write!(f, "error obtaining a db connection")
        }
    }
}