use html::metadata::Post;
use search::base_search::{SearchPostData, SearchPostMetaCompile};
use serde_json;
use std::fs;
use std::path::Path;

/// Generates a JSON file with the schema compatible with the search engine.
/// The output is saved to `output_dir/posts.json`.
pub fn generate_posts_json(posts: &[Post], output_dir: &str) {
    let mut posts_data: Vec<SearchPostData> = Vec::new();

    for post in posts {
        let meta = SearchPostMetaCompile {
            title: post.meta.title.clone(),
            tags: post.meta.topics.clone(),
            tldr: post.meta.tldr.clone(),
            path: format!("posts/{}.html", post.name),
        };
        let fields = vec![
            post.meta.title.clone(),
            post.meta.topics.join(", "),
            post.meta.tldr.clone().unwrap_or_default(),
            post.markdown_body.clone(),
        ];
        let post_data = SearchPostData {
            id: post.name.clone(),
            fields,
            meta: serde_json::to_value(&meta).unwrap(),
        };
        posts_data.push(post_data);
    }

    let posts_json_path = Path::new(output_dir).join("posts.json");
    let json_data = serde_json::to_string_pretty(&posts_data).unwrap();
    fs::write(posts_json_path, json_data).unwrap();
    println!("✅ Saved posts.json with {} entries", posts_data.len());
}
