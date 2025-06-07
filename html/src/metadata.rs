use pulldown_cmark::{Parser, html};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Possible errors when parsing a blog post from a Markdown file.
#[derive(Debug)]
pub enum BlogParseError {
    MetadataNotFound,
    YamlParseError(String),
    IoError(String),
}

impl std::fmt::Display for BlogParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlogParseError::MetadataNotFound => {
                write!(f, "The markdown file is missing a metadata (YAML) header.")
            }
            BlogParseError::YamlParseError(msg) => write!(f, "Failed to parse metadata: {}", msg),
            BlogParseError::IoError(msg) => write!(f, "File error: {}", msg),
        }
    }
}

impl std::error::Error for BlogParseError {}

/// Metadata extracted from the YAML front matter of a blog post.
#[derive(Serialize, Deserialize, Debug)]
pub struct BlogMeta {
    pub title: String,
    pub topics: Vec<String>,
    pub published: bool,
    pub published_at: String,
    pub tldr: Option<String>,
}

impl BlogMeta {
    pub fn from_markdown_str(content: &str) -> Result<(Self, String), BlogParseError> {
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

                let meta: BlogMeta = serde_yaml::from_str(&meta_str).map_err(|e| {
                    BlogParseError::YamlParseError(
                        e.to_string()
                            .lines()
                            .next()
                            .unwrap_or("unknown error")
                            .to_string(),
                    )
                })?;

                let body = lines.collect::<Vec<_>>().join("\n");
                return Ok((meta, body));
            }
        }

        Err(BlogParseError::MetadataNotFound)
    }
}

/// Represents a parsed blog post.
pub struct Post {
    pub meta: BlogMeta,
    pub html_body: String,
    pub name: String, // file name without extension
}

impl Post {
    /// Parses a Markdown file into a Post.
    /// - Returns `Err(BlogParseError)` if the file cannot be read or metadata is invalid.
    /// - Returns `Ok(None)` if the post is not published.
    /// - Returns `Ok(Some(Post))` if successfully parsed and published.
    pub fn from_markdown_file(path: &Path) -> Result<Option<Self>, BlogParseError> {
        let content =
            fs::read_to_string(path).map_err(|e| BlogParseError::IoError(e.to_string()))?;

        let (meta, body_md) = BlogMeta::from_markdown_str(&content)?;

        if !meta.published {
            return Ok(None);
        }

        let parser = Parser::new(&body_md);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Ok(Some(Post {
            meta,
            html_body: html_output,
            name: path.file_stem().unwrap().to_string_lossy().to_string(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_markdown_str_invalid_yaml() {
        let markdown = r#"---
title: "Test Post"
topics: ["rust", "blog"
published: true
published_at: "2025-06-07"
---

This is the blog post content.
"#;

        let result = BlogMeta::from_markdown_str(markdown);

        match result {
            Err(BlogParseError::YamlParseError(msg)) => {
                assert!(
                    msg.contains("while parsing a flow sequence")
                        || msg.contains("did not find expected ',' or ']'")
                        || msg.contains("expected ',' or ']'"),
                    "Unexpected error message: {}",
                    msg
                );
            }
            _ => panic!("Expected YamlParseError, got {:?}", result),
        }
    }

    #[test]
    fn test_from_markdown_str_missing_metadata() {
        let markdown = r#"
# No Front Matter Here

This post does not have any metadata.
"#;

        let result = BlogMeta::from_markdown_str(markdown);
        assert!(matches!(result, Err(BlogParseError::MetadataNotFound)));
    }
}
