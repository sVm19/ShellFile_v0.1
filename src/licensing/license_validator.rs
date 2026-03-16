use std::collections::HashSet;
use std::path::PathBuf;
use sha2::{Sha256, Digest};
use std::sync::OnceLock;

const HASHES: &str = include_str!("../../keys/hashes.txt");

fn load_valid_key_hashes() -> &'static HashSet<String> {
    static HASHES_SET: OnceLock<HashSet<String>> = OnceLock::new();
    HASHES_SET.get_or_init(|| {
        HASHES
            .lines()
            .map(|s| s.trim().to_string())
            .collect()
    })
}

pub fn validate_key(key: &str) -> bool {
    let parts: Vec<&str> = key.split('-').collect();
    if parts.len() != 4 || parts[0] != "SFPRO" { return false; }
    
    let hash = format!("{:x}", Sha256::digest(key.as_bytes()));
    load_valid_key_hashes().contains(&hash)
}

fn license_path() -> PathBuf {
    let appdata = std::env::var("LOCALAPPDATA").unwrap_or_default();
    if appdata.trim().is_empty() {
        eprintln!("Error: LOCALAPPDATA not found");
        std::process::exit(1);
    }
    PathBuf::from(appdata)
        .join("ShellFile")
        .join("license.json")
}

pub fn store_license(key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = license_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let data = serde_json::json!({
        "product": "ShellFile Pro",
        "license_key": key,
        "activated": true,
        "activated_at": chrono::Local::now().format("%Y-%m-%d").to_string()
    });
    std::fs::write(path, serde_json::to_string_pretty(&data)?)?;
    Ok(())
}

pub fn is_pro_activated() -> bool {
    let path = license_path();
    if !path.exists() { return false; }
    
    let Ok(content) = std::fs::read_to_string(&path) else { return false; };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) else { return false; };
    
    let activated = json["activated"].as_bool().unwrap_or(false);
    let key = json["license_key"].as_str().unwrap_or("");
    
    activated && validate_key(key)
}
