use crate::posts_json::generate_posts_json;
use config::MejiroConfig;
use html;
use html::metadata::Post;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn compile(input_dir: &str, output_dir: &str, config_path: &str) {
    fs::create_dir_all(output_dir).unwrap();

    let config = MejiroConfig::load_config(config_path);
    let css_path = Path::new(&config.styles.css_file);

    // Generate hashed CSS filename
    let css_filename =
        css_filename_with_hash(css_path).expect("Could not generate CSS filename with hash");

    // Copy CSS
    let dest_css = Path::new(output_dir).join(&css_filename);
    copy_file(css_path, &dest_css, "CSS").unwrap();

    // Copy icon
    let icon_path = Path::new(&config.styles.icon);
    let icon_file_name = icon_path.file_name().unwrap_or_default().to_str().unwrap();
    let dest_icon = Path::new(output_dir).join(icon_file_name);
    copy_file(icon_path, &dest_icon, "icon").unwrap();

    // Copy images
    let src_images = Path::new(&config.images_dir);
    let dest_images = Path::new(output_dir).join("images");
    copy_images(src_images, &dest_images);

    // Write mejiro-search-pkg
    write_search_pkg(output_dir);

    // Build footer
    let footer = html::footer_html(&config.site_title);

    // Collect published posts
    let mut posts = Vec::new();
    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
    {
        match Post::from_markdown_file(entry.path()) {
            Ok(Some(post)) => posts.push(post),
            Ok(None) => {
                println!("Skipping unpublished post: {}", entry.path().display());
            }
            Err(e) => {
                eprintln!("Error parsing {}: {}", entry.path().display(), e);
            }
        }
    }
    posts.sort_by(|a, b| b.meta.published_at.cmp(&a.meta.published_at));

    // Build post pages
    let icon_path_rel = format!("../{}", icon_file_name);
    let aside = html::aside_html(
        &config.owner.name,
        &config.owner.github_link,
        &config.owner.linkedin_link,
        &icon_path_rel,
    );
    let icon = html::icon_html(&icon_path_rel);
    build_post_pages(
        &posts,
        Path::new(output_dir),
        &aside,
        &footer,
        &icon,
        &css_filename,
    );

    // Build index.html
    build_index_page(
        Path::new(output_dir),
        &posts,
        &config,
        &css_filename,
        &footer,
    );

    // Generate posts.json
    generate_posts_json(&posts, output_dir);

    let post_paths: Vec<String> = posts
        .iter()
        .map(|post| format!("{}.html", post.name))
        .collect();

    println!("\nOutput directory structure:");
    println!("public/");
    println!("├── {css_filename}");
    println!("├── icon.png");
    println!("├── images/");
    println!("├── mejiro-search-pkg/");
    println!("├── index.html");
    println!("├── posts/");

    // Show top/bottom posts only if many
    if post_paths.len() > 10 {
        for path in &post_paths[..3] {
            println!("│   ├── {}", path);
        }
        println!("│   ├── ... ({} posts omitted)", post_paths.len() - 6);
        for path in &post_paths[post_paths.len() - 3..] {
            println!("│   ├── {}", path);
        }
    } else {
        for path in &post_paths {
            println!("│   ├── {}", path);
        }
    }

    println!("✅ Build complete. Output directory: ./public");
}

fn css_filename_with_hash(css_path: &Path) -> Option<String> {
    if css_path.exists() {
        let bytes = fs::read(css_path).ok()?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash = hasher.finalize();
        let hash_hex = format!("{:x}", hash);

        Some(format!("style.{}.css", &hash_hex[..8]))
    } else {
        eprintln!("CSS file not found: {:?}", css_path);
        None
    }
}

fn copy_file(src: &Path, dest: &Path, description: &str) -> std::io::Result<()> {
    if src.exists() {
        fs::copy(src, dest)?;
    } else {
        eprintln!("{} not found: {:?}", description, src);
    }
    Ok(())
}

fn copy_images(src_dir: &Path, dest_dir: &Path) {
    if src_dir.exists() {
        for entry in WalkDir::new(src_dir).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                let rel = entry.path().strip_prefix(src_dir).unwrap();
                let dest = dest_dir.join(rel);
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                fs::copy(entry.path(), &dest).unwrap();
            }
        }
    } else {
        eprintln!("Images directory not found: {:?}", src_dir);
    }
}

fn build_post_pages(
    posts: &[Post],
    output_dir: &Path,
    aside: &str,
    footer: &str,
    icon: &str,
    css_filename: &str,
) {
    for post in posts {
        let output_path = output_dir.join("posts").join(format!("{}.html", post.name));
        fs::create_dir_all(output_path.parent().unwrap()).unwrap();

        let css_relative_path = format!("../{}", css_filename);
        let post_html = html::post_html(post, aside, footer, icon, &css_relative_path);
        fs::write(&output_path, post_html).unwrap();
    }
}

fn build_index_page(
    output_dir: &Path,
    posts: &[Post],
    config: &MejiroConfig,
    css_filename: &str,
    footer: &str,
) {
    let icon_file = &config.styles.icon;
    let aside = html::aside_html(
        &config.owner.name,
        &config.owner.github_link,
        &config.owner.linkedin_link,
        icon_file,
    );
    let icon = html::icon_html(icon_file);

    let index_html = html::index_html(
        &config.owner.name,
        posts,
        &aside,
        footer,
        &icon,
        css_filename,
    );
    let index_path = output_dir.join("index.html");
    fs::write(&index_path, index_html).unwrap();
}

fn write_search_pkg(output_dir: &str) {
    let pkg_dir = Path::new(output_dir).join("mejiro-search-pkg");
    fs::create_dir_all(&pkg_dir).unwrap();

    let dest_js = pkg_dir.join("mejiro_search.js");
    fs::write(&dest_js, mejiro_search_js()).unwrap();

    let dest_wasm = pkg_dir.join("mejiro_search_bg.wasm");
    fs::write(&dest_wasm, mejiro_search_bg_wasm()).unwrap();
}

fn mejiro_search_js() -> &'static str {
    include_str!("../assets/mejiro-search-pkg/mejiro_search.js")
}

fn mejiro_search_bg_wasm() -> &'static [u8] {
    include_bytes!("../assets/mejiro-search-pkg/mejiro_search_bg.wasm")
}
