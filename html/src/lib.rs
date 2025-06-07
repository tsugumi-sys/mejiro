use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogMeta {
    pub title: String,
    pub topics: Vec<String>,
    pub published: bool,
    pub published_at: String,
    pub tldr: Option<String>,
}
