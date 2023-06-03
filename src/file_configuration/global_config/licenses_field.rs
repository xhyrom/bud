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
    github_licenses: HashMap<String, GithubLicense>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GithubLicense {
    pub key: String,
    pub name: String,
    pub permissions: Option<Vec<String>>,
    pub conditions: Option<Vec<String>>,
    pub limitations: Option<Vec<String>>,
}

impl Licenses {
    pub fn get(&self, name: &str) -> Option<&Custom> {
        self.custom.get(name)
    }

    pub fn github_licenses(&mut self) {
        let body = reqwest::blocking::get(
            "https://raw.githubusercontent.com/xHyroM/bud/main/licenses/licenses.json",
        )
        .unwrap()
        .json::<HashMap<String, GithubLicense>>();

        self.github_licenses = body.unwrap();
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
