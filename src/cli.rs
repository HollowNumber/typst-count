//! Command-line interface definitions for typst-count.
//!
//! This module defines the CLI structure using `clap`, including all command-line
//! arguments, options, and their associated enums for output formats and counting modes.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Command-line arguments for the typst-count tool.
///
/// This structure defines all available options for counting words and characters
/// in Typst documents, including input files, output formats, counting modes,
/// and various filtering and limiting options.
#[derive(Parser)]
#[command(name = "typst-count")]
#[command(version, about = "Count words and characters in Typst documents")]
#[command(long_about = "Count words and characters in Typst documents.\n\n\
                  Counts are based on the compiled document, meaning only rendered \
                  text is counted. Code, markup, headers, and footers are excluded.")]
pub struct Cli {
    /// Path(s) to Typst document(s) to count.
    ///
    /// Multiple files can be specified to get counts for each file plus totals.
    /// Path(s) to Typst document(s)
    #[arg(required = true, value_name = "FILE")]
    pub input: Vec<PathBuf>,

    /// Output format for results.
    ///
    /// Available formats:
    /// - `human`: Human-readable table format (default)
    /// - `json`: JSON format for machine processing
    /// - `csv`: CSV format for spreadsheet import
    #[arg(short = 'f', long, value_enum, default_value_t = OutputFormat::Human)]
    pub format: OutputFormat,

    /// What to count in the documents.
    ///
    /// - `both`: Count both words and characters (default)
    /// - `words`: Count only words
    /// - `characters`: Count only characters
    #[arg(short = 'm', long = "mode", value_enum, default_value_t = CountMode::Both)]
    pub mode: CountMode,

    /// Write output to a file instead of stdout.
    ///
    /// If not specified, output is written to stdout. The file format is
    /// determined by the `--format` option.
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Display mode when processing multiple files.
    ///
    /// - `auto`: Detailed for multiple files, simple for single file (default)
    /// - `total`: Show only totals, no per-file breakdown
    /// - `quiet`: Suppress labels, output only numbers
    /// - `detailed`: Always show per-file breakdown
    #[arg(short = 'd', long = "display", value_enum, default_value_t = DisplayMode::Auto)]
    pub display: DisplayMode,

    /// Exclude content from imported/included files.
    ///
    /// By default, text from all imported and included files is counted.
    /// Use this flag to count only the main file(s) specified on the command line.
    #[arg(short = 'e', long = "exclude-imports")]
    pub exclude_imports: bool,

    /// Exit with error if word count exceeds this limit.
    ///
    /// Useful for CI/CD pipelines to enforce maximum document length.
    /// Exit code will be 1 if the limit is exceeded.
    #[arg(long, value_name = "N")]
    pub max_words: Option<usize>,

    /// Exit with error if word count is below this limit.
    ///
    /// Useful for CI/CD pipelines to enforce minimum document length.
    /// Exit code will be 1 if the count is below the limit.
    #[arg(long, value_name = "N")]
    pub min_words: Option<usize>,

    /// Exit with error if character count exceeds this limit.
    ///
    /// Useful for CI/CD pipelines to enforce maximum document size.
    /// Exit code will be 1 if the limit is exceeded.
    #[arg(long, value_name = "N")]
    pub max_characters: Option<usize>,

    /// Exit with error if character count is below this limit.
    ///
    /// Useful for CI/CD pipelines to enforce minimum document size.
    /// Exit code will be 1 if the count is below the limit.
    #[arg(long, value_name = "N")]
    pub min_characters: Option<usize>,
}

/// Output format for displaying count results.
///
/// Determines how the word and character counts are formatted and presented.
#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum OutputFormat {
    /// Human-readable table format (default).
    ///
    /// Displays results in an easy-to-read format with labels and formatting.
    Human,
    /// JSON output for machine processing.
    ///
    /// Outputs results as JSON with file paths, counts, and totals.
    /// Suitable for parsing by scripts and other tools.
    Json,
    /// CSV output for spreadsheet import.
    ///
    /// Outputs results in comma-separated values format, suitable for
    /// importing into spreadsheet applications or data analysis tools.
    Csv,
}

/// What to count in the document.
///
/// Determines whether to count words, characters, or both.
#[derive(Clone, Copy, ValueEnum, PartialEq, Eq, Debug)]
pub enum CountMode {
    /// Count both words and characters (default).
    Both,
    /// Count only words.
    ///
    /// Words are counted by splitting on whitespace.
    Words,
    /// Count only characters.
    ///
    /// Counts all Unicode scalar values including spaces and punctuation.
    Characters,
}

/// Display mode for formatting output when processing multiple files.
///
/// Controls how detailed the output should be and how results are presented.
#[derive(Clone, Copy, ValueEnum, PartialEq, Eq, Debug)]
pub enum DisplayMode {
    /// Automatic mode (default).
    ///
    /// Shows detailed breakdown for multiple files, simple output for single file.
    Auto,
    /// Show only totals without per-file breakdown.
    ///
    /// Useful when you only care about aggregate counts across all files.
    Total,
    /// Suppress all labels and output only numbers.
    ///
    /// Outputs raw numbers only, suitable for piping to other commands.
    Quiet,
    /// Always show detailed per-file breakdown.
    ///
    /// Shows counts for each file individually even for single files.
    Detailed,
}
