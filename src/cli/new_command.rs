use clap::{arg, ArgMatches, Command};

pub fn new() -> Command {
    Command::new("new")
        .about("Create a new project")
        .args(&[arg!(template: "Template name")])
}

pub fn handle(_: &ArgMatches) {
    println!("New command");
}
