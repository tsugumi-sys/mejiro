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

impl BlogMeta {
    /// Parses the given markdown content, extracting front matter (YAML) and body.
    /// Returns (BlogMeta, markdown_body) on success.
    pub fn from_markdown_str(content: &str) -> Option<(Self, String)> {
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
}

pub struct Post {
    pub meta: BlogMeta,
    pub html_body: String,
    pub name: String, // file name (without extension)
}

impl Post {
    pub fn from_markdown_file(path: &Path) -> Option<Self> {
        let content = fs::read_to_string(path).ok()?;
        let (meta, body_md) = BlogMeta::from_markdown_str(&content)?;
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
