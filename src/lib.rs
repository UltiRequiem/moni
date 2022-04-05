use futures::future::join_all;
use owo_colors::{colors::*, OwoColorize};
use std::{
    fs::{metadata, set_permissions},
    io::Result,
    path::Path,
};
use tokio::fs::{remove_dir_all, remove_file};
use walkdir::WalkDir;

pub fn common_unwanted_directories() -> Vec<&'static str> {
    vec![
        "node_modules",
        "target",
        "__pycache__",
        ".cache",
        "dist",
        "build",
        "bowerComponents",
    ]
}

pub fn set_writable(path: &Path) {
    let mut perms = metadata(path).unwrap().permissions();

    perms.set_readonly(false);

    set_permissions(path, perms).unwrap();
}

pub async fn delete(entry: &Path) -> Result<()> {
    if entry.is_file() {
        remove_file(entry).await?;
    } else {
        remove_dir_all(entry).await?;
    }

    println!("{}", entry.display().fg::<Black>().bg::<Yellow>());

    Ok(())
}

pub async fn delete_dirs(dirs: Vec<&str>) {
    let mut futures = Vec::new();

    for dir in dirs.iter() {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.into_path();
            set_writable(&path);
            let future = async move { delete(&path).await };
            futures.push(future);
        }
    }

    join_all(futures).await;
}
