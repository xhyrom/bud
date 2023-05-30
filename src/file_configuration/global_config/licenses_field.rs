use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Licenses {
    #[serde(default = "default_values::default")]
    pub default: String,
    #[serde(default = "default_values::copyright_holder")]
    pub copyright_holder: String,
    #[serde(default = "default_values::custom")]
    pub custom: HashMap<String, Custom>,

    #[serde(skip)]
    github_licenses: HashMap<String, String>,
}

struct GithubLicense {
    pub key: String,
    pub name: String,
}

impl Licenses {
    pub fn get(&self, name: &str) -> Option<&Custom> {
        self.custom.get(name)
    }

    pub fn github_licenses() {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Custom {
    pub name: String,
    pub path: String,
}

pub mod default_values {
    use std::collections::HashMap;

    pub fn licenses() -> super::Licenses {
        super::Licenses {
            default: default(),
            copyright_holder: copyright_holder(),
            custom: custom(),
            github_licenses: HashMap::new(),
        }
    }

    pub fn default() -> String {
        String::from("MIT")
    }

    pub fn copyright_holder() -> String {
        String::from("Your Name")
    }

    pub fn custom() -> HashMap<String, super::Custom> {
        HashMap::new()
    }
}
