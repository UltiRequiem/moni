use clap::App;
use futures::executor::block_on;
use std::{fs, io, path};
use walkdir::WalkDir;

fn to_delete() -> Vec<&'static str> {
    vec![
        "node_modules",
        "__pycache__",
        ".cache",
        "dist",
        "build",
        "bowerComponents",
    ]
}

async fn delete(entry: &path::Path) -> io::Result<()> {
    if entry.is_file() {
        fs::remove_file(entry)?;
    } else {
        fs::remove_dir_all(entry)?;
    }

    println!("{}", entry.display());

    Ok(())
}

async fn async_main() -> io::Result<()> {
    for dir in to_delete().iter() {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            delete(entry.path()).await?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    App::new("moni")
        .bin_name("moni")
        .version("v0.1.0")
        .author("UltiRequiem <https://github.com/UltiRequiem>")
        .about("Delete directories to free up disk space.")
        .get_matches();

    block_on(async_main())?;

    Ok(())
}
