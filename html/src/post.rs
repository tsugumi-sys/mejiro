use crate::metadata::Post;

pub fn post_html(
    post: &Post,
    site_title: &str,
    aside_html: &str,
    footer_html: &str,
    icon_html: &str,
    css_file_path: &str,
) -> String {
    // Generate the header for the post
    let header_html = format!(
        r#"<header>
  <h1>{title}</h1>
  <div class="post-meta">
    <span class="published-at">{published_at}</span>
    {summary}
  </div>
</header>"#,
        title = post.meta.title,
        published_at = post.meta.published_at,
        summary = if let Some(tldr) = &post.meta.tldr {
            format!(r#"<p class="summary">{tldr}</p>"#)
        } else {
            String::new()
        }
    );

    // Final HTML assembly
    let og_description = post.meta.tldr.as_deref().unwrap_or(site_title);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{title}</title>
  <meta property="og:title" content="{og_title}" />
  <meta property="og:description" content="{og_description}" />
  <meta property="og:type" content="article" />
  <meta property="og:site_name" content="{site_title}" />
  <link rel="stylesheet" href="{css_file_path}">
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/themes/prism-tomorrow.min.css">
  <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-core.min.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js"></script>
  <script>
    // Load languages on demand from the CDN
    if (window.Prism && Prism.plugins.autoloader) {{
      Prism.plugins.autoloader.languages_path = "https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/";
    }}
  </script>
  {icon_html}
</head>
<body>
  <div class="container">
    {aside_html}
    <main>
      {header_html}
      <article>
        {body}
      </article>
    </main>
  </div>
  {footer_html}
  <script>Prism.highlightAll();</script>
</body>
</html>
"#,
        title = post.meta.title,
        og_title = post.meta.title,
        og_description = og_description,
        site_title = site_title,
        css_file_path = css_file_path,
        icon_html = icon_html,
        aside_html = aside_html,
        header_html = header_html,
        body = post.html_body,
        footer_html = footer_html
    )
}
