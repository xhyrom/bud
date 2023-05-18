mod config;
mod constants;

fn main() {
    println!("🌷 Bud {}", (*config::CONFIG).hello);
}
