use crate::errors::CredentialsError;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct AwsCredentials {
    access_key_id: String,
    secret_access_key: String,
}

impl AwsCredentials {
    pub fn new<K, S>(access_key_id: K, secret_access_key: S) -> AwsCredentials
    where
        K: Into<String>,
        S: Into<String>,
    {
        AwsCredentials {
            access_key_id: access_key_id.into(),
            secret_access_key: secret_access_key.into(),
        }
    }
}

pub fn parse_credentials_file(
    location: &Path,
) -> Result<HashMap<String, AwsCredentials>, CredentialsError> {
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
    };
    let file = File::open(location)?;

    let profile_regex = Regex::new(r"^\[([^\]]+)\]$").unwrap();
    let mut profiles: HashMap<String, AwsCredentials> = HashMap::new();
    let mut access_key_id: Option<String> = None;
    let mut secret_access_key: Option<String> = None;
    let mut profile_name: Option<String> = None;

    let file_lines = BufReader::new(&file);
    for line in file_lines.lines() {
        let unwrapped_line: String = line.unwrap();

        // Ignore commented lines
        if unwrapped_line.starts_with("#") {
            continue;
        }
        //println!("{}", unwrapped_line);
        if profile_regex.is_match(&unwrapped_line) {
            if  profile_name.is_some() && access_key_id.is_some() && secret_access_key.is_some() {
                let credentials =
                    AwsCredentials::new(access_key_id.unwrap(), secret_access_key.unwrap());
                profiles.insert(profile_name.unwrap(), credentials);
            }
        }
        access_key_id = None;
        secret_access_key = None;
        profile_name = None;
    }
    //    println!("{:?}", profiles);
    Err(CredentialsError {
        message: "Hello".to_string(),
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
