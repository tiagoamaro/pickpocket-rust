use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub id: String,
    pub url: String,
    pub title: String,
}
