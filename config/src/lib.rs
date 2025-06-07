mod owner;
mod styles;

use owner::BlogOwner;
use serde::Deserialize;
use styles::BlogStyles;

#[derive(Deserialize)]
pub struct MejiroConfig {
    pub owner: BlogOwner,
    pub site_title: String,
    pub styles: BlogStyles,
}

impl MejiroConfig {
    /// Loads the blog configuration from a YAML file
    pub fn load_config(config_path: &str) -> Self {
        let contents =
            std::fs::read_to_string(config_path).expect("Could not read the config file");

        // Parse the YAML into a Config struct
        let config: MejiroConfig =
            serde_yaml::from_str(&contents).expect("Failed to parse YAML file into Config");

        config
    }
}
