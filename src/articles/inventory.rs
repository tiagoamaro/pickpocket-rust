use crate::articles::article::Article;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub articles: HashMap<String, Article>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            articles: HashMap::new(),
        }
    }
}
