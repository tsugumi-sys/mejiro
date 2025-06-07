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
  <h1>{}</h1>
  <div class="post-meta">
    <span class="published-at">{}</span>
    {}
  </div>
</header>"#,
        post.meta.title,
        post.meta.published_at,
        if let Some(tldr) = &post.meta.tldr {
            format!(r#"<p class="summary">{}</p>"#, tldr)
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
  <title>{}</title>
  <link rel="stylesheet" href="{}">
  {}
</head>
<body>
  <div class="container">
    {}
    <main>
      {}
      <article>
        {}
      </article>
    </main>
  </div>
  {}
</body>
</html>
"#,
        post.meta.title,
        css_file_path,
        icon_html,
        aside_html,
        header_html,
        post.html_body,
        footer_html
    )
}
