use crate::config::OrgConfig;
use crate::error::Result;
use crate::generator::{FileWriter, TemplateContext, TemplateEngine, init_git, validate_project};
use crate::utils::naming::validate_name;
use crate::utils::path::dir_exists_and_nonempty;
use console::style;
use std::path::PathBuf;

#[derive(clap::Args, Debug)]
pub struct ServiceArgs {
    pub name: String,
    #[arg(short = 'd', long, default_value = "none")]
    pub db: String,
    #[arg(long)]
    pub auth: bool,
    #[arg(long, default_value = "true")]
    pub docker: bool,
    #[arg(long)]
    pub openapi: bool,
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

pub fn run_service(args: ServiceArgs, org_config: OrgConfig, base_dir: PathBuf) -> Result<()> {
    validate_name(&args.name)?;
    let project_name = format!("sgm-service-{}", args.name);
    let project_dir = base_dir.join(&project_name);

    if dir_exists_and_nonempty(&project_dir) && !args.force {
        return Err(crate::error::ScaffoldError::DirExists(
            project_dir.display().to_string(),
        ));
    }

    println!("{}", style(format!("Scaffolding service: {}", project_name)).cyan().bold());

    let mut ctx = TemplateContext::new(&project_name, "service", &org_config);
    ctx.db_type = if args.db == "none" { None } else { Some(args.db.clone()) };
    ctx.has_auth = args.auth;
    ctx.has_openapi = args.openapi;
    ctx.description = format!("SGM Studios {} service", args.name);

    let writer = FileWriter::new(project_dir.clone(), args.force, args.dry_run);
    for d in &["src", "src/routes", "src/models", ".github/workflows"] {
        writer.create_dir(d)?;
    }
    let engine = TemplateEngine::new();
    let templates = [
        ("service/Cargo.toml.jinja", "Cargo.toml"),
        ("service/src/main.rs.jinja", "src/main.rs"),
        ("service/src/routes/mod.rs.jinja", "src/routes/mod.rs"),
        ("service/.gitignore.jinja", ".gitignore"),
        ("service/deny.toml.jinja", "deny.toml"),
        ("service/rust-toolchain.toml.jinja", "rust-toolchain.toml"),
        ("service/.cargo/config.toml.jinja", ".cargo/config.toml"),
        ("service/README.md.jinja", "README.md"),
        ("service/CHANGELOG.md.jinja", "CHANGELOG.md"),
        ("service/LICENSE.jinja", "LICENSE"),
    ];
    for (t, o) in &templates {
        let content = engine.render(t, &ctx)?;
        writer.create_file(o, &content)?;
    }
    if !args.no_ci {
        let content = engine.render("service/.github/workflows/ci.yml.jinja", &ctx)?;
        writer.create_file(".github/workflows/ci.yml", &content)?;
    }
    if args.docker {
        let content = engine.render("service/Dockerfile.jinja", &ctx)?;
        writer.create_file("Dockerfile", &content)?;
    }
    post_gen(&project_dir, &args)?;
    print_summary(&project_name, &project_dir);
    Ok(())
}

fn post_gen(dir: &std::path::PathBuf, args: &ServiceArgs) -> Result<()> {
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
    println!("  cargo run");
    println!();
}