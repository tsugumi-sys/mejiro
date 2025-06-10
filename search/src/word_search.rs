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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_post(id: u32, fields: &[&str]) -> SearchPostData {
        SearchPostData {
            id: id.to_string(),
            fields: fields.iter().map(|s| s.to_string()).collect(),
            meta: json!({ "id": id }),
        }
    }

    #[test]
    fn search_is_case_insensitive() {
        let posts = vec![
            make_post(1, &["Hello World"]),
            make_post(2, &["rust language"]),
        ];

        let engine = WordSearchEngine::new(posts);
        let results = engine.search("hello");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0]["id"], 1);

        let results_upper = engine.search("RUST");
        assert_eq!(results_upper.len(), 1);
        assert_eq!(results_upper[0]["id"], 2);
    }

    #[test]
    fn search_matches_complete_words() {
        let posts = vec![
            make_post(1, &["rust lang"]),
            make_post(2, &["crust test"]),
        ];

        let engine = WordSearchEngine::new(posts);
        let results = engine.search("rust");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0]["id"], 1);
    }

    #[test]
    fn search_returns_empty_for_no_match() {
        let posts = vec![make_post(1, &["hello world"])];
        let engine = WordSearchEngine::new(posts);
        let results = engine.search("absent");
        assert!(results.is_empty());
    }
}
