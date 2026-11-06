use crate::config::OrgConfig;
use crate::error::Result;
use crate::generator::{FileWriter, TemplateContext, TemplateEngine, init_git, validate_project};
use crate::utils::naming::validate_name;
use crate::utils::path::dir_exists_and_nonempty;
use console::style;
use std::path::PathBuf;

#[derive(clap::Args, Debug)]
pub struct LibraryArgs {
    pub name: String,

    #[arg(long)]
    pub public_api: bool,

    #[arg(long)]
    pub no_std: bool,

    #[arg(long)]
    pub wasm: bool,

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

pub fn run_library(args: LibraryArgs, org_config: OrgConfig, base_dir: PathBuf) -> Result<()> {
    validate_name(&args.name)?;
    let project_name = if args.name.starts_with("sgm-") {
        args.name.clone()
    } else {
        format!("sgm-{}", args.name)
    };
    let project_dir = base_dir.join(&project_name);

    if dir_exists_and_nonempty(&project_dir) && !args.force {
        return Err(crate::error::ScaffoldError::DirExists(
            project_dir.display().to_string(),
        ));
    }

    println!("{}", style(format!("Scaffolding library: {}", project_name)).cyan().bold());

    let mut ctx = TemplateContext::new(&project_name, "library", &org_config);
    ctx.description = format!("SGM Studios {} library", args.name);

    let writer = FileWriter::new(project_dir.clone(), args.force, args.dry_run);
    create_dirs(&writer)?;
    let engine = TemplateEngine::new();
    render(&engine, &ctx, &writer, &args)?;
    post_gen(&project_dir, &args)?;
    print_summary(&project_name, &project_dir);
    Ok(())
}

fn create_dirs(w: &FileWriter) -> Result<()> {
    for d in &["src", "tests", "benches", "examples", ".github/workflows"] {
        w.create_dir(d)?;
    }
    Ok(())
}

fn render(e: &TemplateEngine, ctx: &TemplateContext, w: &FileWriter, args: &LibraryArgs) -> Result<()> {
    let templates = [
        ("library/Cargo.toml.jinja", "Cargo.toml"),
        ("library/src/lib.rs.jinja", "src/lib.rs"),
        ("library/src/error.rs.jinja", "src/error.rs"),
        ("library/tests/integration.rs.jinja", "tests/integration.rs"),
        ("library/benches/bench.rs.jinja", "benches/bench.rs"),
        ("library/.gitignore.jinja", ".gitignore"),
        ("library/deny.toml.jinja", "deny.toml"),
        ("library/rust-toolchain.toml.jinja", "rust-toolchain.toml"),
        ("library/.cargo/config.toml.jinja", ".cargo/config.toml"),
        ("library/README.md.jinja", "README.md"),
        ("library/CHANGELOG.md.jinja", "CHANGELOG.md"),
        ("library/LICENSE.jinja", "LICENSE"),
    ];
    for (t, o) in &templates {
        let content = e.render(t, &ctx)?;
        w.create_file(o, &content)?;
    }
    if !args.no_ci {
        let content = e.render("library/.github/workflows/ci.yml.jinja", &ctx)?;
        w.create_file(".github/workflows/ci.yml", &content)?;
    }
    Ok(())
}

fn post_gen(dir: &std::path::PathBuf, args: &LibraryArgs) -> Result<()> {
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
    println!("  cargo build");
    println!("  cargo test");
    println!();
}