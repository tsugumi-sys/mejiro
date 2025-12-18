use std::fs;
use std::path::Path;

/// Infers the language identifier from a file path based on its extension.
fn infer_language_from_path(path: &str) -> &str {
    if let Some(ext) = Path::new(path).extension() {
        match ext.to_str().unwrap_or("") {
            "rs" => "rust",
            "py" => "python",
            "js" => "javascript",
            "ts" => "typescript",
            "tsx" => "typescript",
            "jsx" => "javascript",
            "java" => "java",
            "c" => "c",
            "cpp" | "cc" | "cxx" | "hpp" | "h" => "cpp",
            "go" => "go",
            "rb" => "ruby",
            "sh" | "bash" => "bash",
            "yaml" | "yml" => "yaml",
            "json" => "json",
            "html" => "html",
            "css" => "css",
            "scss" | "sass" => "scss",
            "md" => "markdown",
            "sql" => "sql",
            "xml" => "xml",
            "toml" => "toml",
            "r" => "r",
            "php" => "php",
            "swift" => "swift",
            "kt" | "kts" => "kotlin",
            "scala" => "scala",
            _ => "",
        }
    } else {
        ""
    }
}

/// Preprocesses markdown content to replace @code[filepath] directives with actual code blocks.
///
/// Searches for @code[...] patterns and replaces them with markdown code blocks containing
/// the content of the referenced files. The file paths are resolved relative to the
/// directory containing the markdown file.
///
/// If a file is not found, the @code[...] directive is left as-is in the output.
///
/// # Arguments
/// * `markdown` - The markdown content to preprocess
/// * `base_dir` - The directory containing the markdown file (used to resolve relative paths)
///
/// # Returns
/// The preprocessed markdown with @code[...] directives replaced by code blocks (or left as-is if files don't exist)
///
/// # Example
/// ```ignore
/// // In markdown: @code[20250608-8LVpG/main.py]
/// // Gets replaced with:
/// // ```python
/// // <content of main.py>
/// // ```
/// ```
pub fn preprocess_code_includes(markdown: &str, base_dir: &Path) -> String {
    let mut result = String::new();
    let mut last_pos = 0;

    while let Some(start) = markdown[last_pos..].find("@code[") {
        let start_pos = last_pos + start;

        // Append everything before @code[
        result.push_str(&markdown[last_pos..start_pos]);

        // Find the closing ]
        if let Some(end) = markdown[start_pos + 6..].find(']') {
            let end_pos = start_pos + 6 + end;
            let file_path = &markdown[start_pos + 6..end_pos];

            // Resolve the file path relative to the markdown file's directory
            let full_path = base_dir.join(file_path);

            // Try to read the file content
            match fs::read_to_string(&full_path) {
                Ok(code_content) => {
                    // Infer language from extension
                    let lang = infer_language_from_path(file_path);

                    // Create markdown code block
                    let code_block = format!("```{}\n{}\n```", lang, code_content);
                    result.push_str(&code_block);
                }
                Err(_) => {
                    // File not found - leave the @code[...] directive as-is
                    result.push_str(&markdown[start_pos..end_pos + 1]);
                }
            }

            last_pos = end_pos + 1;
        } else {
            // No closing bracket found, just append the @code[ and continue
            result.push_str("@code[");
            last_pos = start_pos + 6;
        }
    }

    // Append the remaining content
    result.push_str(&markdown[last_pos..]);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_infer_language_from_path() {
        assert_eq!(infer_language_from_path("main.py"), "python");
        assert_eq!(infer_language_from_path("app.rs"), "rust");
        assert_eq!(infer_language_from_path("index.js"), "javascript");
        assert_eq!(infer_language_from_path("App.tsx"), "typescript");
        assert_eq!(infer_language_from_path("test.go"), "go");
        assert_eq!(infer_language_from_path("unknown.xyz"), "");
    }

    #[test]
    fn test_preprocess_code_includes() {
        let temp_dir = TempDir::new().unwrap();
        let code_file = temp_dir.path().join("test.py");

        let mut file = fs::File::create(&code_file).unwrap();
        writeln!(file, "def hello():").unwrap();
        writeln!(file, "    print('Hello, World!')").unwrap();

        let markdown = "# Test\n\n@code[test.py]\n\nSome text";
        let result = preprocess_code_includes(markdown, temp_dir.path());

        assert!(result.contains("```python"));
        assert!(result.contains("def hello():"));
        assert!(result.contains("print('Hello, World!')"));
        assert!(result.contains("Some text"));
    }

    #[test]
    fn test_preprocess_code_includes_missing_file() {
        let temp_dir = TempDir::new().unwrap();
        let markdown = "@code[nonexistent.py]";
        let result = preprocess_code_includes(markdown, temp_dir.path());

        // When file doesn't exist, the directive should be left as-is
        assert_eq!(result, "@code[nonexistent.py]");
    }

    #[test]
    fn test_preprocess_no_code_includes() {
        let temp_dir = TempDir::new().unwrap();
        let markdown = "# Test\n\nJust regular markdown content.";
        let result = preprocess_code_includes(markdown, temp_dir.path());

        assert_eq!(result, markdown);
    }
}
