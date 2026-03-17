mod core;
mod windows;

use std::env;
use std::path::PathBuf;

fn print_help() {
    println!("ShellFile");
    println!("Usage: shellfile <command_or_ext> [path]");
    println!();
    println!("Commands:");
    println!("  --help, -h           Show this help message");
    println!("  --register           Register the Windows context menu");
    println!("  --unregister         Unregister the Windows context menu");
    println!("  --add-template       Open the custom template folder");
    println!("  project <type> [path] Create a project scaffold");
    println!();
    println!("File creation:");
    println!("  shellfile <ext> [path] Create a new file for the given extension");
}

fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| {
        eprintln!("Error: cannot determine working directory");
        std::process::exit(1);
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: shellfile <command_or_ext> [path]");
        std::process::exit(1);
    }

    let cmd = args[1].as_str();

    match cmd {
        "--help" | "-h" => {
            print_help();
        }
        "--register" => {
            if let Err(e) = windows::integration::register_context_menu() {
                eprintln!("Failed to register context menu: {}", e);
                std::process::exit(1);
            }
            windows::integration::restart_explorer();
            println!("Context menu registered.");
        }
        "--unregister" => {
            if let Err(e) = windows::integration::unregister_context_menu() {
                eprintln!("Failed to unregister context menu: {}", e);
                std::process::exit(1);
            }
            windows::integration::restart_explorer();
            println!("Context menu unregistered.");
        }
        "--add-template" => {
            let dir = core::template_loader::user_template_dir();
            if let Err(e) = std::fs::create_dir_all(&dir) {
                eprintln!("Failed to create template directory: {}", e);
                std::process::exit(1);
            }
            if let Err(e) = std::process::Command::new("explorer").arg(&dir).spawn() {
                eprintln!("Failed to open explorer: {}", e);
                std::process::exit(1);
            }
        }
        "project" => {
            let p_type = args.get(2).unwrap_or_else(|| {
                eprintln!("Please provide project type (python, node, rust)");
                std::process::exit(1);
            });
            let target_dir = args.get(3).map(PathBuf::from).unwrap_or_else(get_current_dir);
            if let Err(e) = core::project_generator::generate_project(&target_dir, p_type) {
                eprintln!("Failed to create project: {}", e);
                std::process::exit(1);
            }
            println!("Created {} project.", p_type);
        }
        ext => {
             let target_dir = args.get(2).map(PathBuf::from).unwrap_or_else(get_current_dir);
             let config_path = PathBuf::from(r"C:\Program Files\ShellFile\config\config.json");
             let author = std::fs::read_to_string(&config_path)
                 .ok()
                 .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
                 .and_then(|v| v["author"].as_str().map(String::from))
                 .unwrap_or_else(|| std::env::var("USERNAME").unwrap_or_else(|_| "Your Name".to_string()));

             let ctx = core::template_loader::TemplateContext {
                 filename: ext.to_string(),
                 author,
             };
             let template_str = core::template_loader::load_template(ext, &ctx);

             match core::file_creator::create_file(&target_dir, ext, template_str) {
                 Ok(path) => {
                     println!("Created: {}", path.display());
                 }
                 Err(e) => {
                     eprintln!("Failed to create file: {}", e);
                     std::process::exit(1);
                 }
             }
        }
    }
}
