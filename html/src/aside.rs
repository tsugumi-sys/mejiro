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
    <img src="{}" alt="Logo">
    <span>{}</span>
  </div>
  <nav class="links">
    <a href="/">Home</a>
    <a href="{}">GitHub</a>
    <a href="{}">LinkedIn</a>
  </nav>
</aside>
"#,
        icon_path, owner_name, owner_github_link, owner_linkedin_link
    )
}
