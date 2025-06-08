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
      <div class="search-bar-wrapper">
        <i id="search-trigger" class="fas fa-search"></i>
        <span id="search-wrapper" class="hidden">
          <input type="text" id="search-input" placeholder="Search..." />
          <i id="search-cleaner" class="fa fa-times"></i>
        </span>
      </div>

      <h1>Posts</h1>
      <ul id="post-list">
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
      <ul id="search-results" class="hidden"></ul>
    </main>
  </div>
"#,
    );

    // Add the footer HTML
    index_html.push_str(footer_html);

    // Add the <script> block for search logic
    index_html.push_str(
        r#"
  <script type="module">
    import init, { search } from './mejiro_search.js';

    let postsData = [];

    async function loadPosts() {
      const response = await fetch('/posts.json');
      postsData = await response.json();
    }

    async function initialize() {
      await init();
      await loadPosts();
    }

    function doSearch() {
      const query = searchInput.value.trim();
      const searchResults = document.getElementById('search-results');
      const postList = document.getElementById('post-list');

      if (!query) {
        searchResults.classList.add('hidden');
        postList.classList.remove('hidden');
        return;
      }

      const results = search(postsData, query);
      const plainResults = results.map(item => Object.fromEntries(item));

      if (plainResults.length === 0) {
        searchResults.innerHTML = '<li>No posts found.</li>';
      } else {
        searchResults.innerHTML = '';
        plainResults.forEach(post => {
          const li = document.createElement('li');
          li.innerHTML = `
            <a href="${post.path}"><strong>${post.title}</strong></a>
            <p class="summary">${post.tldr || ''}</p>
            <p class="topics">Tags: ${post.tags.join(', ')}</p>
            <p class="published-at">Published at: ${post.published_at || 'Unknown'}</p>
          `;
          searchResults.appendChild(li);
        });
      }

      searchResults.classList.remove('hidden');
      postList.classList.add('hidden');
    }

    const searchTrigger = document.getElementById('search-trigger');
    const searchWrapper = document.getElementById('search-wrapper');
    const searchInput = document.getElementById('search-input');
    const searchCleaner = document.getElementById('search-cleaner');

    searchTrigger.addEventListener('click', () => {
      searchWrapper.classList.toggle('hidden');
      if (!searchWrapper.classList.contains('hidden')) {
        searchInput.focus();
      }
    });

    searchCleaner.addEventListener('click', () => {
      searchInput.value = '';
      doSearch();
    });

    searchInput.addEventListener('input', doSearch);

    initialize();
  </script>
"#,
    );

    // Close body and html
    index_html.push_str("\n</body>\n</html>\n");

    index_html
}
