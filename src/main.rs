use std::path::Path;
mod common;
mod config;
mod errors;

fn main() {
    let aws_env_var = "AWS_CREDS_FILE_PATH";
    match config::get_config_env(aws_env_var) {
        Ok(env) => {
            let _ = common::parse_credentials_file(Path::new(&env.to_string()));
            ()
        }
        Err(e) => {
            println!("{}", e);
            ()
        }
    }
}
