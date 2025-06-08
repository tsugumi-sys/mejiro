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
