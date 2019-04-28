use crate::configuration::Configuration;
use crate::logger;
use std::fs;
use std::path::PathBuf;

pub struct TokenHandler {
    configuration: Configuration,
}

impl TokenHandler {
    pub fn new() -> Self {
        Self {
            configuration: Default::default(),
        }
    }

    pub fn save_oauth(&self, token: &str) {
        self.save_token(&self.configuration.oauth_token_file, token)
    }

    pub fn save_auth(&self, token: &str) {
        self.save_token(&self.configuration.authorization_token_file, token)
    }

    pub fn read_auth(&self) -> String {
        self.read_token(&self.configuration.authorization_token_file, "Authorization Token file does not exist. Make sure you request authorization before proceeding.")
    }

    pub fn read_oauth(&self) -> String {
        self.read_token(&self.configuration.oauth_token_file, "OAuth Token file does not exist. Make sure you request authorization before proceeding.")
    }

    fn save_token(&self, path: &PathBuf, token: &str) {
        match fs::write(path, token) {
            Ok(_) => "ok",
            Err(_) => logger::log("Could not write to token file"),
        };
    }

    fn read_token(&self, path: &PathBuf, message: &str) -> String {
        match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => {
                logger::log(message);
                "no-token".to_owned()
            }
        }
    }
}
