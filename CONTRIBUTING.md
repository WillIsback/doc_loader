# Contributing to Doc Loader

Thank you for your interest in contributing to Doc Loader! This document provides guidelines for contributing to the project.

## Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/WillIsback/doc_loader.git
   cd doc_loader
   ```

2. **Install Rust (if not already installed):**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. **Build the project:**
   ```bash
   cargo build
   ```

4. **Run tests:**
   ```bash
   cargo test
   ```

## Python Development

For Python bindings development:

1. **Create a virtual environment:**
   ```bash
   python3 -m venv venv
   source venv/bin/activate
   ```

2. **Install maturin:**
   ```bash
   pip install maturin
   ```

3. **Build and install Python package:**
   ```bash
   maturin develop --features python
   ```

## Code Style

- Follow Rust formatting conventions: `cargo fmt`
- Run clippy for linting: `cargo clippy`
- Write tests for new functionality
- Update documentation for API changes

## Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Add tests if applicable
5. Ensure all tests pass: `cargo test`
6. Run formatting and linting: `cargo fmt && cargo clippy`
7. Commit your changes with a clear message
8. Push to your fork: `git push origin feature/your-feature-name`
9. Submit a pull request

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- Operating system
- File format and size (if applicable)
- Command used
- Complete error messages
- Expected vs actual behavior

## Feature Requests

We welcome feature requests! Please:

- Check existing issues to avoid duplicates
- Clearly describe the feature and its benefits
- Provide examples of how it would be used
- Consider implementation complexity

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
