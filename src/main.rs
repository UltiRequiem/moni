use clap::Parser;
use moni::{common_unwanted_directories, delete_dirs};
use owo_colors::OwoColorize;
use std::time::Instant;

#[derive(Parser)]
#[clap(author="Eliaz Bobadilla", version="1.0.0", about="Delete unwanted files.", long_about = None)]
struct Args {
    #[clap(short, long)]
    files: Vec<String>,
}

#[tokio::main]
async fn main() {
    // putting something so Rust can infer the type
    let mut parsed_new_dirs = vec![".DS_Store"];

    let args = Args::parse();

    for var in &args.files {
        parsed_new_dirs.push(var);
    }

    parsed_new_dirs.append(&mut common_unwanted_directories());

    let start = Instant::now();

    delete_dirs(parsed_new_dirs).await;

    println!(
        "{}",
        format!("Done in {}!", start.elapsed().as_secs_f32()).green()
    )
}
