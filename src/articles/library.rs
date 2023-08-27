use crate::articles::api::API;
use crate::articles::article::Article;
use crate::articles::inventory::Inventory;
use crate::configuration::Configuration;
use crate::logger;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    read: Inventory,
    unread: Inventory,
}

impl Library {
    pub fn new() -> Library {
        Library {
            read: Inventory::new(),
            unread: Inventory::new(),
        }
    }

    pub fn guarantee_home_folder() {
        let config = Configuration::default();
        match std::fs::create_dir_all(config.home_folder) {
            Ok(_) => {}
            Err(error) => {
                let message = format!("Could not create home folder. Motive: {}", error);
                logger::log(&message);
            }
        };
    }

    fn write_inventory(library: &Library) {
        let config = Configuration::default();
        let library_string = serde_yaml::to_string(library).unwrap();

        std::fs::write(config.library_file, library_string).ok();
    }

    fn load() -> Library {
        let config = Configuration::default();

        if !Path::new(&config.library_file).exists() {
            logger::log("Inventory file not found. Creating...");
            Library::write_inventory(&Library::new());
            File::open(&config.library_file).unwrap();
        }
        logger::log(config.library_file.to_str().unwrap());
        let content = std::fs::read_to_string(config.library_file).unwrap();
        logger::log(&content);
        serde_yaml::from_str::<Library>(&content).unwrap()
    }

    fn random_unread_article() -> Option<Article> {
        let library = Library::load();
        let article_ids: Vec<&String> = library.unread.articles.keys().collect();
        let mut rng = rand::thread_rng();
        let choice = article_ids.choose(&mut rng);

        match choice {
            Some(article_id) => {
                let id = article_id.to_string();
                let article = &library.unread.articles[&id];

                Some(article.to_owned())
            }
            None => None,
        }
    }

    fn move_to_read(article_id: String) {
        let mut library = Library::load();

        match library.unread.articles.remove(&article_id) {
            Some(read_article) => {
                library
                    .read
                    .articles
                    .insert(read_article.id.to_owned(), read_article.to_owned());
            }
            None => {}
        };

        Library::write_inventory(&library);
    }

    pub fn status() {
        let library = Library::load();

        logger::log(&format!(
            "You have {} read articles",
            &library.read.articles.len()
        ));
        logger::log(&format!(
            "You have {} unread articles",
            &library.unread.articles.len()
        ));
    }

    pub fn pick(quantity: Option<usize>) {
        let quantity = quantity.unwrap_or(1);

        for _ in 0..quantity {
            match Library::random_unread_article() {
                Some(article) => {
                    Library::move_to_read(article.id);
                    open::that(article.url).ok();
                }
                None => {
                    logger::log("You have read all articles!");
                }
            };
        }
    }

    pub async fn renew() {
        let api = API::new();
        let library = Library::load();

        // Delete read articles from Pocket
        let read_articles: Vec<&Article> = library.read.articles.values().collect();
        api.delete(read_articles).await;

        // Retrieve new articles from Pocket
        let value = api.retrieve().await; 
        let api_list =  value["list"].to_owned();
        let api_articles =
            match serde_json::from_value::<HashMap<String, serde_json::Value>>(api_list) {
                Ok(articles) => articles,
                Err(_) => HashMap::new(),
            };

        let new_inventory: HashMap<String, Article> = api_articles
            .into_iter()
            .map(|(id, data)| {
                let resolved_title = data["resolved_title"].as_str();
                let given_title = data["given_title"].as_str();

                let title = match resolved_title {
                    Some(title) => title,
                    None => given_title.unwrap_or(""),
                };

                (
                    id.to_string(),
                    Article {
                        id: id.to_owned(),
                        url: data["given_url"].as_str().unwrap().to_owned(),
                        title: title.to_owned(),
                    },
                )
            })
            .collect();

        // Create new Library
        let new_library = Library {
            read: Inventory::new(),
            unread: Inventory {
                articles: new_inventory,
            },
        };

        Library::write_inventory(&new_library);
        logger::log("Refreshed library");
    }
}
