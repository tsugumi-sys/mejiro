use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchPostData {
    pub id: String,
    pub fields: Vec<String>,
    pub meta: serde_json::Value,
}

pub trait SearchEngine {
    fn search(&self, query: &str) -> Vec<serde_json::Value>;
}

// NOTE: When compile in mejiro-cli, json data are saved based on this schema.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchPostMetaCompile {
    pub title: String,
    pub tags: Vec<String>,
    pub tldr: Option<String>,
    pub path: String,
}
