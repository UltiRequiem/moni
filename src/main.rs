use clap::Command;
use moni::{common_unwanted_directories, delete_dirs};
use owo_colors::OwoColorize;
use std::time::Instant;

#[tokio::main]
async fn main() {
    Command::new("moni")
        .bin_name("moni")
        .version("v0.1.0")
        .author("UltiRequiem <https://github.com/UltiRequiem>")
        .about("Delete directories to free up disk space.")
        .get_matches();

    let start = Instant::now();

    let dirs_to_del = common_unwanted_directories();

    delete_dirs(dirs_to_del).await;

    println!(
        "{}",
        format!("Done in {}!", start.elapsed().as_secs_f32()).green()
    )
}
