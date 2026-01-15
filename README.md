# typst-count

[![Crates.io](https://img.shields.io/crates/v/typst-count.svg)](https://crates.io/crates/typst-count)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A command-line tool to count words and characters in [Typst](https://typst.app/) documents.

## Features

- Count words and characters from compiled Typst documents
- Counts only rendered text (excludes markup, code, and comments)
- Handles imported and included files automatically
- Multiple output formats: human-readable, JSON, CSV
- Support for multiple input files with totals
- CI-friendly with limit checks and proper exit codes
- Flexible display modes for different use cases

## Installation

### From crates.io

```bash
cargo install typst-count
```

### From source

```bash
git clone https://github.com/HollowNumber/typst-count.git
cd typst-count
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Count both words and characters
typst-count document.typ

# Count only words
typst-count document.typ --words

# Count only characters
typst-count document.typ --characters
```

### Output Formats

```bash
# Human-readable output (default)
typst-count document.typ

# JSON output for scripting
typst-count document.typ --format json

# CSV output for data analysis
typst-count document.typ --format csv
```

### Multiple Files

```bash
# Count multiple files with totals
typst-count doc1.typ doc2.typ doc3.typ

# Output results to a file
typst-count *.typ --output results.json --format json
```

### Advanced Options

```bash
# Exclude imported/included files (count only main file)
typst-count document.typ --exclude-imports

# Quiet mode (exit code only, useful in CI)
typst-count document.typ --quiet

# Set limits with exit codes (useful for CI/CD)
typst-count document.typ --min-words 500 --max-words 1000
typst-count document.typ --min-chars 2000 --max-chars 5000
```

## How It Works

`typst-count` compiles your Typst document and extracts the rendered text content. This means:

- **Counted**: All text that appears in the final rendered document
- **Counted**: Content from imported/included files (by default)
- **Counted**: Markup syntax (like `*bold*`, `_italic_`)
- **Not counted**: Code blocks and inline code
- **Not counted**: Comments
- **Not counted**: Function definitions and calls

### Counting Method

- **Words**: Split by whitespace (same as `wc -w`). Note: For languages without spaces (e.g., Chinese, Japanese), each character may be counted as a separate "word"
- **Characters**: Total character count including spaces and punctuation

## Examples

### Example Output (Human-readable)

```
File: document.typ
Characters: 1,842
Words: 287
```

### Example Output (JSON)

```json
{
  "files": [
    {
      "path": "document.typ",
      "characters": 1842,
      "words": 287
    }
  ],
  "total": {
    "characters": 1842,
    "words": 287
  }
}
```

## CI/CD Integration

Check word count limits in your CI pipeline:

```yaml
# .github/workflows/check-docs.yml
- name: Check documentation word count
  run: |
    cargo install typst-count
    typst-count docs.typ --min-words 1000 --max-words 5000
```

Exit codes:
- `0`: Success (within limits if specified)
- `1`: Below minimum or above maximum limit
- `2`: Compilation or other error

## Comparison with Other Tools

- **vs `wc`**: Counts only rendered text, not markup/code
- **vs Typst word count function**: Can be used in CI/CD and with multiple files
- **vs Tinymist LSP**: Standalone tool, doesn't require editor integration

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development

```bash
# Clone the repository
git clone https://github.com/HollowNumber/typst-count.git
cd typst-count

# Run tests
cargo test

# Run clippy
cargo clippy --all-targets

# Format code
cargo fmt
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Typst](https://typst.app/) - A new markup-based typesetting system
- Inspired by classic tools like `wc` and modern Typst tooling

## Related Projects

- [Typst](https://github.com/typst/typst) - The Typst compiler
- [Tinymist](https://github.com/Myriad-Dreamin/tinymist) - Typst LSP server with editor integration
