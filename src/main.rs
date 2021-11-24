use std::{fs, io};
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

fn main() -> io::Result<()> {
    let dirs_to_del = to_del();

    for dir in dirs_to_del.iter() {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let entry = entry.path();

            if entry.is_file() {
                fs::remove_file(entry)?;
            } else {
                fs::remove_dir_all(entry)?;
            }

            println!("{}", entry.display());
        }
    }

    Ok(())
}
