use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GiscusConfig {
    pub repo: String,
    pub repo_id: String,
    pub category: String,
    pub category_id: String,
    #[serde(default = "default_mapping")]
    pub mapping: String,
    #[serde(default)]
    pub strict: bool,
    #[serde(default = "default_reactions_enabled")]
    pub reactions_enabled: bool,
    #[serde(default)]
    pub emit_metadata: bool,
    #[serde(default = "default_input_position")]
    pub input_position: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_lang")]
    pub lang: String,
    #[serde(default = "default_loading")]
    pub loading: String,
}

fn default_mapping() -> String {
    "pathname".to_string()
}

fn default_reactions_enabled() -> bool {
    true
}

fn default_input_position() -> String {
    "bottom".to_string()
}

fn default_theme() -> String {
    "preferred_color_scheme".to_string()
}

fn default_lang() -> String {
    "en".to_string()
}

fn default_loading() -> String {
    "lazy".to_string()
}
