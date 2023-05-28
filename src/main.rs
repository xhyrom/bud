use clap::{arg, Command};

use crate::config::CONFIG_PATH;

mod config;
mod constants;

fn main() {
    let command = Command::new("bud")
        .version("0.1.0")
        .author("xHyroM")
        .args(&[arg!(--"config-path" [path] "Path to the config file")]);

    let matches = command.get_matches();

    let mut config_path = CONFIG_PATH.lock().unwrap();
    *config_path = if matches.contains_id("config-path") {
        matches
            .get_one::<String>("config-path")
            .unwrap()
            .to_string()
    } else {
        format!("{}/config.toml", *constants::FOLDER)
    };
    drop(config_path);

    println!("🌷 Bud {:?}", config::CONFIG.license);
}
