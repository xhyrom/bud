use std::fs;

use clap::Command;
use runtime::{globals::prompts::get_prompts, Runtime};

pub fn new() -> Command {
    Command::new("test").about("Test")
}

pub fn handle() {
    let contents = fs::read_to_string("templates/dummy/.bud/main.lua");
    let binding = contents.unwrap();
    let contents = binding.as_str();

    let runtime = Runtime::new();

    let mut prompts = get_prompts();
    prompts.push("".to_string());
    prompts.push("hyro".to_string());
    prompts.push(String::from("yes"));

    drop(prompts);

    runtime.run(contents).unwrap();
}
