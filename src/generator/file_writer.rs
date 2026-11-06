use crate::error::{Result, ScaffoldError};
use std::path::{Path, PathBuf};

pub struct FileWriter {
    base_dir: PathBuf,
    force: bool,
    dry_run: bool,
}

impl FileWriter {
    pub fn new(base_dir: PathBuf, force: bool, dry_run: bool) -> Self {
        Self {
            base_dir,
            force,
            dry_run,
        }
    }

    pub fn create_file(&self, relative_path: &str, content: &str) -> Result<()> {
        let full_path = self.base_dir.join(relative_path);

        if full_path.exists() && !self.force {
            return Err(ScaffoldError::FileExists(full_path.display().to_string()));
        }

        if self.dry_run {
            println!("  [dry-run] Would create: {}", full_path.display());
            return Ok(());
        }

        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&full_path, content)?;
        Ok(())
    }

    pub fn create_dir(&self, relative_path: &str) -> Result<()> {
        let full_path = self.base_dir.join(relative_path);
        if self.dry_run {
            println!("  [dry-run] Would create dir: {}", full_path.display());
            return Ok(());
        }
        std::fs::create_dir_all(&full_path)?;
        Ok(())
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }
}