use clap::{arg, ArgMatches, Command};
use logger::info;

use crate::templates::Template;

pub fn new() -> Command {
    Command::new("new")
        .about("Create a new project")
        .args(&[arg!(template: "Template name")])
}

pub fn handle(matches: &ArgMatches) {
    let template = matches
        .get_one::<String>("template")
        .unwrap_or(&"dummy".to_string())
        .to_owned();

    info!("Creating new project from template: {}", template);

    let template = Template::new(None, template);
    template.download();

    info!("Successfully cloned template")
}
