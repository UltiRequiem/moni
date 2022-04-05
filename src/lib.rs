use std::{
    fs::{metadata, set_permissions},
    path::Path,
};

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
