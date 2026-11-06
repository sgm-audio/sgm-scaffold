<div align="center">

# 🛠️ sgm-scaffold

**SGM Studios project scaffolding CLI**

*Generate complete, compilable, CI-ready audio projects in one command.*

[![CI](https://github.com/SGM-Studios/sgm-scaffold/actions/workflows/ci.yml/badge.svg)](https://github.com/SGM-Studios/sgm-scaffold/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/sgm-scaffold.svg)](https://crates.io/crates/sgm-scaffold)

</div>

---

## Overview

`sgm-scaffold` is a command-line tool that generates complete, production-ready project scaffolding for SGM Studios audio software. Every generated project includes:

- ✅ Compilable Rust code (passes `cargo check` immediately)
- ✅ Full CI/CD pipeline (GitHub Actions)
- ✅ Code quality gates (clippy, rustfmt, cargo-deny, cargo-audit)
- ✅ Test scaffolding (unit + integration + benchmarks)
- ✅ Documentation (README, CHANGELOG, LICENSE, AI disclosure)
- ✅ Git initialization with proper .gitignore

## Supported Project Types

| Type | Command | Description |
|------|---------|-------------|
| `plugin` | `sgm-scaffold plugin <name>` | Audio plugin (nih-plug, VST3/CLAP) |
| `library` | `sgm-scaffold library <name>` | Rust library crate |
| `tool` | `sgm-scaffold tool <name>` | CLI application (clap 4) |
| `ml-model` | `sgm-scaffold ml-model <name>` | ML model (PyTorch + Rust inference) |
| `service` | `sgm-scaffold service <name>` | Backend service (Axum 0.8) |
| `wasm` | `sgm-scaffold wasm <name>` | WebAssembly audio (AudioWorklet) |
| `mobile` | `sgm-scaffold mobile <name>` | Mobile audio app (iOS/Android) |

## Installation

### From crates.io (coming soon)
```bash
cargo install sgm-scaffold
```

### From source
```bash
git clone https://github.com/SGM-Studios/sgm-scaffold.git
cd sgm-scaffold
cargo install --path .
```

## Quick Start

```bash
# Generate a reverb plugin
sgm-scaffold plugin drift --category reverb --io stereo

# Generate a DSP library
sgm-scaffold library sgm-dsp-filters

# Generate a CLI tool
sgm-scaffold tool sgm-bench

# Generate an ML model project
sgm-scaffold ml-model stem-separator --model-type separation

# Generate a web service
sgm-scaffold service render-api --db postgres

# Generate a WASM audio project
sgm-scaffold wasm visualizer --framework react

# Generate a mobile app
sgm-scaffold mobile sgm-ios-host --platforms ios,android
```

## Plugin Categories

When generating plugins, use `--category` to customize the DSP scaffolding:

| Category | Description |
|----------|-------------|
| `eq` | Parametric equalizer |
| `compressor` | Dynamics processor |
| `reverb` | Algorithmic reverb |
| `delay` | Delay effect |
| `saturation` | Harmonic saturator |
| `filter` | Filter effect |
| `modulation` | Modulation effect |
| `utility` | Utility processor |
| `instrument` | Virtual instrument |

## Generated Project Structure (Plugin Example)

```
sgm-plugin-drift/
├── Cargo.toml              # nih-plug deps, release profile
├── src/
│   ├── lib.rs              # Plugin entry point (VST3 + CLAP)
│   ├── params.rs           # Parameter definitions
│   ├── dsp/
│   │   ├── mod.rs
│   │   └── processor.rs    # DSP processing (no alloc in RT thread)
│   ├── editor/
│   │   ├── mod.rs          # GUI (vizia/egui/iced)
│   │   └── theme.rs        # SGM Studios color tokens
│   └── util/
│       └── smoothing.rs    # Parameter smoothing
├── presets/factory/        # Factory presets
├── tests/                  # Integration + DSP accuracy tests
├── benches/                # Criterion benchmarks
├── xtask/                  # Build/bundle tasks
├── .github/workflows/      # Full CI/CD pipeline
├── deny.toml               # Cargo deny config
├── rust-toolchain.toml     # Pinned toolchain
└── README.md               # With AI disclosure
```

## Standards

All generated projects follow SGM Studios standards:
- **Rust edition 2024**, MSRV 1.88
- **Real-time safety**: No allocations, no locks, no I/O in audio thread
- **Memory safety**: Rust ownership model, Miri validation
- **Code quality**: `clippy::pedantic`, `-Wall -Wextra -Werror`
- **Testing**: >85% coverage targets, DSP accuracy tests, fuzz testing
- **Security**: `cargo audit`, `cargo deny`, `cargo vet`, SBOM generation
- **Accessibility**: WCAG 2.2 AA compliant UI
- **Compliance**: EU AI Act disclosure, GDPR-aware

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting PRs.

## Contributors

See [CONTRIBUTORS.md](CONTRIBUTORS.md) for a list of contributors. We welcome contributions! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting PRs.

## License

MIT OR Apache-2.0

## AI Disclosure

This project uses AI-assisted development. AI-generated code is reviewed by human engineers before merge. No user data is collected.

---

<div align="center">

*Built by [SGM Studios](https://github.com/SGM-Studios) — Next-Generation Audio Processing & Music Production Infrastructure*

</div>