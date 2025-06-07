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

    /// Initialize default configuration, posts directory, and assets
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

        let yaml_str =
            serde_yaml::to_string(&default_config).expect("Failed to serialize default config");

        let mut file = fs::File::create(config_path).expect("Failed to create configuration file");
        file.write_all(yaml_str.as_bytes())
            .expect("Failed to write default config");
        println!("✅ Created default configuration: {}", config_path);

        // Create posts directory if needed
        if !Path::new(posts_dir).exists() {
            fs::create_dir_all(posts_dir).expect("Failed to create posts directory");
            println!("✅ Created posts directory: {}", posts_dir);
        } else {
            println!("✅ Posts directory already exists: {}", posts_dir);
        }

        // Create images directory if needed
        if !Path::new(&default_config.images_dir).exists() {
            fs::create_dir_all(&default_config.images_dir)
                .expect("Failed to create images directory");
            println!("✅ Created images directory: {}", default_config.images_dir);
        } else {
            println!(
                "✅ Images directory already exists: {}",
                default_config.images_dir
            );
        }

        // Write embedded default CSS to style.css
        let mut style_file = fs::File::create("style.css").expect("Failed to create style.css");
        style_file
            .write_all(Self::default_css().as_bytes())
            .expect("Failed to write style.css");
        println!("✅ Created style.css from embedded asset");

        // Create a tiny icon.png (1x1 transparent PNG)
        let icon_data = BASE64_STANDARD.decode(
            "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR4nGNgYAAAAAMAASsJTYQAAAAASUVORK5CYII=",
        )
        .unwrap();
        let mut icon_file = fs::File::create("icon.png").expect("Failed to create icon.png");
        icon_file
            .write_all(&icon_data)
            .expect("Failed to write icon.png");
        println!("✅ Created default icon.png (1x1 transparent PNG)");
    }
}
