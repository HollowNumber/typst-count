# Contributing to typst-count

Thank you for your interest in contributing to typst-count!

We welcome contributions of all kinds: bug reports, feature requests, documentation improvements, and code contributions.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)

## Code of Conduct

This project follows a simple code of conduct: be respectful, be constructive, and help create a welcoming environment for everyone.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A GitHub account

### Setting Up Your Development Environment

1. **Fork the repository** on GitHub

2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/typst-count.git
   cd typst-count
   ```

3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/HollowNumber/typst-count.git
   ```

4. **Build the project**:
   ```bash
   cargo build
   ```

5. **Run the tests**:
   ```bash
   cargo test
   ```

6. **Try the CLI**:
   ```bash
   cargo run -- --help
   ```

## How to Contribute

### Reporting Bugs

Before creating a bug report:
- Check the existing issues to avoid duplicates
- Collect information about your environment (OS, Rust version, etc.)

When filing a bug report, include:
- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior vs. actual behavior
- Any error messages or logs
- Your environment details (OS, Rust version, typst-count version)
- Sample Typst files if relevant (or minimal reproducible examples)

### Suggesting Features

Feature requests are welcome! Please:
- Check existing issues/PRs to avoid duplicates
- Explain the use case and why it would be valuable
- Provide examples of how the feature would be used
- Consider whether it fits the scope of the project

### Improving Documentation

Documentation improvements are always appreciated:
- Fix typos or unclear wording
- Add examples or clarify existing ones
- Improve README, CHANGELOG, or code comments
- Add or improve rustdoc comments

## Development Workflow

### Creating a Branch

```bash
# Update your main branch
git checkout main
git pull upstream main

# Create a feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### Making Changes

1. Write your code following the [Coding Standards](#coding-standards)
2. Add or update tests as needed
3. Update documentation (README, rustdoc, etc.)
4. Run the test suite
5. Run clippy and fix any warnings
6. Format your code

### Committing Changes

Write clear, descriptive commit messages:

```
Add JSON output format support

- Implement JSON serialization for CountResult
- Add --format json CLI option
- Add tests for JSON output
- Update documentation with JSON examples
```

Follow these guidelines:
- Use the imperative mood ("Add feature" not "Added feature")
- First line should be 50 characters or less
- Add a blank line after the first line
- Wrap subsequent lines at 72 characters
- Reference relevant issues (e.g., "Fixes #123")

## Coding Standards

### Rust Style Guide

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` to format code (enforced in CI)
- Use `cargo clippy` and address all warnings (enforced in CI)

### Code Organization

```
src/
├── main.rs          # Entry point (minimal)
├── lib.rs           # Library exports
├── cli.rs           # CLI argument parsing
├── world.rs         # Typst world implementation
├── counter.rs       # Counting logic
└── output/          # Output formatting
    ├── mod.rs
    ├── human.rs
    ├── json.rs
    └── csv.rs
```

### Best Practices

- **Keep functions small and focused**: Each function should do one thing well
- **Use descriptive names**: Variables, functions, and types should be self-documenting
- **Add comments for complex logic**: Explain *why*, not *what*
- **Use `Result` and `?` for error handling**: Avoid `unwrap()` in library code
- **Write rustdoc comments for public APIs**: Include examples where helpful
- **Prefer composition over inheritance**: Use traits and structs effectively

### Error Handling

```rust
// Good: Propagate errors with context
fn process_file(path: &Path) -> Result<CountResult> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    // ...
}

// Avoid: Unwrapping in library code
fn process_file(path: &Path) -> CountResult {
    let content = fs::read_to_string(path).unwrap(); // ❌
    // ...
}
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_word_count

# Run tests with coverage (if installed)
cargo tarpaulin
```

### Writing Tests

- Add unit tests in the same file as the code being tested
- Add integration tests in `tests/` directory
- Test edge cases and error conditions
- Use descriptive test names

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_simple_document() {
        let input = "Hello world";
        let result = count_text(input);
        assert_eq!(result.words, 2);
        assert_eq!(result.characters, 11);
    }

    #[test]
    fn test_count_with_punctuation() {
        let input = "Hello, world!";
        let result = count_text(input);
        assert_eq!(result.words, 2);
        assert_eq!(result.characters, 13);
    }
}
```

### Test Coverage

We aim for high test coverage, especially for:
- Core counting logic
- CLI argument parsing
- Output formatting
- Error handling
- Edge cases (empty files, invalid input, etc.)

## Pull Request Process

### Before Submitting

Ensure your PR passes all checks:

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test

# Build release binary
cargo build --release
```

### Submitting a Pull Request

1. **Push your branch** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create a pull request** on GitHub

3. **Fill out the PR template** with:
   - Description of changes
   - Motivation and context
   - How to test the changes
   - Related issues (use "Fixes #123" to auto-close)
   - Screenshots (if UI changes)

4. **Wait for review**:
   - Address any feedback
   - Update your branch if needed
   - Be responsive to comments

### PR Guidelines

- Keep PRs focused on a single feature or fix
- Avoid mixing unrelated changes
- Rebase on main if your branch is outdated
- Update documentation as needed
- Add tests for new features
- Keep commits clean (consider squashing if many small commits)

### Review Process

- Maintainers will review your PR
- CI checks must pass (tests, clippy, formatting)
- At least one approval is required
- Feedback should be addressed or discussed
- Once approved, a maintainer will merge your PR

## Release Process

For maintainers only:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with release date and notes
3. Commit: `git commit -am "Release v0.x.0"`
4. Tag: `git tag -a v0.x.0 -m "Release v0.x.0"`
5. Push: `git push && git push --tags`
6. Publish: `cargo publish`
7. Create GitHub release with changelog notes

## Questions?

If you have questions:
- Check existing issues and documentation
- Open a discussion issue
- Reach out to maintainers

Thank you for contributing!
