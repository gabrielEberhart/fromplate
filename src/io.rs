use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Checks if a file exists at the given path
pub fn find_templates(dir: &str, extension: &OsStr) -> Result<Vec<PathBuf>, io::Error> {
    let mut valid_templates = vec![];
    let path = Path::new(dir);

    for entry in fs::read_dir(path)? {
        let entry = entry?; // unwrap entry from container
        let path = entry.path();

        // check if path is a template for the extension
        if path.is_file() && path.extension() == Some(extension) {
            valid_templates.push(path);
        }
    }

    Ok(valid_templates)
}
