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

#[cfg(test)]
mod common_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_parse_credentials_file() {
        let result = parse_credentials_file(Path::new(".aws/credentials"));
        println!("{:?}", result);
    }
}
