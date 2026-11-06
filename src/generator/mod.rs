pub mod file_writer;
pub mod template;

pub use file_writer::FileWriter;
pub use template::{TemplateContext, TemplateEngine};

use crate::error::Result;
use std::path::Path;

/// Run post-generation validation (cargo check).
pub fn validate_project(project_dir: &Path) -> Result<()> {
    let output = std::process::Command::new("cargo")
        .args(["check", "--all-targets"])
        .current_dir(project_dir)
        .output()
        .map_err(|e| crate::error::ScaffoldError::ValidationFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(crate::error::ScaffoldError::ValidationFailed(
            stderr.to_string(),
        ));
    }
    Ok(())
}

/// Initialize a git repository in the project directory.
pub fn init_git(project_dir: &Path) -> Result<()> {
    // git init
    let output = std::process::Command::new("git")
        .arg("init")
        .current_dir(project_dir)
        .output()
        .map_err(|e| crate::error::ScaffoldError::Git(e.to_string()))?;

    if !output.status.success() {
        return Err(crate::error::ScaffoldError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    // git add -A
    let _ = std::process::Command::new("git")
        .args(["add", "-A"])
        .current_dir(project_dir)
        .output();

    // git commit
    let _ = std::process::Command::new("git")
        .args(["commit", "-m", "Initial scaffold from sgm-scaffold"])
        .current_dir(project_dir)
        .output();

    Ok(())
}