use crate::error::{Result, ScaffoldError};
use minijinja::{Environment, context};
use rust_embed::RustEmbed;
use serde::Serialize;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct TemplateAssets;

pub struct TemplateEngine {
    env: Environment<'static>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        let mut env = Environment::new();
        // Register custom filters
        env.add_filter("upper", |value: String| value.to_uppercase());
        env.add_filter("lower", |value: String| value.to_lowercase());

        // Load all embedded templates
        for file in TemplateAssets::iter() {
            if let Some(content) = TemplateAssets::get(&file) {
                if let Ok(text) = std::str::from_utf8(&content.data) {
                    let _ = env.add_template_owned(file.to_string(), text.to_string());
                }
            }
        }
        Self { env }
    }

    pub fn render(&self, template_name: &str, ctx: &TemplateContext) -> Result<String> {
        let template = self
            .env
            .get_template(template_name)
            .map_err(|e| ScaffoldError::TemplateNotFound(template_name.to_string(), e.to_string()))?;
        template
            .render(ctx)
            .map_err(|e| ScaffoldError::RenderError(template_name.to_string(), e.to_string()))
    }

    /// List all available templates.
    pub fn list_templates(&self) -> Vec<String> {
        TemplateAssets::iter().map(|s| s.to_string()).collect()
    }
}

/// Template context passed to all templates.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateContext {
    // Identity
    pub project_name: String,
    pub crate_name: String,
    pub module_name: String,
    pub struct_name: String,
    pub display_name: String,
    pub plugin_name: String,
    pub description: String,

    // Organization
    pub author: String,
    pub author_email: String,
    pub license: String,
    pub repo_url: String,

    // Versions
    pub rust_version: String,
    pub edition: String,
    pub nih_plug_version: String,

    // Project type
    pub project_type: String,

    // Plugin-specific
    pub plugin_category: String,
    pub io_layout: String,
    pub clap_features: Vec<String>,
    pub vst3_subcategory: String,
    pub class_id: String,
    pub has_midi: bool,
    pub has_sidechain: bool,
    pub tail_ms: u32,
    pub latency_samples: u32,
    pub gui_framework: String,

    // ML-specific
    pub model_type: String,
    pub ml_framework: String,
    pub ml_runtime: String,

    // WASM-specific
    pub web_framework: String,
    pub has_audio_worklet: bool,
    pub has_webgpu: bool,

    // Mobile-specific
    pub platforms: Vec<String>,
    pub min_ios_version: String,
    pub min_android_sdk: u32,

    // Service-specific
    pub db_type: Option<String>,
    pub has_auth: bool,
    pub has_openapi: bool,

    // Metadata
    pub year: u16,
}

impl TemplateContext {
    pub fn new(project_name: &str, project_type: &str, org_config: &crate::config::OrgConfig) -> Self {
        let crate_name = crate::utils::naming::to_snake_case(project_name);
        let module_name = crate_name.clone();
        let struct_name = crate::utils::naming::to_pascal_case(project_name);
        let display_name = crate::utils::naming::to_display_name(project_name);
        // Derive plugin name (strip "sgm-plugin-" prefix if present)
        let plugin_name = project_name
            .strip_prefix("sgm-plugin-")
            .unwrap_or(project_name)
            .to_string();
        let plugin_display = crate::utils::naming::to_pascal_case(&plugin_name);
        let year = chrono::Utc::now().format("%Y").to_string().parse().unwrap_or(2026);

        Self {
            project_name: project_name.to_string(),
            crate_name,
            module_name,
            struct_name,
            display_name,
            plugin_name: plugin_display,
            description: format!("SGM Studios {} project", project_type),
            author: org_config.defaults.author.clone(),
            author_email: org_config.defaults.author_email.clone(),
            license: org_config.brand.license_proprietary.clone(),
            repo_url: format!("https://github.com/{}", org_config.organization.github),
            rust_version: org_config.engineering.msrv.clone(),
            edition: org_config.engineering.rust_edition.clone(),
            nih_plug_version: org_config.engineering.nih_plug_version.clone(),
            project_type: project_type.to_string(),
            plugin_category: "effect".to_string(),
            io_layout: "stereo".to_string(),
            clap_features: vec!["AudioEffect".to_string(), "Stereo".to_string()],
            vst3_subcategory: "Fx".to_string(),
            class_id: crate::utils::naming::generate_class_id(),
            has_midi: false,
            has_sidechain: false,
            tail_ms: 0,
            latency_samples: 0,
            gui_framework: "vizia".to_string(),
            model_type: "effects".to_string(),
            ml_framework: "pytorch".to_string(),
            ml_runtime: "ort".to_string(),
            web_framework: "react".to_string(),
            has_audio_worklet: true,
            has_webgpu: false,
            platforms: vec!["ios".to_string(), "android".to_string()],
            min_ios_version: org_config.defaults.min_ios_version.clone(),
            min_android_sdk: org_config.defaults.min_android_sdk,
            db_type: None,
            has_auth: false,
            has_openapi: false,
            year,
        }
    }
}