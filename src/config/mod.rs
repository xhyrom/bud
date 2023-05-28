use std::{
    fs::{self},
    io,
};

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::constants;

mod license_field;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "license_field::default_values::license")]
    pub license: license_field::License,
}

lazy_static! {
    pub static ref CONFIG: Config = match Config::new() {
        Ok(config) => config,
        Err(err) => {
            logger::error!("Failed to load config: {}", err);
            std::process::exit(1);
        }
    };
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::initialize()?;

        let config_path = format!("{}/config.toml", *constants::FOLDER);

        let contents = fs::read_to_string(config_path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    fn initialize() -> io::Result<()> {
        fs::create_dir_all((*constants::FOLDER).clone())?;

        let config_path = format!("{}/config.toml", *constants::FOLDER);

        if fs::metadata(&config_path).is_err() {
            return match fs::File::create(config_path) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            };
        }

        Ok(())
    }
}
