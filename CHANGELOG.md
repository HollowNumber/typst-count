# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-XX

### Added
- Initial release of typst-count
- Word and character counting for Typst documents
- Support for counting from compiled/rendered documents only
- Multiple output formats: human-readable, JSON, CSV
- Support for multiple input files with totals
- `--exclude-imports` flag to count only the main file
- `--format` option for output format selection (human/json/csv)
- `--output` option to write results to a file
- `--quiet` mode for CI/CD integration
- `--min-words`, `--max-words`, `--min-chars`, `--max-chars` for limit checking
- Proper exit codes for CI/CD pipelines
- Modular architecture with `cli`, `world`, `counter`, and `output` modules
- Comprehensive CLI built with clap
- Integration with Typst compiler API

### Features
- Counts only rendered text (excludes markup, code, comments)
- Handles imported and included files by default
- Word counting using whitespace splitting (compatible with `wc -w`)
- Character counting including spaces and punctuation
- Support for complex Typst documents with imports

### Documentation
- README with installation instructions, usage examples, and feature overview
- MIT license
- Code examples and CLI usage documentation

[Unreleased]: https://github.com/HollowNumber/typst-count/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/HollowNumber/typst-count/releases/tag/v0.1.0
