
# Contributing to Stasis Engine

Thank you for your interest in contributing to Stasis Engine.

## Development setup

1. Clone the repository
2. Install Rust via rustup
3. Run tests:

```bash
cargo test --workspace
```

4. Run linting:

```bash
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

## Code style

- Follow standard Rust formatting (rustfmt)
- All public APIs must have rustdoc comments
- No clippy warnings allowed
- Write tests for new functionality

## Pull request process

1. Fork the repository
2. Create a feature branch from main
3. Make your changes with tests
4. Ensure cargo test, cargo clippy, and cargo fmt pass
5. Submit a pull request with a clear description

## Reporting issues

Use GitHub Issues. Include:
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment (OS, Rust version)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
