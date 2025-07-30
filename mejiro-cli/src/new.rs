use chrono::Utc;
use rand::{Rng, distr::Alphanumeric};
use std::fs;
use std::io::Write;
use std::path::Path;

use html::metadata::BlogMeta;

pub fn new(output_dir: &str) {
    let output_path = Path::new(output_dir);

    // Check and create the output directory if it doesn't exist
    if !output_path.exists() {
        fs::create_dir_all(output_path).expect("Failed to create output directory");
        let path = output_path.display();
        println!("Created output directory: {path}");
    }

    let date = Utc::now().format("%Y%m%d").to_string();
    let rand_suffix: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    let filename = format!("{date}-{rand_suffix}.md");
    let filepath = output_path.join(&filename);

    let today = Utc::now().format("%Y-%m-%d").to_string();
    let meta = BlogMeta {
        title: "New Post".to_string(),
        topics: vec![],
        published: false,
        published_at: today,
        tldr: Some("A short summary here.".to_string()),
    };

    let yaml_frontmatter = serde_yaml::to_string(&meta).expect("Failed to serialize frontmatter");
    let content = format!("---\n{yaml_frontmatter}---\n\n# New Post\n\nWrite your content here!\n");

    let mut file = fs::File::create(&filepath).expect("Failed to create file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to file");

    let path = filepath.display();
    println!("✅ New blog post created: {path}");
}
