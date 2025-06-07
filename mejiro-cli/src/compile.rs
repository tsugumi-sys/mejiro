use config::MejiroConfig;
use html;
use html::metadata::Post;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn compile(input_dir: &str, output_dir: &str, config_path: &str) {
    fs::create_dir_all(output_dir).unwrap();

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

    // Build post pages.
    // NOTE: post html are created under the posts sub directory. so we need to specify relateive path.
    let relative_icon_path = &format!("../{}", icon_file_name);
    let aside = html::aside_html(
        &config.owner.name,
        &config.owner.github_link,
        &config.owner.linkedin_link,
        &relative_icon_path,
    );
    let footer = html::footer_html(&config.site_title);
    let icon = html::icon_html(&relative_icon_path);

    // Collect published posts
    let posts: Vec<Post> = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map(|e| e == "md").unwrap_or(false))
        .filter_map(|entry| Post::from_markdown_file(entry.path()))
        .collect();

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
        &icon_file_name,
    );
    let icon = html::icon_html(icon_file_name);
    let index_html_content = html::index_html(&posts, &aside, &footer, &icon);

    let index_path = Path::new(output_dir).join("index.html");
    fs::write(index_path, index_html_content).unwrap();
    println!("✅ Generated index.html with {} posts", posts.len());
}
