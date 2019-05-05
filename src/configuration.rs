use dirs;
use std::env;
use std::path::PathBuf;

pub struct Configuration {
    // Files
    pub authorization_token_file: PathBuf,
    pub home_folder: PathBuf,
    pub library_file: PathBuf,
    pub oauth_token_file: PathBuf,

    // Pocket
    pub consumer_key: String,
    pub pocket_homepage: String,
    pub pocket_oauth_authorize_url: String,
    pub pocket_oauth_request_url: String,
    pub pocket_retrieve_url: String,
    pub pocket_send_url: String,
    pub pocket_user_authorize_url: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            // Files
            authorization_token_file: Self::home_folder().join("authorization_token"),
            home_folder: Self::home_folder(),
            library_file: Self::home_folder().join("library_file"),
            oauth_token_file: Self::home_folder().join("oauth_token"),
            // Pocket
            consumer_key: env::var("POCKET_CONSUMER_KEY")
                .unwrap_or("58132-f824d5fbf935681e22e86a3c".to_owned()),
            pocket_homepage: "https://getpocket.com".to_owned(),
            pocket_oauth_authorize_url: "https://getpocket.com/v3/oauth/authorize".to_owned(),
            pocket_oauth_request_url: "https://getpocket.com/v3/oauth/request".to_owned(),
            pocket_retrieve_url: "https://getpocket.com/v3/get".to_owned(),
            pocket_send_url: "https://getpocket.com/v3/send".to_owned(),
            pocket_user_authorize_url: "https://getpocket.com/auth/authorize".to_owned(),
        }
    }
}

impl Configuration {
    pub fn home_folder() -> PathBuf {
        dirs::home_dir().unwrap().join(".pickpocket")
    }

    pub fn default() -> Self {
        Self { ..Default::default() }
    }
}
