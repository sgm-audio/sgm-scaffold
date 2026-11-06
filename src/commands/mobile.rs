use crate::config::OrgConfig;
use crate::error::Result;
use crate::generator::{FileWriter, TemplateContext, TemplateEngine, init_git};
use crate::utils::naming::validate_name;
use crate::utils::path::dir_exists_and_nonempty;
use console::style;
use std::path::PathBuf;

#[derive(clap::Args, Debug)]
pub struct MobileArgs {
    pub name: String,
    #[arg(short = 'p', long, default_value = "ios,android")]
    pub platforms: String,
    #[arg(long, default_value = "16.0")]
    pub min_ios: String,
    #[arg(long, default_value = "26")]
    pub min_android: u32,
    #[arg(long)]
    pub force: bool,
    #[arg(long)]
    pub no_git: bool,
    #[arg(long)]
    pub dry_run: bool,
}

pub fn run_mobile(args: MobileArgs, org_config: OrgConfig, base_dir: PathBuf) -> Result<()> {
    validate_name(&args.name)?;
    let project_name = format!("sgm-mobile-{}", args.name);
    let project_dir = base_dir.join(&project_name);

    if dir_exists_and_nonempty(&project_dir) && !args.force {
        return Err(crate::error::ScaffoldError::DirExists(
            project_dir.display().to_string(),
        ));
    }

    println!("{}", style(format!("Scaffolding mobile: {}", project_name)).cyan().bold());

    let platforms: Vec<String> = args.platforms.split(',').map(|s| s.trim().to_string()).collect();
    let mut ctx = TemplateContext::new(&project_name, "mobile", &org_config);
    ctx.platforms = platforms.clone();
    ctx.min_ios_version = args.min_ios.clone();
    ctx.min_android_sdk = args.min_android;
    ctx.description = format!("SGM Studios {} mobile app", args.name);

    let writer = FileWriter::new(project_dir.clone(), args.force, args.dry_run);
    for d in &["shared/src", ".github/workflows"] {
        writer.create_dir(d)?;
    }
    if platforms.contains(&"ios".to_string()) {
        writer.create_dir("ios")?;
    }
    if platforms.contains(&"android".to_string()) {
        writer.create_dir("android")?;
    }
    let engine = TemplateEngine::new();
    let templates = [
        ("mobile/Cargo.toml.jinja", "Cargo.toml"),
        ("mobile/shared/src/lib.rs.jinja", "shared/src/lib.rs"),
        ("mobile/README.md.jinja", "README.md"),
        ("mobile/LICENSE.jinja", "LICENSE"),
    ];
    for (t, o) in &templates {
        let content = engine.render(t, &ctx)?;
        writer.create_file(o, &content)?;
    }
    if !args.no_git {
        if !args.dry_run {
            let _ = init_git(&project_dir);
        }
    }
    print_summary(&project_name, &project_dir);
    Ok(())
}

fn print_summary(name: &str, dir: &std::path::PathBuf) {
    println!();
    println!("{}", style(format!("  ✓ {} created!", name)).green().bold());
    println!("  Location: {}", dir.display());
    println!();
    println!("{}", style("Next steps:").bold());
    println!("  cd {}", name);
    println!("  cargo build (shared Rust core)");
    println!("  Open ios/ in Xcode or android/ in Android Studio");
    println!();
}