use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct License {
    #[serde(default = "default_values::prefer")]
    pub prefer: String,
    #[serde(default = "default_values::copyright_holder")]
    pub copyright_holder: String,
    #[serde(default = "default_values::other")]
    pub other: HashMap<String, Custom>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Custom {
    pub name: String,
    pub content: String,
}

pub mod default_values {
    use std::collections::HashMap;

    pub fn license() -> super::License {
        super::License {
            prefer: prefer(),
            copyright_holder: copyright_holder(),
            other: other(),
        }
    }

    pub fn prefer() -> String {
        String::from("MIT")
    }

    pub fn copyright_holder() -> String {
        String::from("Your Name")
    }

    pub fn other() -> HashMap<String, super::Custom> {
        HashMap::new()
    }
}
