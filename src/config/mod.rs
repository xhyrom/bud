use std::{fs::{self, File}, io};

use serde::Deserialize;
use lazy_static::lazy_static;

use crate::{constants};

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
    },
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

  fn initialize() -> Result<File, io::Error> {
    fs::create_dir_all((*constants::FOLDER).clone()).and_then(|_| {
      let config_path = format!("{}/config.toml", *constants::FOLDER);

      if fs::metadata(&config_path).is_err() {
        return fs::File::create(config_path)
      }
      
      Ok(fs::File::open(config_path)?)
    })
  }
}