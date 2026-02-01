# Contributing to oc_diagdoc

Thank you for your interest in contributing to oc_diagdoc! 🦀

## Development Setup

```bash
# Clone the repository
git clone https://github.com/enerBydev/oc_diagdoc.git
cd oc_diagdoc

# Build in debug mode
cargo build

# Run tests
cargo test

# Run with verbose output
cargo run -- verify ./Datos --verbose
```

## Code Standards

### Rust Style
- Follow Rust standard formatting: `cargo fmt`
- Pass all clippy lints: `cargo clippy`
- Add doc comments to all public items

### Commit Messages
Use conventional commits:
- `feat:` new feature
- `fix:` bug fix
- `docs:` documentation
- `test:` tests
- `refactor:` code refactoring

### Pull Request Process

1. Update documentation for any new features
2. Add tests for new functionality
3. Ensure `cargo test` passes
4. Ensure `cargo clippy` has no warnings
5. Update CHANGELOG.md

## Architecture Overview

See [ARCHITECTURE.md](ARCHITECTURE.md) for system design.

## License

By contributing, you agree that your contributions will be licensed under MIT.
