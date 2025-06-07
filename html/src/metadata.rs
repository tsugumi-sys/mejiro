use pulldown_cmark::{Parser, html};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogMeta {
    pub title: String,
    pub topics: Vec<String>,
    pub published: bool,
    pub published_at: String,
    pub tldr: Option<String>,
}

pub struct Post {
    pub meta: BlogMeta,
    pub html_body: String,
    pub name: String, // file name (without extension)
}

impl Post {
    pub fn from_markdown_file(path: &Path) -> Option<Self> {
        let content = fs::read_to_string(path).ok()?;
        let (meta, body_md) = parse_markdown_with_meta(&content)?;
        if !meta.published {
            return None;
        }

        let parser = Parser::new(&body_md);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Some(Post {
            meta,
            html_body: html_output,
            name: path.file_stem().unwrap().to_string_lossy().to_string(),
        })
    }
}

fn parse_markdown_with_meta(content: &str) -> Option<(BlogMeta, String)> {
    let mut lines = content.lines();
    if let Some(first_line) = lines.next() {
        if first_line.trim() == "---" {
            let mut meta_lines = Vec::new();
            for line in &mut lines {
                if line.trim() == "---" {
                    break;
                }
                meta_lines.push(line);
            }
            let meta_str = meta_lines.join("\n");
            let meta: BlogMeta = serde_yaml::from_str(&meta_str).ok()?;
            let body = lines.collect::<Vec<_>>().join("\n");
            return Some((meta, body));
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{}_{}.md", prefix, nanos))
    }

    #[test]
    fn parse_markdown_with_meta_success() {
        let content = r#"---
title: \"Test Post\"
topics:
  - rust
published: true
published_at: \"2024-01-01\"
tldr: \"Summary\"
---

# Heading
Body text."#;
        let (meta, body) = parse_markdown_with_meta(content).expect("should parse");
        assert_eq!(meta.title, "Test Post");
        assert_eq!(meta.topics, vec!["rust"]);
        assert!(meta.published);
        assert_eq!(meta.published_at, "2024-01-01");
        assert_eq!(meta.tldr.as_deref(), Some("Summary"));
        assert!(body.contains("Heading"));
    }

    #[test]
    fn parse_markdown_with_meta_none() {
        assert!(parse_markdown_with_meta("no frontmatter").is_none());
    }

    #[test]
    fn from_markdown_file_unpublished() {
        let content = r#"---
title: \"Private\"
topics: []
published: false
published_at: \"2024-01-01\"
---

Content"#;
        let path = temp_path("unpublished");
        fs::write(&path, content).expect("write temp file");
        let post = Post::from_markdown_file(&path);
        fs::remove_file(&path).ok();
        assert!(post.is_none());
    }

    #[test]
    fn from_markdown_file_success() {
        let content = r#"---
title: \"Public\"
topics: [\"rust\"]
published: true
published_at: \"2024-01-02\"
---

# Hello
World"#;
        let path = temp_path("published");
        fs::write(&path, content).expect("write temp file");
        let post = Post::from_markdown_file(&path).expect("should parse post");
        fs::remove_file(&path).ok();
        assert_eq!(post.meta.title, "Public");
        assert!(post.html_body.contains("<h1>Hello</h1>"));
    }
}
