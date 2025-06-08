use crate::base_search::{SearchEngine, SearchPostData};

pub struct WordSearchEngine {
    posts: Vec<SearchPostData>,
}

impl WordSearchEngine {
    pub fn new(posts: Vec<SearchPostData>) -> Self {
        Self { posts }
    }
}
impl SearchEngine for WordSearchEngine {
    fn search(&self, query: &str) -> Vec<serde_json::Value> {
        let query_lower = query.to_lowercase();
        self.posts
            .iter()
            .filter(|post| {
                post.fields.iter().any(|field| {
                    field
                        .to_lowercase()
                        .split_whitespace()
                        .any(|word| word == query_lower)
                })
            })
            .map(|post| post.meta.clone())
            .collect()
    }
}
