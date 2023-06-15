use clap::{arg, Command};

use crate::file_configuration;

mod new_command;
#[cfg(debug_assertions)]
mod test_command;
mod version_command;

fn command() -> Command {
    #[cfg(debug_assertions)]
    return Command::new("bud")
        .version("0.1.0")
        .author("xHyroM")
        .subcommand(new_command::new())
        .subcommand(version_command::new())
        .subcommand(test_command::new())
        .args(&[arg!(--"global-config-path" [path] "Path to the global config file")])
        .disable_version_flag(true)
        .arg_required_else_help(true)
        .subcommand_required(true);
    #[cfg(not(debug_assertions))]
    return Command::new("bud")
        .version("0.1.0")
        .author("xHyroM")
        .subcommand(new_command::new())
        .subcommand(version_command::new())
        .args(&[arg!(--"global-config-path" [path] "Path to the global config file")])
        .disable_version_flag(true)
        .arg_required_else_help(true)
        .subcommand_required(true);
}

pub fn handle() {
    let cmd = command();
    let matches = cmd.get_matches();

    file_configuration::initialize(&matches);

    match matches.subcommand() {
        Some(("new", matches)) => new_command::handle(matches),
        Some(("version", _)) => version_command::handle(),
        Some(("test", _)) => test_command::handle(),
        _ => unreachable!(),
    }
}
