use html::metadata::BlogMeta;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn parse_meta(path: &Path) -> Option<BlogMeta> {
    let content = fs::read_to_string(path).ok()?;
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
            return Some(meta);
        }
    }
    None
}

pub fn list(input_dir: &str, all: bool) {
    let posts: Vec<(String, BlogMeta)> = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map(|e| e == "md").unwrap_or(false))
        .filter_map(|entry| {
            let name = entry.path().file_stem()?.to_string_lossy().to_string();
            let meta = parse_meta(entry.path())?;
            Some((name, meta))
        })
        .collect();

    for (name, meta) in posts {
        if !all && !meta.published {
            continue;
        }
        println!("---");
        println!("name: {}", name);
        let yaml = serde_yaml::to_string(&meta).unwrap_or_default();
        print!("{}", yaml);
    }
}

