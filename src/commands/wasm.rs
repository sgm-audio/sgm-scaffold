use crate::config::OrgConfig;
use crate::error::Result;
use crate::generator::{FileWriter, TemplateContext, TemplateEngine, init_git, validate_project};
use crate::utils::naming::validate_name;
use crate::utils::path::dir_exists_and_nonempty;
use console::style;
use std::path::PathBuf;

#[derive(clap::Args, Debug)]
pub struct WasmArgs {
    pub name: String,
    #[arg(short = 'f', long, default_value = "react")]
    pub framework: String,
    #[arg(long, default_value = "true")]
    pub audio_worklet: bool,
    #[arg(long)]
    pub webgpu: bool,
    #[arg(long)]
    pub pwa: bool,
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

pub fn run_wasm(args: WasmArgs, org_config: OrgConfig, base_dir: PathBuf) -> Result<()> {
    validate_name(&args.name)?;
    let project_name = format!("sgm-wasm-{}", args.name);
    let project_dir = base_dir.join(&project_name);

    if dir_exists_and_nonempty(&project_dir) && !args.force {
        return Err(crate::error::ScaffoldError::DirExists(
            project_dir.display().to_string(),
        ));
    }

    println!("{}", style(format!("Scaffolding WASM: {}", project_name)).cyan().bold());

    let mut ctx = TemplateContext::new(&project_name, "wasm", &org_config);
    ctx.web_framework = args.framework.clone();
    ctx.has_audio_worklet = args.audio_worklet;
    ctx.has_webgpu = args.webgpu;
    ctx.description = format!("SGM Studios {} web audio", args.name);

    let writer = FileWriter::new(project_dir.clone(), args.force, args.dry_run);
    for d in &["src", "web", ".github/workflows"] {
        writer.create_dir(d)?;
    }
    let engine = TemplateEngine::new();
    let templates = [
        ("wasm/Cargo.toml.jinja", "Cargo.toml"),
        ("wasm/src/lib.rs.jinja", "src/lib.rs"),
        ("wasm/.gitignore.jinja", ".gitignore"),
        ("wasm/rust-toolchain.toml.jinja", "rust-toolchain.toml"),
        ("wasm/README.md.jinja", "README.md"),
        ("wasm/LICENSE.jinja", "LICENSE"),
    ];
    for (t, o) in &templates {
        let content = engine.render(t, &ctx)?;
        writer.create_file(o, &content)?;
    }
    post_gen(&project_dir, &args)?;
    print_summary(&project_name, &project_dir);
    Ok(())
}

fn post_gen(dir: &std::path::PathBuf, args: &WasmArgs) -> Result<()> {
    if !args.dry_run {
        let _ = std::process::Command::new("cargo").args(["fmt"]).current_dir(dir).output();
        if !args.no_validate {
            let _ = validate_project(dir);
        }
        if !args.no_git {
            let _ = init_git(dir);
        }
    }
    Ok(())
}

fn print_summary(name: &str, dir: &std::path::PathBuf) {
    println!();
    println!("{}", style(format!("  ✓ {} created!", name)).green().bold());
    println!("  Location: {}", dir.display());
    println!();
    println!("{}", style("Next steps:").bold());
    println!("  cd {}", name);
    println!("  cargo build --target wasm32-unknown-unknown");
    println!("  wasm-pack build --target web");
    println!();
}