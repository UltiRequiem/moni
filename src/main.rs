use futures::future::join_all;
use moni::{common_unwanted_directories, set_writable};
use owo_colors::{colors::*, OwoColorize};
use std::{io::Result, path::Path, time::Instant};
use tokio::fs::{remove_dir_all, remove_file};
use walkdir::WalkDir;

async fn delete(entry: &Path) -> Result<()> {
    if entry.is_file() {
        remove_file(entry).await?;
    } else {
        remove_dir_all(entry).await?;
    }

    println!("{}", entry.display().fg::<Black>().bg::<Yellow>());

    Ok(())
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let dirs_to_del = common_unwanted_directories();

    let mut futures = Vec::new();

    for dir in dirs_to_del.iter() {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.into_path();
            set_writable(&path);
            let future = async move { delete(&path).await };
            futures.push(future);
        }
    }

    join_all(futures).await;

    println!("{}", format!("{}", start.elapsed().as_secs_f32()).green())
}
