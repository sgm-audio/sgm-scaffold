use std::path::{Path, PathBuf};

/// Detect if we're inside a Cargo workspace. Returns the workspace root if found.
pub fn detect_workspace(dir: &Path) -> Option<PathBuf> {
    let mut current = dir.to_path_buf();
    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
                if content.contains("[workspace]") {
                    return Some(current);
                }
            }
        }
        if !current.pop() {
            break;
        }
    }
    None
}

/// Check if a directory exists and is non-empty.
pub fn dir_exists_and_nonempty(path: &Path) -> bool {
    path.exists() && path.is_dir() && path.read_dir().map(|mut d| d.next().is_some()).unwrap_or(false)
}

/// Ensure a directory exists (create if not).
pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Get the user's config directory for sgm-scaffold.
pub fn config_dir() -> Option<PathBuf> {
    // Cross-platform config dir without external crate
    if cfg!(target_os = "windows") {
        std::env::var("APPDATA").ok().map(|p| PathBuf::from(p).join("sgm-scaffold"))
    } else if cfg!(target_os = "macos") {
        std::env::var("HOME").ok().map(|p| PathBuf::from(p).join("Library/Application Support/sgm-scaffold"))
    } else {
        std::env::var("XDG_CONFIG_HOME")
            .or_else(|_| std::env::var("HOME").map(|h| format!("{}/.config", h)))
            .ok()
            .map(|p| PathBuf::from(p).join("sgm-scaffold"))
    }
}

/// Get the global templates directory.
pub fn templates_dir() -> Option<PathBuf> {
    config_dir().map(|p| p.join("templates"))
}