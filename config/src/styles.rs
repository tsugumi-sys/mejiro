use serde::Deserialize;

#[derive(Deserialize)]
pub struct BlogStyles {
    theme: String,
    pub icon: String,
    pub css_file: String,
}

impl BlogStyles {
    pub fn new(theme: &str, icon: &str, css_file: &str) -> Self {
        Self {
            theme: theme.to_string(),
            icon: icon.to_string(),
            css_file: css_file.to_string(),
        }
    }
}
