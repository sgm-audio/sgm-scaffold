mod cli;
mod commands;
mod config;
mod error;
mod generator;
mod utils;

use crate::cli::{Cli, Commands};
use clap::Parser;
use crate::config::load_config;
use crate::error::Result;
use console::style;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("{}", style(format!("Error: {}", e)).red().bold());
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let org_config = load_config(cli.config.as_deref())?;
    let base_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    match cli.command {
        Commands::Plugin { name, category, io, gui, midi, sidechain, tail, latency, .. } => {
            let args = commands::plugin::PluginArgs {
                name, category, io, gui, midi, sidechain, tail, latency,
                preset_format: "ron".to_string(),
                clap_only: false, vst3_only: false, au: false,
                force: cli.force, no_git: cli.no_git, no_ci: cli.no_ci,
                no_validate: cli.no_validate, dry_run: cli.dry_run,
            };
            commands::plugin::run_plugin(args, org_config, base_dir)
        }

        Commands::Library { name, .. } => {
            let args = commands::library::LibraryArgs {
                name, public_api: false, no_std: false, wasm: false,
                force: cli.force, no_git: cli.no_git, no_ci: cli.no_ci,
                no_validate: cli.no_validate, dry_run: cli.dry_run,
            };
            commands::library::run_library(args, org_config, base_dir)
        }

        Commands::Tool { name, .. } => {
            let args = commands::tool::ToolArgs {
                name, subcommands: false, config_file: false,
                force: cli.force, no_git: cli.no_git, no_ci: cli.no_ci,
                no_validate: cli.no_validate, dry_run: cli.dry_run,
            };
            commands::tool::run_tool(args, org_config, base_dir)
        }

        Commands::MlModel { name, model_type, framework, runtime } => {
            let args = commands::ml_model::MlModelArgs {
                name, model_type, framework, runtime,
                force: cli.force, no_git: cli.no_git, no_ci: cli.no_ci,
                no_validate: cli.no_validate, dry_run: cli.dry_run,
            };
            commands::ml_model::run_ml_model(args, org_config, base_dir)
        }

        Commands::Service { name, db, auth, openapi } => {
            let args = commands::service::ServiceArgs {
                name, db, auth, openapi, docker: true,
                force: cli.force, no_git: cli.no_git, no_ci: cli.no_ci,
                no_validate: cli.no_validate, dry_run: cli.dry_run,
            };
            commands::service::run_service(args, org_config, base_dir)
        }

        Commands::Wasm { name, framework, webgpu } => {
            let args = commands::wasm::WasmArgs {
                name, framework, audio_worklet: true, webgpu, pwa: false,
                force: cli.force, no_git: cli.no_git, no_ci: cli.no_ci,
                no_validate: cli.no_validate, dry_run: cli.dry_run,
            };
            commands::wasm::run_wasm(args, org_config, base_dir)
        }

        Commands::Mobile { name, platforms } => {
            let args = commands::mobile::MobileArgs {
                name, platforms, min_ios: "16.0".to_string(), min_android: 26,
                force: cli.force, no_git: cli.no_git, dry_run: cli.dry_run,
            };
            commands::mobile::run_mobile(args, org_config, base_dir)
        }
    }
}