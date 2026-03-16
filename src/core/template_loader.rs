use std::path::PathBuf;
use chrono::Local;

pub struct TemplateContext {
    pub filename: String,
    pub author: String,
}

pub fn load_template(ext: &str, ctx: &TemplateContext) -> Option<String> {
    let system_path = system_template_path(ext);
    
    let raw = std::fs::read_to_string(&user_template_path(ext))
        .or_else(|_| std::fs::read_to_string(&system_path))
        .ok()?;

    Some(apply_variables(&raw, ctx))
}

fn apply_variables(template: &str, ctx: &TemplateContext) -> String {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let year = Local::now().format("%Y").to_string();

    let author_val = if ctx.author.trim().is_empty() {
        "Your Name"
    } else {
        &ctx.author
    };

    template
        .replace("{{filename}}", &ctx.filename)
        .replace("{{date}}", &date)
        .replace("{{year}}", &year)
        .replace("{{author}}", author_val)
}

fn system_template_path(ext: &str) -> PathBuf {
    PathBuf::from(r"C:\Program Files\ShellFile\templates")
        .join(format!("{}.tpl", ext))
}

pub fn user_template_dir() -> PathBuf {
    let appdata = std::env::var("LOCALAPPDATA").unwrap_or_default();
    if appdata.trim().is_empty() {
        eprintln!("Error: LOCALAPPDATA not found");
        std::process::exit(1);
    }
    PathBuf::from(appdata)
        .join("ShellFile")
        .join("templates")
}

pub fn user_template_path(ext: &str) -> PathBuf {
    user_template_dir().join(format!("{}.tpl", ext))
}
