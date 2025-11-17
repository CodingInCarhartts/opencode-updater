# Contributing

Thank you for your interest in contributing to opencode-updater! This document outlines the process and standards for contributions.

## Development Setup

1. Ensure you have Rust installed (version 1.70+ recommended).
2. Clone the repository: `git clone https://github.com/CodingInCarhartts/opencode-updater.git`
3. Build the project: `cargo build`
4. Run tests: `cargo test`

## Code Standards

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) and [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/index.html).
- Use `rustfmt` for code formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Write unit and integration tests for new features.
- Keep functions small and focused; add documentation comments for public APIs.

## Development Process

1. Fork the repository and create a feature branch from `main`.
2. Make changes, ensuring tests pass and code is formatted/linted.
3. Update `CHANGELOG.md` for any user-facing changes.
4. Commit with clear, descriptive messages following [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) (e.g., `feat(updater): Add checksum verification`).
5. Open a pull request with a detailed description of changes and why they're needed.
6. Address review feedback and ensure CI passes.

## Reporting Issues

- Use GitHub Issues for bugs, features, or questions.
- Provide steps to reproduce, expected vs. actual behavior, and environment details (Rust version, OS).

## License

By contributing, you agree to license your contributions under the MIT License.