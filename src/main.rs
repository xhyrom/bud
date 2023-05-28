mod cli;
mod config;
mod constants;

fn main() {
    cli::handle();

    println!("🌷 Bud {:?}", config::CONFIG.license);
}
