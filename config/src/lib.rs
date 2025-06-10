use base64::prelude::*;
use std::fs;
use std::io::Write;
use std::path::Path;

mod owner;
mod styles;

use owner::BlogOwner;
use serde::{Deserialize, Serialize};
use styles::BlogStyles;

#[derive(Serialize, Deserialize)]
pub struct MejiroConfig {
    pub owner: BlogOwner,
    pub site_title: String,
    pub styles: BlogStyles,
    #[serde(default = "default_images_dir")]
    pub images_dir: String,
}

fn default_images_dir() -> String {
    "images".to_string()
}

impl MejiroConfig {
    /// Load blog configuration from YAML
    pub fn load_config(config_path: &str) -> Self {
        let contents = fs::read_to_string(config_path).unwrap_or_else(|e| {
            panic!("❌ Could not read the config file: {} ({})", config_path, e);
        });

        serde_yaml::from_str(&contents).unwrap_or_else(|e| {
            panic!("❌ Failed to parse YAML file '{}': {}", config_path, e);
        })
    }

    /// Returns the embedded default CSS as a string
    pub fn default_css() -> &'static str {
        include_str!("../../mejiro-cli/assets/style.css")
    }

    pub fn initialize_config(config_path: &str, posts_dir: &str) {
        let default_config = MejiroConfig {
            owner: BlogOwner {
                name: "Your Name".to_string(),
                github_link: "https://github.com/your-profile".to_string(),
                linkedin_link: "https://linkedin.com/in/your-profile".to_string(),
            },
            site_title: "My Blog".to_string(),
            styles: BlogStyles {
                css_file: "style.css".to_string(),
                icon: "icon.png".to_string(),
            },
            images_dir: "images".to_string(),
        };

        Self::write_config_file(config_path, &default_config);
        Self::create_dir_if_not_exists(posts_dir);
        Self::create_dir_if_not_exists(&default_config.images_dir);
        Self::write_default_css("style.css");
        Self::write_default_icon("icon.png");

        // Tree view for user-friendly output
        let entries = vec![
            ("mejiro.yml", "Your blog configuration"),
            ("style.css", "Default blog styling (customize as needed)"),
            ("icon.png", "Default icon (replace with your own icon)"),
            ("posts/", "Your blog posts directory"),
            ("images/", "Directory for images used in your blog"),
        ];

        // Calculate the longest entry name for alignment
        let longest_len = entries
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(0);

        println!("\n🐦 Hello Mejiro! Let’s get your blog started!");
        println!("\n🎉 Your Mejiro Blog structure:");
        for (name, desc) in entries {
            let padding = " ".repeat(longest_len - name.len() + 1);
            println!("├── {}{}# {}", name, padding, desc);
        }
    }

    fn write_config_file(config_path: &str, config: &MejiroConfig) {
        if !Path::new(config_path).exists() {
            let yaml_str =
                serde_yaml::to_string(config).expect("Failed to serialize default config");

            let mut file =
                fs::File::create(config_path).expect("Failed to create configuration file");
            file.write_all(yaml_str.as_bytes())
                .expect("Failed to write default config");
        }
    }

    fn create_dir_if_not_exists(dir: &str) {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).expect("Failed to create directory");
        }
    }

    fn write_default_css(css_path: &str) {
        let mut style_file = fs::File::create(css_path).expect("Failed to create CSS file");
        style_file
            .write_all(Self::default_css().as_bytes())
            .expect("Failed to write CSS file");
    }

    fn write_default_icon(icon_path: &str) {
        let icon_data = BASE64_STANDARD
            .decode("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR4nGNgYAAAAAMAASsJTYQAAAAASUVORK5CYII=")
            .unwrap();

        let mut icon_file = fs::File::create(icon_path).expect("Failed to create icon file");
        icon_file
            .write_all(&icon_data)
            .expect("Failed to write icon file");
    }
}
