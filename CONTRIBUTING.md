# Contributing to sgm-scaffold

Thank you for your interest in contributing to `sgm-scaffold`! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Reporting Issues](#reporting-issues)

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Create a new branch for your changes

```bash
git clone https://github.com/YOUR_USERNAME/sgm-scaffold.git
cd sgm-scaffold
git checkout -b feature/your-feature-name
```

## Development Setup

### Prerequisites
- Rust 1.88+ (install via [rustup](https://rustup.rs))
- Git

### Build
```bash
cargo build
```

### Run
```bash
cargo run -- plugin test-project --category eq
```

### Test
```bash
cargo test
```

### Lint
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Making Changes

### Project Structure
```
sgm-scaffold/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli.rs               # Clap argument definitions
│   ├── commands/            # Command handlers (one per project type)
│   ├── config/              # Org config loader
│   ├── generator/           # Template engine, file writer, validation
│   └── utils/               # Naming, paths, prompts
├── templates/               # Jinja2 templates (one dir per project type)
└── tests/                   # Integration tests
```

### Adding a New Project Type

1. Create a new command module in `src/commands/`
2. Add the command to `src/cli.rs` (enum variant) and `src/main.rs` (match arm)
3. Create template directory under `templates/`
4. Add templates following the existing pattern
5. Update `src/commands/mod.rs` to export the new module
6. Add tests

### Template Guidelines
- Templates use MiniJinja (Jinja2-compatible) syntax
- All templates receive a `TemplateContext` with standard fields
- Use `{% raw %}` blocks for literal `{{ }}` (e.g., GitHub Actions expressions)
- Generated code must pass `cargo check` and `cargo test`
- Include SGM Studios license headers in all generated files

### Code Style
- Follow Rust standard formatting (`rustfmt`)
- All public items must have doc comments (`#![deny(missing_docs)]` where applicable)
- No warnings allowed (`-D warnings`)
- Use `thiserror` for error types
- Prefer `anyhow` for application-level errors

## Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests (manual)
Generate each project type and verify compilation:
```bash
for type in plugin library tool ml-model service wasm mobile; do
    cargo run -- $type test-$type --no-git --no-validate
    cd sgm-$type-test-$type 2>/dev/null || cd sgm-plugin-test-$type 2>/dev/null
    cargo check && cargo test
    cd ..
done
```

### Template Testing
When modifying templates, always regenerate and verify:
```bash
cargo run -- plugin test-template --category eq --no-git
cd sgm-plugin-test-template
cargo check && cargo test && cargo clippy -- -D warnings
```

## Submitting a Pull Request

1. Ensure all tests pass: `cargo test`
2. Ensure no warnings: `cargo clippy -- -D warnings`
3. Ensure formatting: `cargo fmt --all -- --check`
4. Update documentation if needed
5. Update CHANGELOG.md with your changes
6. Push to your fork and create a PR

### PR Title Format
Use conventional commits:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation only
- `refactor:` Code refactoring
- `test:` Adding tests
- `chore:` Maintenance

### Review Process
- All PRs require at least one review
- CI must pass (formatting, clippy, tests, build)
- Squash merge is preferred

## Reporting Issues

Use [GitHub Issues](https://github.com/sgm-audio/sgm-scaffold/issues) to report bugs or request features.

### Bug Reports
Include:
- Steps to reproduce
- Expected behavior
- Actual behavior
- Rust version (`rustc --version`)
- OS and architecture

### Feature Requests
Include:
- Use case description
- Proposed solution
- Alternatives considered

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 license.

---

## Contributors

See [CONTRIBUTORS.md](CONTRIBUTORS.md) for a list of contributors.