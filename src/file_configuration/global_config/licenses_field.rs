use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Licenses {
    #[serde(default = "default_values::default")]
    pub default: String,
    #[serde(default = "default_values::copyright_holder")]
    pub copyright_holder: String,
    #[serde(default = "default_values::custom")]
    pub custom: HashMap<String, License>,

    #[serde(skip)]
    pub default_licenses: Vec<License>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct License {
    pub key: String,
    pub name: String,
    pub permissions: Option<Vec<String>>,
    pub conditions: Option<Vec<String>>,
    pub limitations: Option<Vec<String>>,
}

impl Licenses {
    pub fn fetch_default_licenses(&mut self) {
        let body = reqwest::blocking::get(
            "https://raw.githubusercontent.com/xHyroM/bud/main/licenses/licenses.json",
        )
        .unwrap()
        .json::<Vec<License>>();

        self.default_licenses = body.unwrap();
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
            default_licenses: vec![],
        }
    }

    pub fn default() -> String {
        String::from("MIT")
    }

    pub fn copyright_holder() -> String {
        String::from("Your Name")
    }

    pub fn custom() -> HashMap<String, super::License> {
        HashMap::new()
    }
}
