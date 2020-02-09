use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub struct CredentialsError {
    pub message: String,
}

impl CredentialsError {
    pub fn new(message: &str) -> CredentialsError {
        CredentialsError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl From<IoError> for CredentialsError {
    fn from(err: IoError) -> CredentialsError {
        CredentialsError::new(err.description())
    }
}
