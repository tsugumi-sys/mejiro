use crate::metadata::Post;

pub fn index_html(posts: &[Post], aside_html: &str, footer_html: &str, icon_html: &str) -> String {
    // Start the page with the container, aside_html, and main
    let mut index_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Akira Noda Blog</title>
  <link rel="stylesheet" href="style.css">
  {}
</head>
<body>
  <div class="container">
    {}
    <main>
      <h1>Recent Posts</h1>
      <ul>
"#,
        icon_html, aside_html
    );

    // Loop through the posts and build the list
    for post in posts {
        let summary_html = post.meta.tldr.as_ref().map_or(String::new(), |s| {
            format!(r#"<p class="summary">{}</p>"#, s)
        });

        let topics_html = if !post.meta.topics.is_empty() {
            let topics = post.meta.topics.join(", ");
            format!(r#"<p class="topics">Tags: {}</p>"#, topics)
        } else {
            String::new()
        };

        let date_html = format!(
            r#"<p class="published-at">Published at: {}</p>"#,
            post.meta.published_at
        );

        index_html.push_str(&format!(
            r#"        <li>
          <a href="posts/{}.html"><strong>{}</strong></a>
          {}
          {}
          {}
        </li>
"#,
            post.name, post.meta.title, summary_html, topics_html, date_html
        ));
    }

    // Close main and container
    index_html.push_str(
        r#"      </ul>
    </main>
  </div>
"#,
    );
    index_html.push_str(footer_html);
    index_html.push_str("\n</body>\n</html>\n");

    index_html
}
