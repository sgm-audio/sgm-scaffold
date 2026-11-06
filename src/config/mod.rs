use crate::error::{Result, ScaffoldError};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct OrgConfig {
    #[serde(default)]
    pub organization: OrgInfo,
    #[serde(default)]
    pub engineering: EngConfig,
    #[serde(default)]
    pub quality: QualityConfig,
    #[serde(default)]
    pub brand: BrandConfig,
    #[serde(default)]
    pub defaults: DefaultsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrgInfo {
    #[serde(default = "default_org_name")]
    pub name: String,
    #[serde(default = "default_github")]
    pub github: String,
    #[serde(default = "default_website")]
    pub website: String,
}

impl Default for OrgInfo {
    fn default() -> Self {
        Self {
            name: default_org_name(),
            github: default_github(),
            website: default_website(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct EngConfig {
    #[serde(default = "default_rust_version")]
    pub msrv: String,
    #[serde(default = "default_edition")]
    pub rust_edition: String,
    #[serde(default = "default_nih_plug")]
    pub nih_plug_version: String,
}

impl Default for EngConfig {
    fn default() -> Self {
        Self {
            msrv: default_rust_version(),
            rust_edition: default_edition(),
            nih_plug_version: default_nih_plug(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct QualityConfig {
    #[serde(default = "default_coverage")]
    pub test_coverage_target: u32,
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            test_coverage_target: default_coverage(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct BrandConfig {
    #[serde(default = "default_license")]
    pub license_proprietary: String,
    #[serde(default = "default_accent")]
    pub accent_color: String,
}

impl Default for BrandConfig {
    fn default() -> Self {
        Self {
            license_proprietary: default_license(),
            accent_color: default_accent(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DefaultsConfig {
    #[serde(default = "default_author")]
    pub author: String,
    #[serde(default = "default_author_email")]
    pub author_email: String,
    #[serde(default = "default_min_ios")]
    pub min_ios_version: String,
    #[serde(default = "default_min_android")]
    pub min_android_sdk: u32,
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            author: default_author(),
            author_email: default_author_email(),
            min_ios_version: default_min_ios(),
            min_android_sdk: default_min_android(),
        }
    }
}

fn default_org_name() -> String {
    "SGM Studios".into()
}
fn default_github() -> String {
    "SGM-Studios".into()
}
fn default_website() -> String {
    "https://sgmstudios.com".into()
}
fn default_rust_version() -> String {
    "1.88".into()
}
fn default_edition() -> String {
    "2024".into()
}
fn default_nih_plug() -> String {
    "0.12".into()
}
fn default_coverage() -> u32 {
    85
}
fn default_license() -> String {
    "LicenseRef-SGM-Proprietary".into()
}
fn default_accent() -> String {
    "#6C5CE7".into()
}
fn default_author() -> String {
    "SGM Studios".into()
}
fn default_author_email() -> String {
    "contact@sgmstudios.com".into()
}
fn default_min_ios() -> String {
    "16.0".into()
}
fn default_min_android() -> u32 {
    26
}

impl Default for OrgConfig {
    fn default() -> Self {
        Self {
            organization: OrgInfo::default(),
            engineering: EngConfig::default(),
            quality: QualityConfig::default(),
            brand: BrandConfig::default(),
            defaults: DefaultsConfig::default(),
        }
    }
}

/// Load org config from a file path or search default locations.
pub fn load_config(config_path: Option<&Path>) -> Result<OrgConfig> {
    // 1. Explicit path
    if let Some(path) = config_path {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ScaffoldError::Config(format!("Cannot read {}: {}", path.display(), e)))?;
        return toml::from_str(&content)
            .map_err(|e| ScaffoldError::Config(format!("Invalid TOML: {}", e)));
    }

    // 2. Current directory
    let local = Path::new("sgm-studios.toml");
    if local.exists() {
        let content = std::fs::read_to_string(local)
            .map_err(|e| ScaffoldError::Config(e.to_string()))?;
        if let Ok(cfg) = toml::from_str(&content) {
            return Ok(cfg);
        }
    }

    // 3. User config dir
    if let Some(cfg_dir) = crate::utils::path::config_dir() {
        let user_config = cfg_dir.join("sgm-studios.toml");
        if user_config.exists() {
            let content = std::fs::read_to_string(&user_config)
                .map_err(|e| ScaffoldError::Config(e.to_string()))?;
            if let Ok(cfg) = toml::from_str(&content) {
                return Ok(cfg);
            }
        }
    }

    // 4. Defaults
    Ok(OrgConfig::default())
}