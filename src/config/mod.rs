use std::env;
use std::env::VarError;
use std::result::Result;

pub fn get_config_env(key: &str) -> Result<String, VarError> {
    //todo: The file path should also be read from another file
    //Now, only env variable is supported
    // env::var returns a Result<String, VarError>
    env::var(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_config_env_var() {
        let result = get_config_env();
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
