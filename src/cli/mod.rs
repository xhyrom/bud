use clap::{arg, Command};

use crate::{config::CONFIG_PATH, constants};

mod new_command;
mod version_command;

fn command() -> Command {
    Command::new("bud")
        .version("0.1.0")
        .author("xHyroM")
        .subcommand(new_command::new())
        .subcommand(version_command::new())
        .args(&[arg!(--"config-path" [path] "Path to the config file")])
        .arg_required_else_help(true)
}

pub fn handle() {
    let cmd = command();
    let matches = cmd.get_matches();

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

    match matches.subcommand() {
        Some(("new", matches)) => new_command::handle(matches),
        Some(("version", _)) => version_command::handle(),
        _ => {}
    }
}
