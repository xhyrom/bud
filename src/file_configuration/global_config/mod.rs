use std::{
    fs::{self},
    sync::{Mutex, MutexGuard},
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::constants;

mod licenses_field;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "licenses_field::default_values::licenses")]
    pub licenses: licenses_field::Licenses,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            licenses: licenses_field::default_values::licenses(),
        }
    }
}

lazy_static! {
    pub static ref CONFIG_PATH: Mutex<String> = Mutex::new(String::new());
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Mutex<Config> = Mutex::new({
        let config_path = CONFIG_PATH.lock().unwrap().to_string();
        match Config::new(config_path) {
            Ok(config) => config,
            Err(err) => {
                logger::error!("Failed to load global config: {}", err);
                std::process::exit(1);
            }
        }
    });
}

impl Config {
    pub fn new(config_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        if fs::metadata(&config_path).is_err() {
            logger::debug!("Global config file not found, creating one...");
            fs::create_dir_all((*constants::DATA_FOLDER).clone())?;

            let config = Self::default();
            config.save()?;

            return Ok(config);
        }

        let contents = fs::read_to_string(&config_path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn serialize(&self) -> Result<String, Box<dyn std::error::Error>> {
        let config = toml::to_string(&self)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = CONFIG_PATH.lock().unwrap().to_string();
        let config = self.serialize()?;

        fs::write(config_path, config)?;
        Ok(())
    }

    pub fn instance() -> MutexGuard<'static, Config> {
        let config = CONFIG.lock().unwrap();
        config
    }
}

pub fn setup(config_path: String) {
    let mut config_path_mutex = CONFIG_PATH.lock().unwrap();
    *config_path_mutex = config_path;
    drop(config_path_mutex);

    lazy_static::initialize(&CONFIG);
}
