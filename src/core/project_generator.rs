use std::fs;
use std::io;
use std::path::Path;

pub fn generate_project(base_dir: &Path, project_type: &str) -> Result<(), io::Error> {
    if base_dir.exists() && fs::read_dir(base_dir)?.next().is_some() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Directory is not empty"));
    }

    match project_type {
        "python" => generate_python_project(base_dir),
        "node" => generate_node_project(base_dir),
        "rust" => generate_rust_project(base_dir),
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown project type")),
    }
}

fn generate_python_project(dir: &Path) -> Result<(), io::Error> {
    fs::create_dir_all(dir.join("src"))?;
    fs::write(dir.join("README.md"), "# Python Project\n")?;
    fs::write(dir.join("src").join("main.py"), "def main():\n    pass\n\nif __name__ == '__main__':\n    main()\n")?;
    fs::write(dir.join("requirements.txt"), "")?;
    fs::write(dir.join(".gitignore"), "__pycache__/\n*.pyc\n")?;
    Ok(())
}

fn generate_node_project(dir: &Path) -> Result<(), io::Error> {
    let project_name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("node-project");
    fs::create_dir_all(dir.join("src"))?;
    fs::write(dir.join("README.md"), format!("# {}\n", project_name))?;
    fs::write(dir.join("src").join("index.js"), "console.log('Hello Node');\n")?;
    fs::write(dir.join("package.json"), format!("{{\n  \"name\": \"{}\",\n  \"version\": \"1.0.0\",\n  \"main\": \"src/index.js\"\n}}\n", project_name))?;
    fs::write(dir.join(".gitignore"), "node_modules/\n")?;
    Ok(())
}

fn generate_rust_project(dir: &Path) -> Result<(), io::Error> {
    let project_name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("rust-project");
    fs::create_dir_all(dir.join("src"))?;
    fs::write(dir.join("README.md"), format!("# {}\n", project_name))?;
    fs::write(dir.join("src").join("main.rs"), "fn main() {\n    println!(\"Hello Rust\");\n}\n")?;
    fs::write(dir.join("Cargo.toml"), format!("[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n", project_name))?;
    fs::write(dir.join(".gitignore"), "/target\n")?;
    Ok(())
}
