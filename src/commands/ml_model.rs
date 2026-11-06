use crate::config::OrgConfig;
use crate::error::Result;
use crate::generator::{FileWriter, TemplateContext, TemplateEngine, init_git, validate_project};
use crate::utils::naming::validate_name;
use crate::utils::path::dir_exists_and_nonempty;
use console::style;
use std::path::PathBuf;

#[derive(clap::Args, Debug)]
pub struct MlModelArgs {
    pub name: String,
    #[arg(short = 't', long, default_value = "effects")]
    pub model_type: String,
    #[arg(long, default_value = "pytorch")]
    pub framework: String,
    #[arg(long, default_value = "ort")]
    pub runtime: String,
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

pub fn run_ml_model(args: MlModelArgs, org_config: OrgConfig, base_dir: PathBuf) -> Result<()> {
    validate_name(&args.name)?;
    let project_name = format!("sgm-ml-{}", args.name);
    let project_dir = base_dir.join(&project_name);

    if dir_exists_and_nonempty(&project_dir) && !args.force {
        return Err(crate::error::ScaffoldError::DirExists(
            project_dir.display().to_string(),
        ));
    }

    println!("{}", style(format!("Scaffolding ML model: {}", project_name)).cyan().bold());

    let mut ctx = TemplateContext::new(&project_name, "ml-model", &org_config);
    ctx.model_type = args.model_type.clone();
    ctx.ml_framework = args.framework.clone();
    ctx.ml_runtime = args.runtime.clone();
    ctx.description = format!("SGM Studios {} ML model", args.name);

    let writer = FileWriter::new(project_dir.clone(), args.force, args.dry_run);
    for d in &["src", "python", "models", "tests", ".github/workflows"] {
        writer.create_dir(d)?;
    }
    let engine = TemplateEngine::new();
    let templates = [
        ("ml-model/Cargo.toml.jinja", "Cargo.toml"),
        ("ml-model/src/lib.rs.jinja", "src/lib.rs"),
        ("ml-model/src/model.rs.jinja", "src/model.rs"),
        ("ml-model/src/preprocessing.rs.jinja", "src/preprocessing.rs"),
        ("ml-model/python/train.py.jinja", "python/train.py"),
        ("ml-model/python/model.py.jinja", "python/model.py"),
        ("ml-model/python/requirements.txt.jinja", "python/requirements.txt"),
        ("ml-model/MODEL_CARD.md.jinja", "MODEL_CARD.md"),
        ("ml-model/.gitignore.jinja", ".gitignore"),
        ("ml-model/README.md.jinja", "README.md"),
        ("ml-model/LICENSE.jinja", "LICENSE"),
    ];
    for (t, o) in &templates {
        let content = engine.render(t, &ctx)?;
        writer.create_file(o, &content)?;
    }
    // Create .gitkeep in models/
    writer.create_file("models/.gitkeep", "")?;
    post_gen(&project_dir, &args)?;
    print_summary(&project_name, &project_dir);
    Ok(())
}

fn post_gen(dir: &std::path::PathBuf, args: &MlModelArgs) -> Result<()> {
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
    println!("  cd python && pip install -r requirements.txt");
    println!("  python train.py");
    println!();
}