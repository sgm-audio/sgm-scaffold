use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "sgm-scaffold")]
#[command(about = "SGM Studios project scaffolding CLI")]
#[command(version = "0.1.0")]
pub struct Cli {
    /// Path to sgm-studios.toml config file
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// Overwrite existing directory
    #[arg(long, global = true)]
    pub force: bool,

    /// Skip git init
    #[arg(long, global = true)]
    pub no_git: bool,

    /// Skip GitHub Actions workflow generation
    #[arg(long, global = true)]
    pub no_ci: bool,

    /// Skip cargo check after generation
    #[arg(long, global = true)]
    pub no_validate: bool,

    /// Print what would be generated without writing
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Show detailed output
    #[arg(long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate an audio plugin project (nih-plug, VST3/CLAP)
    Plugin {
        /// Plugin name (kebab-case)
        name: String,

        /// Plugin category
        #[arg(short = 'c', long, default_value = "effect")]
        category: String,

        /// I/O layout
        #[arg(short = 'i', long, default_value = "stereo")]
        io: String,

        /// GUI framework
        #[arg(short = 'g', long, default_value = "vizia")]
        gui: String,

        /// Enable MIDI input
        #[arg(long)]
        midi: bool,

        /// Enable sidechain input
        #[arg(long)]
        sidechain: bool,

        /// Tail time in ms
        #[arg(long, default_value = "0")]
        tail: u32,

        /// Plugin latency in samples
        #[arg(long, default_value = "0")]
        latency: u32,

        /// Preset format
        #[arg(long, default_value = "ron")]
        preset_format: String,

        /// CLAP only (no VST3)
        #[arg(long)]
        clap_only: bool,

        /// VST3 only (no CLAP)
        #[arg(long)]
        vst3_only: bool,

        /// Also generate AU (macOS)
        #[arg(long)]
        au: bool,
    },

    /// Generate a Rust library crate
    Library {
        name: String,
        #[arg(long)]
        public_api: bool,
        #[arg(long)]
        no_std: bool,
        #[arg(long)]
        wasm: bool,
    },

    /// Generate a CLI tool
    Tool {
        name: String,
        #[arg(long)]
        subcommands: bool,
        #[arg(long)]
        config_file: bool,
    },

    /// Generate an ML model project
    MlModel {
        name: String,
        #[arg(short = 't', long, default_value = "effects")]
        model_type: String,
        #[arg(long, default_value = "pytorch")]
        framework: String,
        #[arg(long, default_value = "ort")]
        runtime: String,
    },

    /// Generate a backend service
    Service {
        name: String,
        #[arg(short = 'd', long, default_value = "none")]
        db: String,
        #[arg(long)]
        auth: bool,
        #[arg(long)]
        openapi: bool,
    },

    /// Generate a WebAssembly audio project
    Wasm {
        name: String,
        #[arg(short = 'f', long, default_value = "react")]
        framework: String,
        #[arg(long)]
        webgpu: bool,
    },

    /// Generate a mobile audio app
    Mobile {
        name: String,
        #[arg(short = 'p', long, default_value = "ios,android")]
        platforms: String,
    },
}