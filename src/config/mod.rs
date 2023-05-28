use std::{
    fs::{self},
    io,
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::constants;

mod license_field;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "license_field::default_values::license")]
    pub license: license_field::License,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            license: license_field::default_values::license(),
        }
    }
}

lazy_static! {
    pub static ref CONFIG_PATH: Mutex<String> = Mutex::new(String::new());
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_path = CONFIG_PATH.lock().unwrap().to_string();
        match Config::new(config_path) {
            Ok(config) => config,
            Err(err) => {
                logger::error!("Failed to load config: {}", err);
                std::process::exit(1);
            }
        }
    };
}

impl Config {
    pub fn new(config_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        Self::initialize(&config_path)?;

        let contents = fs::read_to_string(&config_path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    fn initialize(config_path: &String) -> io::Result<()> {
        fs::create_dir_all((*constants::FOLDER).clone())?;

        if fs::metadata(config_path).is_err() {
            return match fs::File::create(config_path) {
                Ok(_) => {
                    let config = toml::to_string(&Config::default()).unwrap_or(String::new());
                    fs::write(config_path, config)?;
                    Ok(())
                }
                Err(err) => Err(err),
            };
        }

        Ok(())
    }
}
