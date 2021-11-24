use std::{fs, io, path};
use walkdir::WalkDir;

fn to_del() -> Vec<&'static str> {
    vec![
        "node_modules",
        "__pycache__",
        ".cache",
        "dist",
        "build",
        "bowerComponents",
    ]
}

fn delete(entry: &path::Path) -> io::Result<()> {
    if entry.is_file() {
        fs::remove_file(entry)?;
    } else {
        fs::remove_dir_all(entry)?;
    }

    println!("{}", entry.display());

    Ok(())
}

fn main() -> io::Result<()> {
    for dir in to_del().iter() {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            delete(entry.path())?;
        }
    }

    Ok(())
}
