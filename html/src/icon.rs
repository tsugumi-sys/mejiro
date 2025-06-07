use std::path::Path;

pub fn icon_html(icon_path: &str) -> String {
    let icon_type = Path::new(icon_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| match ext {
            "png" => "image/png",
            "ico" => "image/x-icon",
            "svg" => "image/svg+xml",
            "jpg" | "jpeg" => "image/jpeg",
            _ => "image/png", // Default fallback
        })
        .unwrap_or("image/png");

    format!(
        r#"<link rel="icon" href="/{}" type="{}">"#,
        icon_path, icon_type
    )
}
