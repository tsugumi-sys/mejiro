use chrono::Datelike;

pub fn footer_html(site_title: &str) -> String {
    let current_year = chrono::Utc::now().year();
    format!(
        r#"
<footer>
    <p>&copy; {current_year} {site_title}</p>
</footer>
"#
    )
}
