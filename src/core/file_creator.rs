use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const NEW_FILE_STEM: &str = "new_file";

pub fn create_file(dir: &Path, ext: &str, template: Option<String>) -> Result<PathBuf, io::Error> {
    let path = unique_path(dir, NEW_FILE_STEM, ext)?;
    
    let content = template.unwrap_or_default();
    fs::write(&path, content)?;
    println!("Created: {}", path.display());
    
    Ok(path)
}

fn unique_path(dir: &Path, stem: &str, ext: &str) -> Result<PathBuf, io::Error> {
    let base = dir.join(format!("{}.{}", stem, ext));
    if !base.exists() { return Ok(base); }
    (1..=9999).map(|i| dir.join(format!("{}_{}.{}", stem, i, ext)))
         .find(|p| !p.exists())
         .ok_or_else(|| io::Error::new(io::ErrorKind::AlreadyExists, "File limit reached"))
}
