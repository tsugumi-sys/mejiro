use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BlogOwner {
    pub name: String,
    pub github_link: String,
    pub linkedin_link: String,
}

impl BlogOwner {
    pub fn new(name: &str, github_link: &str, linkedin_link: &str) -> Self {
        Self {
            name: name.to_string(),
            github_link: github_link.to_string(),
            linkedin_link: linkedin_link.to_string(),
        }
    }
}
