use chrono::Datelike;

pub fn footer_html(site_title: &str) -> String {
    let current_year = chrono::Utc::now().year();
    format!(
        r#"
<footer>
    <p>&copy; {} {}</p>
</footer>
"#,
        current_year, site_title
    )
}
