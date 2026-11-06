use crate::config::OrgConfig;
use crate::error::Result;
use crate::generator::{FileWriter, TemplateContext, TemplateEngine, init_git, validate_project};
use crate::utils::naming::validate_name;
use crate::utils::path::dir_exists_and_nonempty;
use console::style;
use std::path::PathBuf;

#[derive(clap::Args, Debug)]
pub struct ToolArgs {
    pub name: String,
    #[arg(long)]
    pub subcommands: bool,
    #[arg(long)]
    pub config_file: bool,
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

pub fn run_tool(args: ToolArgs, org_config: OrgConfig, base_dir: PathBuf) -> Result<()> {
    validate_name(&args.name)?;
    let project_name = if args.name.starts_with("sgm-tool-") {
        args.name.clone()
    } else {
        format!("sgm-tool-{}", args.name)
    };
    let project_dir = base_dir.join(&project_name);

    if dir_exists_and_nonempty(&project_dir) && !args.force {
        return Err(crate::error::ScaffoldError::DirExists(
            project_dir.display().to_string(),
        ));
    }

    println!("{}", style(format!("Scaffolding tool: {}", project_name)).cyan().bold());

    let mut ctx = TemplateContext::new(&project_name, "tool", &org_config);
    ctx.description = format!("SGM Studios {} CLI tool", args.name);

    let writer = FileWriter::new(project_dir.clone(), args.force, args.dry_run);
    for d in &["src", "src/commands", ".github/workflows"] {
        writer.create_dir(d)?;
    }
    let engine = TemplateEngine::new();
    let templates = [
        ("tool/Cargo.toml.jinja", "Cargo.toml"),
        ("tool/src/main.rs.jinja", "src/main.rs"),
        ("tool/src/cli.rs.jinja", "src/cli.rs"),
        ("tool/src/commands/mod.rs.jinja", "src/commands/mod.rs"),
        ("tool/.gitignore.jinja", ".gitignore"),
        ("tool/deny.toml.jinja", "deny.toml"),
        ("tool/rust-toolchain.toml.jinja", "rust-toolchain.toml"),
        ("tool/.cargo/config.toml.jinja", ".cargo/config.toml"),
        ("tool/README.md.jinja", "README.md"),
        ("tool/CHANGELOG.md.jinja", "CHANGELOG.md"),
        ("tool/LICENSE.jinja", "LICENSE"),
    ];
    for (t, o) in &templates {
        let content = engine.render(t, &ctx)?;
        writer.create_file(o, &content)?;
    }
    if !args.no_ci {
        let content = engine.render("tool/.github/workflows/ci.yml.jinja", &ctx)?;
        writer.create_file(".github/workflows/ci.yml", &content)?;
    }
    post_gen(&project_dir, &args)?;
    print_summary(&project_name, &project_dir);
    Ok(())
}

fn post_gen(dir: &std::path::PathBuf, args: &ToolArgs) -> Result<()> {
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
    println!("  cargo run -- --help");
    println!();
}