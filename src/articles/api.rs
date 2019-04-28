use crate::authentication::token_handler::TokenHandler;
use crate::configuration::Configuration;
use crate::logger;

static ACTION_DELETE: &str = "delete";
static STATE_UNREAD: &str = "unread";

pub struct API {
    configuration: Configuration,
}

impl API {
    pub fn new() -> Self {
        Self {
            configuration: Default::default(),
        }
    }

    pub fn retrieve(&self) -> serde_json::Value {
        let token_handler = TokenHandler::new();
        let (consumer_key, pocket_retrieve_url, access_token) = (
            &self.configuration.consumer_key,
            &self.configuration.pocket_retrieve_url,
            &token_handler.read_auth(),
        );

        let params = [
            ("consumer_key", consumer_key),
            ("access_token", access_token),
            ("state", &STATE_UNREAD.to_owned()),
        ];
        let response = reqwest::Client::new()
            .post(pocket_retrieve_url)
            .form(&params)
            .send();

        match response {
            Ok(mut response) => {
                let response_text = response.text().unwrap();
                let json: serde_json::Value = serde_json::from_str(&response_text).unwrap();

                json.to_owned()
            }
            Err(_) => {
                logger::log("Could not retrieve Pocket's data");

                serde_json::Value::Null
            }
        }
    }

    pub fn delete(&self, article_ids: Vec<String>) {}
}
