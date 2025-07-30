pub fn aside_html(
    owner_name: &str,
    owner_github_link: &str,
    owner_linkedin_link: &str,
    icon_path: &str,
) -> String {
    format!(
        r#"
<aside>
  <div class="logo">
    <img src="{icon_path}" alt="Logo">
    <span>{owner_name}</span>
  </div>
  <nav class="links">
    <a href="/">Home</a>
    <a href="{owner_github_link}">GitHub</a>
    <a href="{owner_linkedin_link}">LinkedIn</a>
  </nav>
</aside>
"#
    )
}
