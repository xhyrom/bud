use clap::Command;
use logger::info;

pub fn new() -> Command {
    Command::new("version").about("Show the version")
}

pub fn handle() {
    #[cfg(debug_assertions)]
    info!("bud {} (debug)", env!("CARGO_PKG_VERSION"));
    #[cfg(not(debug_assertions))]
    info!("bud {} (release)", env!("CARGO_PKG_VERSION"));
}
