use crate::config::OrgConfig;
use crate::error::Result;
use crate::generator::{FileWriter, TemplateContext, TemplateEngine, init_git, validate_project};
use crate::utils::naming::{to_pascal_case, validate_name};
use crate::utils::path::{detect_workspace, dir_exists_and_nonempty};
use console::style;
use std::path::PathBuf;

#[derive(clap::Args, Debug)]
pub struct PluginArgs {
    /// Plugin name (kebab-case)
    pub name: String,

    /// Plugin category
    #[arg(short = 'c', long, default_value = "effect")]
    pub category: String,

    /// I/O layout
    #[arg(short = 'i', long, default_value = "stereo")]
    pub io: String,

    /// GUI framework
    #[arg(short = 'g', long, default_value = "vizia")]
    pub gui: String,

    /// Enable MIDI input
    #[arg(long)]
    pub midi: bool,

    /// Enable sidechain input
    #[arg(long)]
    pub sidechain: bool,

    /// Tail time in ms
    #[arg(long, default_value = "0")]
    pub tail: u32,

    /// Plugin latency in samples
    #[arg(long, default_value = "0")]
    pub latency: u32,

    /// Preset format
    #[arg(long, default_value = "ron")]
    pub preset_format: String,

    /// CLAP only (no VST3)
    #[arg(long)]
    pub clap_only: bool,

    /// VST3 only (no CLAP)
    #[arg(long)]
    pub vst3_only: bool,

    /// Also generate AU (macOS)
    #[arg(long)]
    pub au: bool,

    // Global flags passed through
    #[arg(long)]
    pub force: bool,

    #[arg(long)]
    pub no_git: bool,

    #[arg(long)]
    pub no_ci: bool,

    #[arg(long)]
    pub no_validate: bool,

    #[arg(long)]
    pub dry_run: bool,
}

pub fn run_plugin(args: PluginArgs, org_config: OrgConfig, base_dir: PathBuf) -> Result<()> {
    // 1. Validate name
    validate_name(&args.name)?;

    let project_name = format!("sgm-plugin-{}", args.name);
    let project_dir = base_dir.join(&project_name);

    // 2. Check if directory exists
    if dir_exists_and_nonempty(&project_dir) && !args.force {
        return Err(crate::error::ScaffoldError::DirExists(
            project_dir.display().to_string(),
        ));
    }

    // 3. Detect workspace
    let workspace_root = detect_workspace(&base_dir);
    let in_workspace = workspace_root.is_some();

    println!(
        "{}",
        style(format!("Scaffolding plugin: {}", project_name)).cyan().bold()
    );
    println!("  Category: {}", args.category);
    println!("  I/O: {}", args.io);
    println!("  GUI: {}", args.gui);
    if in_workspace {
        println!("  Workspace detected: {}", workspace_root.as_ref().unwrap().display());
    }
    println!();

    // 4. Build template context
    let mut ctx = TemplateContext::new(&project_name, "plugin", &org_config);
    ctx.plugin_category = args.category.clone();
    ctx.io_layout = args.io.clone();
    ctx.gui_framework = args.gui.clone();
    ctx.has_midi = args.midi;
    ctx.has_sidechain = args.sidechain;
    ctx.tail_ms = args.tail;
    ctx.latency_samples = args.latency;

    // Set CLAP features based on category and I/O
    ctx.clap_features = build_clap_features(&args.category, &args.io, args.midi);
    ctx.vst3_subcategory = category_to_vst3_subcategory(&args.category);

    // Set description
    ctx.description = format!(
        "SGM Studios {} plugin — {}",
        to_pascal_case(&args.name),
        category_description(&args.category)
    );

    // 5. Create directory structure
    let writer = FileWriter::new(project_dir.clone(), args.force, args.dry_run);
    create_directory_structure(&writer)?;

    // 6. Render and write templates
    let engine = TemplateEngine::new();
    render_templates(&engine, &ctx, &writer, &args)?;

    // 7. Post-generation
    if !args.dry_run {
        // Format generated Rust files
        let _ = std::process::Command::new("cargo")
            .args(["fmt"])
            .current_dir(&project_dir)
            .output();

        // Validate
        if !args.no_validate {
            println!("{}", style("Validating project...").cyan());
            match validate_project(&project_dir) {
                Ok(()) => {
                    println!("{}", style("✓ Project compiles successfully").green().bold());
                }
                Err(e) => {
                    println!("{}", style(format!("⚠ Validation warning: {}", e)).yellow());
                    println!("  The project was generated but may need manual fixes.");
                }
            }
        }

        // Git init
        if !args.no_git {
            match init_git(&project_dir) {
                Ok(()) => println!("{}", style("✓ Git repository initialized").green()),
                Err(e) => println!("{}", style(format!("⚠ Git init failed: {}", e)).yellow()),
            }
        }

        // Update workspace if detected
        if in_workspace {
            if let Some(ws_root) = &workspace_root {
                update_workspace_members(ws_root, &project_name)?;
            }
        }
    }

    // 8. Print summary
    print_summary(&project_name, &project_dir, &args);

    Ok(())
}

fn create_directory_structure(writer: &FileWriter) -> Result<()> {
    let dirs = [
        "src",
        "src/dsp",
        "src/editor",
        "src/editor/components",
        "src/util",
        "presets/factory",
        "tests",
        "benches",
        "xtask/src",
    ];

    for dir in &dirs {
        writer.create_dir(dir)?;
    }

    // CI directory (optional, created by template writer)
    writer.create_dir(".github/workflows")?;

    Ok(())
}

fn render_templates(
    engine: &TemplateEngine,
    ctx: &TemplateContext,
    writer: &FileWriter,
    args: &PluginArgs,
) -> Result<()> {
    // Core files
    let core_templates = [
        ("plugin/Cargo.toml.jinja", "Cargo.toml"),
        ("plugin/src/lib.rs.jinja", "src/lib.rs"),
        ("plugin/src/params.rs.jinja", "src/params.rs"),
        ("plugin/src/dsp/mod.rs.jinja", "src/dsp/mod.rs"),
        ("plugin/src/dsp/processor.rs.jinja", "src/dsp/processor.rs"),
        ("plugin/src/editor/mod.rs.jinja", "src/editor/mod.rs"),
        ("plugin/src/editor/theme.rs.jinja", "src/editor/theme.rs"),
        ("plugin/src/editor/components/mod.rs.jinja", "src/editor/components/mod.rs"),
        ("plugin/src/util/mod.rs.jinja", "src/util/mod.rs"),
        ("plugin/src/util/smoothing.rs.jinja", "src/util/smoothing.rs"),
        ("plugin/tests/integration.rs.jinja", "tests/integration.rs"),
        ("plugin/tests/dsp_accuracy.rs.jinja", "tests/dsp_accuracy.rs"),
        ("plugin/benches/processing.rs.jinja", "benches/processing.rs"),
        ("plugin/xtask/Cargo.toml.jinja", "xtask/Cargo.toml"),
        ("plugin/xtask/src/main.rs.jinja", "xtask/src/main.rs"),
        ("plugin/.gitignore.jinja", ".gitignore"),
        ("plugin/deny.toml.jinja", "deny.toml"),
        ("plugin/rust-toolchain.toml.jinja", "rust-toolchain.toml"),
        ("plugin/.cargo/config.toml.jinja", ".cargo/config.toml"),
        ("plugin/README.md.jinja", "README.md"),
        ("plugin/CHANGELOG.md.jinja", "CHANGELOG.md"),
        ("plugin/LICENSE.jinja", "LICENSE"),
        ("plugin/presets/factory/default.sgmpreset.jinja", "presets/factory/default.sgmpreset"),
    ];

    for (template, output) in &core_templates {
        let content = engine.render(template, ctx)?;
        writer.create_file(output, &content)?;
    }

    // CI workflow (optional)
    if !args.no_ci {
        let content = engine.render("plugin/.github/workflows/ci.yml.jinja", ctx)?;
        writer.create_file(".github/workflows/ci.yml", &content)?;
    }

    Ok(())
}

fn build_clap_features(category: &str, io: &str, midi: bool) -> Vec<String> {
    let mut features = vec!["AudioEffect".to_string()];

    match io {
        "stereo" => features.push("Stereo".to_string()),
        "mono" => features.push("Mono".to_string()),
        "surround-5.1" => features.push("Surround".to_string()),
        "surround-7.1" => features.push("Surround".to_string()),
        _ => features.push("Stereo".to_string()),
    }

    match category {
        "reverb" => features.push("Reverb".to_string()),
        "delay" => features.push("Delay".to_string()),
        "eq" => features.push("Equalizer".to_string()),
        "compressor" => features.push("Compressor".to_string()),
        "filter" => features.push("Filter".to_string()),
        "modulation" => features.push("Modulation".to_string()),
        "instrument" => {
            features[0] = "Instrument".to_string();
            features.push("Synthesizer".to_string());
        }
        _ => {}
    }

    if midi {
        features.push("NotePorts".to_string());
    }

    features
}

fn category_to_vst3_subcategory(category: &str) -> String {
    match category {
        "instrument" => "Instrument".to_string(),
        "eq" => "Eq".to_string(),
        "compressor" => "Dynamics".to_string(),
        "reverb" => "Reverb".to_string(),
        "delay" => "Delay".to_string(),
        "saturation" => "Distortion".to_string(),
        "filter" => "Filter".to_string(),
        "modulation" => "Modulation".to_string(),
        "utility" => "Fx".to_string(),
        _ => "Fx".to_string(),
    }
}

fn category_description(category: &str) -> &'static str {
    match category {
        "eq" => "parametric equalizer",
        "compressor" => "dynamics processor",
        "reverb" => "algorithmic reverb",
        "delay" => "delay effect",
        "saturation" => "harmonic saturator",
        "filter" => "filter effect",
        "modulation" => "modulation effect",
        "utility" => "utility processor",
        "instrument" => "virtual instrument",
        _ => "audio effect",
    }
}

fn update_workspace_members(workspace_root: &PathBuf, project_name: &str) -> Result<()> {
    let cargo_toml_path = workspace_root.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml_path)?;
    
    // Simple approach: find the members array and add the new crate
    // This is a basic implementation; a proper one would use a TOML parser
    if content.contains(&format!("\"{}\"", project_name)) {
        return Ok(()); // Already a member
    }

    // Find the members line and append
    let new_content = content.replace(
        "members = [",
        &format!("members = [\n    \"crates/{}\",", project_name),
    );

    std::fs::write(&cargo_toml_path, new_content)?;
    println!(
        "{}",
        style(format!("  Added {} to workspace members", project_name)).green()
    );
    Ok(())
}

fn print_summary(project_name: &str, project_dir: &PathBuf, args: &PluginArgs) {
    println!();
    println!("{}", style("═══════════════════════════════════════").cyan());
    println!(
        "{}",
        style(format!("  ✓ {} created!", project_name)).green().bold()
    );
    println!("{}", style("═══════════════════════════════════════").cyan());
    println!();
    println!("  Location: {}", project_dir.display());
    println!("  Category: {}", args.category);
    println!("  I/O:      {}", args.io);
    println!("  GUI:      {}", args.gui);
    println!();
    println!("{}", style("Next steps:").bold());
    println!("  cd {}", project_name);
    println!("  cargo build");
    println!("  cargo test");
    println!("  cargo run --example (if applicable)");
    println!();
    println!("{}", style("Validation:").bold());
    println!("  cargo clippy -- -D warnings");
    println!("  cargo nextest run");
    println!();
}