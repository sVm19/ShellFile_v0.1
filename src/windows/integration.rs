use winreg::enums::*;
use winreg::RegKey;
use std::process::Command;

const FREE_EXTS: &[(&str, &str)] = &[
    ("md", "New Markdown File"),
    ("py", "New Python File"),
    ("js", "New JavaScript File"),
    ("jsx", "New React Component"),
    ("ts", "New TypeScript File"),
    ("tsx", "New React TypeScript Component"),
    ("cpp", "New C++ File"),
    ("c", "New C File"),
    ("h", "New Header File"),
    ("java", "New Java File"),
    ("cs", "New C# File"),
    ("php", "New PHP File"),
    ("rb", "New Ruby File"),
    ("json", "New JSON File"),
    ("xml", "New XML File"),
    ("html", "New HTML File"),
    ("css", "New CSS File"),
    ("scss", "New SCSS File"),
    ("txt", "New Text File"),
    ("podman", "New Podman File"),
];

const PRO_EXTS: &[(&str, &str)] = &[
    ("rs", "New Rust File"),
    ("go", "New Go File"),
    ("swift", "New Swift File"),
    ("kt", "New Kotlin File"),
    ("sql", "New SQL File"),
    ("csv", "New CSV File"),
    ("env", "New ENV File"),
    ("gitignore", "New Gitignore File"),
    ("dockerfile", "New Dockerfile"),
    ("makefile", "New Makefile"),
    ("bin", "New Binary File"),
    ("lib", "New Library File"),
    ("obj", "New Object File"),
    ("pdb", "New Debug Symbols File"),
    ("ini", "New INI Config"),
    ("cfg", "New Config File"),
    ("conf", "New Conf File"),
    ("toml", "New TOML File"),
    ("yaml", "New YAML File"),
    ("yml", "New YAML File"),
    ("proto", "New Protobuf File"),
    ("graphql", "New GraphQL Schema"),
    ("pem", "New PEM Certificate"),
    ("crt", "New Certificate File"),
    ("key", "New Key File"),
    ("p12", "New PKCS12 File"),
    ("avro", "New Avro Schema"),
    ("parquet", "New Parquet File"),
    ("log", "New Log File"),
    ("dump", "New Dump File"),
    ("trace", "New Trace File"),
];

pub fn register_context_menu() -> Result<(), Box<dyn std::error::Error>> {
    if !is_elevated::is_elevated() {
        eprintln!("Please run as Administrator");
        return Err("Not Administrator".into());
    }

    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let base = r"Directory\Background\shell";
    let is_pro = crate::licensing::license_validator::is_pro_activated();

    for &(ext, label) in FREE_EXTS {
        let key_path = format!(r"{}\ShellFile_{}", base, ext);
        let (key, _) = hkcr.create_subkey(&key_path)?;
        key.set_value("", &label)?;

        let (cmd_key, _) = hkcr.create_subkey(format!(r"{}\command", key_path))?;
        let exe = r"C:\Program Files\ShellFile\shellfile.exe";
        cmd_key.set_value("", &format!("\"{}\" {} \"%V\"", exe, ext))?;
    }

    for &(ext, label) in PRO_EXTS {
        let label = if is_pro {
            label.to_string()
        } else {
            format!("{} [Pro]", label)
        };
        let key_path = format!(r"{}\ShellFile_{}", base, ext);
        let (key, _) = hkcr.create_subkey(&key_path)?;
        key.set_value("", &label)?;

        let (cmd_key, _) = hkcr.create_subkey(format!(r"{}\command", key_path))?;
        let exe = r"C:\Program Files\ShellFile\shellfile.exe";
        cmd_key.set_value("", &format!("\"{}\" {} \"%V\"", exe, ext))?;
    }
    Ok(())
}

pub fn unregister_context_menu() -> Result<(), Box<dyn std::error::Error>> {
    if !is_elevated::is_elevated() {
        eprintln!("Please run as Administrator");
        return Err("Not Administrator".into());
    }

    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    for ext in &[
        "md", "py", "js", "jsx", "ts", "tsx", "cpp", "c", "h", "rs", "go",
        "java", "cs", "php", "rb", "swift", "kt", "json", "yaml", "yml",
        "toml", "xml", "html", "css", "scss", "sql",
        "txt", "csv", "env", "gitignore", "dockerfile", "makefile", "podman",
        "bin", "lib", "obj", "pdb", "ini", "cfg", "conf",
        "plist", "proto", "thrift", "graphql", "wasm", "htaccess", 
        "webmanifest", "pem", "crt", "key", "p12", "cmake", "gradle", "lock",
        "nix", "msgpack", "avro", "parquet", "cbor", "log", "dump", "trace"
    ] {
        let _ = hkcr.delete_subkey_all(
            format!(r"Directory\Background\shell\ShellFile_{}", ext)
        );
    }
    Ok(())
}

pub fn restart_explorer() {
    Command::new("taskkill").args(["/F", "/IM", "explorer.exe"]).status().ok();
    std::thread::sleep(std::time::Duration::from_millis(500));
    Command::new("explorer.exe").spawn().ok();
}
