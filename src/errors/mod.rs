use std::fmt;

#[derive(Debug)]
pub struct CredentialsError {
    message: String,
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
