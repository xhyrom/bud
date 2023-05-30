use clap::{arg, ArgMatches, Command};
use logger::debug;

use crate::file_configuration::GLOBAL_CONFIG;

pub fn new() -> Command {
    Command::new("config")
        .about("Manage configuration")
        .arg(arg!(<key> "Key"))
        .arg(arg!(<value> "Value"))
}

pub fn handle(matches: &ArgMatches) {
    debug!("{:?}", GLOBAL_CONFIG.licenses);

    let key = matches.get_one::<String>("key").unwrap();
    let value = matches.get_one::<String>("value").unwrap();

    _ = key;
    _ = value;
}
