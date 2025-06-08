use crate::posts_json::generate_posts_json;
use config::MejiroConfig;
use html;
use html::metadata::Post;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn mejiro_search_js() -> &'static str {
    include_str!("../assets/mejiro-search-pkg/mejiro_search.js")
}

fn mejiro_search_bg_wasm() -> &'static [u8] {
    include_bytes!("../assets/mejiro-search-pkg/mejiro_search_bg.wasm")
}

pub fn compile(input_dir: &str, output_dir: &str, config_path: &str) {
    fs::create_dir_all(output_dir).unwrap();

    // Create the mejiro-search-pkg directory in the output dir
    let pkg_dir = Path::new(output_dir).join("mejiro-search-pkg");
    fs::create_dir_all(&pkg_dir).unwrap_or_else(|e| {
        panic!("❌ Failed to create mejiro-search-pkg directory: {:?}", e);
    });

    // Write mejiro_search.js
    let dest_js = pkg_dir.join("mejiro_search.js");
    fs::write(&dest_js, mejiro_search_js()).unwrap_or_else(|e| {
        panic!("❌ Failed to write mejiro_search.js: {:?}", e);
    });

    // Write mejiro_search_bg.wasm
    let dest_wasm = pkg_dir.join("mejiro_search_bg.wasm");
    fs::write(&dest_wasm, mejiro_search_bg_wasm()).unwrap_or_else(|e| {
        panic!("❌ Failed to write mejiro_search_bg.wasm: {:?}", e);
    });

    println!("✅ Wrote mejiro-search-pkg directory and files");

    let config = MejiroConfig::load_config(config_path);

    let css_path = Path::new(&config.styles.css_file);
    if css_path.exists() {
        let dest_path = Path::new(output_dir).join(css_path.file_name().unwrap());
        fs::copy(css_path, dest_path).unwrap();
        println!("✅ Copied CSS file to output directory");
    } else {
        eprintln!("❌ CSS file not found: {:?}", css_path);
    }

    let icon_path = Path::new(&config.styles.icon);
    let icon_file_name: &str = icon_path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("icon.png");

    if icon_path.exists() {
        let dest_path = Path::new(output_dir).join(icon_file_name);
        fs::copy(icon_path, dest_path).unwrap();
        println!("✅ Copied icon file to output directory");
    } else {
        eprintln!("❌ icon file not found: {:?}", icon_path);
    }

    // Copy images directory
    let src_images = Path::new(&config.images_dir);
    let dest_images = Path::new(output_dir).join("images");
    if src_images.exists() {
        for entry in WalkDir::new(src_images).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                let rel = entry.path().strip_prefix(src_images).unwrap();
                let dest = dest_images.join(rel);
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                fs::copy(entry.path(), &dest).unwrap();
            }
        }
        println!("✅ Copied images to output directory");
    } else {
        eprintln!("❌ images directory not found: {:?}", src_images);
    }

    // Build post pages.
    let icon_path = format!("../{}", icon_file_name);
    let aside = html::aside_html(
        &config.owner.name,
        &config.owner.github_link,
        &config.owner.linkedin_link,
        &icon_path,
    );
    let footer = html::footer_html(&config.site_title);
    let icon = html::icon_html(&icon_path);

    // Collect published posts, logging errors explicitly
    let mut posts: Vec<Post> = Vec::new();
    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map(|e| e == "md").unwrap_or(false))
    {
        match Post::from_markdown_file(entry.path()) {
            Ok(Some(post)) => posts.push(post),
            Ok(None) => {
                println!("ℹ️ Skipping unpublished post: {}", entry.path().display());
            }
            Err(e) => {
                eprintln!("❌ Error parsing {}: {}", entry.path().display(), e);
            }
        }
    }
    posts.sort_by(|a, b| b.meta.published_at.cmp(&a.meta.published_at));

    // Build each post page
    for post in &posts {
        let output_path = Path::new(output_dir)
            .join("posts")
            .join(format!("{}.html", post.name));
        fs::create_dir_all(output_path.parent().unwrap()).unwrap();

        let css_relative_path = format!("../{}", css_path.file_name().unwrap().to_string_lossy());
        let post_html_content = html::post_html(post, &aside, &footer, &icon, &css_relative_path);
        fs::write(output_path, post_html_content).unwrap();
        println!("✅ Built post: {}.html", post.name);
    }

    // Build index
    let aside = html::aside_html(
        &config.owner.name,
        &config.owner.github_link,
        &config.owner.linkedin_link,
        icon_file_name,
    );
    let icon = html::icon_html(icon_file_name);
    let index_html_content = html::index_html(&posts, &aside, &footer, &icon);

    let index_path = Path::new(output_dir).join("index.html");
    fs::write(index_path, index_html_content).unwrap();
    println!("✅ Generated index.html with {} posts", posts.len());

    generate_posts_json(&posts, output_dir);
}
