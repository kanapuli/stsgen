fn main() {
    println!("Hello");
}
pub mod config {
    use std::env;
    use std::env::VarError;
    use std::result::Result;

    pub fn get_config_env() -> Result<String, VarError> {
        //todo: The file path should also be read from another file
        //Now, only env variable is supported
        let key = "AWS_CREDS_FILE_PATH";
        // env::var returns a Result<String, VarError>
        env::var(key)
    }
}
pub mod common {

    use crate::errors::CredentialsError;
    use std::fs;
    use std::path::Path;

    #[derive(Debug)]
    pub struct AwsCredentials {
        aws_access_key_id: String,
        aws_secret_access_key: String,
    }

    pub fn parse_credentials_file(location: &Path) -> Result<AwsCredentials, CredentialsError> {
        match fs::metadata(location) {
            Err(_) => {
                return Err(CredentialsError::new(
                    "Could not stat base AWS credentials file",
                ))
            }
            Ok(metadata) => {
                if !metadata.is_file() {
                    return Err(CredentialsError::new(
                        "Could not read base AWS credentials file",
                    ));
                }
            }
        }
        Ok(AwsCredentials {
            aws_secret_access_key: "".to_string(),
            aws_access_key_id: "".to_string(),
        })
    }
}

pub mod errors {
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
}
#[cfg(test)]
mod common_tests {
    use super::common;
    use std::path::Path;

    #[test]
    fn test_parse_credentials_file() {
        let result = common::parse_credentials_file(Path::new(".aws/credentials"));
        println!("{:?}", result);
    }
}

#[cfg(test)]
mod config_tests {
    use super::config;
    #[test]
    fn check_config_env_var() {
        let result = config::get_config_env();
        match result {
            Ok(val) => {
                println!("{}", val);
                assert!(true, "Got some env var")
            }
            Err(e) => {
                println!("{}", e);
                panic!("Failed to get environment variable")
            }
        }
    }
}
