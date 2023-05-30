pub mod global_config;

use crate::constants;
use clap::ArgMatches;

pub use global_config::CONFIG as GLOBAL_CONFIG;
pub use global_config::CONFIG_PATH as GLOBAL_CONFIG_PATH;

pub fn initialize(matches: &ArgMatches) {
    let config_path = if matches.contains_id("global-config-path") {
        matches
            .get_one::<String>("global-config-path")
            .unwrap()
            .to_string()
    } else {
        format!("{}/config.toml", *constants::DATA_FOLDER)
    };

    global_config::setup(config_path)
}
