use crate::metadata::Post;

pub fn post_html(
    post: &Post,
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
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{title}</title>
  <link rel="stylesheet" href="{css_file_path}">
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
</body>
</html>
"#,
        title = post.meta.title,
        css_file_path = css_file_path,
        icon_html = icon_html,
        aside_html = aside_html,
        header_html = header_html,
        body = post.html_body,
        footer_html = footer_html
    )
}
