use clap::{arg, ArgMatches, Command};
use logger::debug;

use crate::file_configuration::global_config::Config;

pub fn new() -> Command {
    Command::new("config")
        .about("Manage configuration")
        .arg(arg!(<key> "Key"))
        .arg(arg!(<value> "Value"))
}

pub fn handle(matches: &ArgMatches) {
    let mut config = Config::instance();

    debug!("{:?}", config.licenses);
    config.licenses.fetch_default_licenses(); // fetch github licenses
    debug!("{:?}", config.licenses.default_licenses);

    drop(config);

    let key = matches.get_one::<String>("key").unwrap();
    let value = matches.get_one::<String>("value").unwrap();

    _ = key;
    _ = value;
}
