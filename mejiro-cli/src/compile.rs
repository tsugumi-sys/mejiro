use crate::posts_json::generate_posts_json;
use config::{GiscusConfig, MejiroConfig};
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
                let path = entry.path().display();
                println!("Skipping unpublished post: {path}");
            }
            Err(e) => {
                let path = entry.path().display();
                eprintln!("Error parsing {path}: {e}");
            }
        }
    }
    posts.sort_by(|a, b| b.meta.published_at.cmp(&a.meta.published_at));

    // Build post pages
    let icon_path_rel = format!("../{icon_file_name}");
    let aside = html::aside_html(
        &config.owner.name,
        &config.owner.github_link,
        &config.owner.linkedin_link,
        &icon_path_rel,
    );
    let icon = html::icon_html(&icon_path_rel);
    let post_page_context = PostPageContext {
        output_dir: Path::new(output_dir),
        site_title: &config.site_title,
        aside: &aside,
        footer: &footer,
        icon: &icon,
        css_filename: &css_filename,
        giscus: config.giscus.as_ref(),
    };
    build_post_pages(&posts, &post_page_context);

    // Build index.html
    build_index_page(
        Path::new(output_dir),
        &posts,
        &config,
        &css_filename,
        icon_file_name,
        &footer,
    );

    // Generate posts.json
    generate_posts_json(&posts, output_dir);

    let post_paths: Vec<String> = posts
        .iter()
        .map(|post| format!("{name}.html", name = post.name))
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
            println!("│   ├── {path}");
        }
        let omitted = post_paths.len() - 6;
        println!("│   ├── ... ({omitted} posts omitted)");
        for path in &post_paths[post_paths.len() - 3..] {
            println!("│   ├── {path}");
        }
    } else {
        for path in &post_paths {
            println!("│   ├── {path}");
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
        let hash_hex = format!("{hash:x}");

        Some(format!("style.{hash}.css", hash = &hash_hex[..8]))
    } else {
        eprintln!("CSS file not found: {css_path:?}");
        None
    }
}

fn copy_file(src: &Path, dest: &Path, description: &str) -> std::io::Result<()> {
    if src.exists() {
        fs::copy(src, dest)?;
    } else {
        eprintln!("{description} not found: {src:?}");
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
        eprintln!("Images directory not found: {src_dir:?}");
    }
}

struct PostPageContext<'a> {
    output_dir: &'a Path,
    site_title: &'a str,
    aside: &'a str,
    footer: &'a str,
    icon: &'a str,
    css_filename: &'a str,
    giscus: Option<&'a GiscusConfig>,
}

fn build_post_pages(posts: &[Post], context: &PostPageContext<'_>) {
    for post in posts {
        let output_path = context
            .output_dir
            .join("posts")
            .join(format!("{name}.html", name = post.name));
        fs::create_dir_all(output_path.parent().unwrap()).unwrap();

        let css_relative_path = format!("../{css_filename}", css_filename = context.css_filename);
        let comments_html = context.giscus.map(giscus_html).unwrap_or_default();
        let post_html = html::post_html(
            post,
            context.site_title,
            context.aside,
            context.footer,
            context.icon,
            &css_relative_path,
            &comments_html,
        );
        fs::write(&output_path, post_html).unwrap();
    }
}

fn giscus_html(config: &GiscusConfig) -> String {
    format!(
        r#"<section class="comments">
  <script src="https://giscus.app/client.js"
    data-repo="{repo}"
    data-repo-id="{repo_id}"
    data-category="{category}"
    data-category-id="{category_id}"
    data-mapping="{mapping}"
    data-strict="{strict}"
    data-reactions-enabled="{reactions_enabled}"
    data-emit-metadata="{emit_metadata}"
    data-input-position="{input_position}"
    data-theme="{theme}"
    data-lang="{lang}"
    data-loading="{loading}"
    crossorigin="anonymous"
    async>
  </script>
</section>"#,
        repo = html_attr(&config.repo),
        repo_id = html_attr(&config.repo_id),
        category = html_attr(&config.category),
        category_id = html_attr(&config.category_id),
        mapping = html_attr(&config.mapping),
        strict = bool_attr(config.strict),
        reactions_enabled = bool_attr(config.reactions_enabled),
        emit_metadata = bool_attr(config.emit_metadata),
        input_position = html_attr(&config.input_position),
        theme = html_attr(&config.theme),
        lang = html_attr(&config.lang),
        loading = html_attr(&config.loading),
    )
}

fn bool_attr(value: bool) -> &'static str {
    if value { "1" } else { "0" }
}

fn html_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn build_index_page(
    output_dir: &Path,
    posts: &[Post],
    config: &MejiroConfig,
    css_filename: &str,
    icon_filename: &str,
    footer: &str,
) {
    let aside = html::aside_html(
        &config.owner.name,
        &config.owner.github_link,
        &config.owner.linkedin_link,
        icon_filename,
    );
    let icon = html::icon_html(icon_filename);

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
