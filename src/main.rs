use clap::Parser;
use home::home_dir;
use moni::{common_unwanted_directories, delete_dirs};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::{path::Path, process::exit, time::Instant};
use tokio::fs::read_to_string;

#[derive(Parser)]
#[clap(author="Eliaz Bobadilla", version="0.2.0", about="Delete unwanted files.", long_about = None)]
struct Args {
    #[clap(short, long)]
    files: Vec<String>,
    #[clap(short, long)]
    config: bool,
}

#[derive(Serialize, Deserialize)]
struct Config {
    files: Vec<String>,
}

async fn read_config() -> Config {
    let home = match home_dir() {
        Some(path) => path,
        None => {
            eprintln!("{}", "Impossible to get your home dir!".red());
            exit(1);
        }
    };

    let home = home.into_os_string().into_string().unwrap();

    let config_path = Path::new(&home).join(".moni.json");

    let config = match read_to_string(config_path).await {
        Ok(config) => config,
        Err(_) => {
            eprintln!("{}", "Impossible to read config file!".red());
            exit(1);
        }
    };

    let config: Config = match serde_json::from_str(&config) {
        Ok(config) => config,
        Err(_) => {
            eprintln!("{}", "Impossible to parse config file!".red());
            exit(1);
        }
    };

    config
}

#[tokio::main]
async fn main() {
    let mut parsed_new_dirs = vec![".DS_Store".to_string()];

    let args = Args::parse();

    if args.config {
        let config = read_config().await;

        for file in config.files {
            parsed_new_dirs.push(file);
        }
    }

    for var in args.files {
        parsed_new_dirs.push(var);
    }

    parsed_new_dirs.append(&mut common_unwanted_directories());

    let start = Instant::now();

    let total_files = delete_dirs(parsed_new_dirs.clone()).await;

    println!(
        "{}",
        format!("Done in {}!", start.elapsed().as_secs_f32()).green()
    );

    println!("{}", format!("Deleted {} files.", total_files).yellow())
}
