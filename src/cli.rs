use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "typst-count")]
#[command(version, about = "Count words and characters in Typst documents")]
#[command(long_about = "Count words and characters in Typst documents.\n\n\
                  Counts are based on the compiled document, meaning only rendered \
                  text is counted. Code, markup, headers, and footers are excluded.")]
pub struct Cli {
    /// Path(s) to Typst document(s)
    #[arg(required = true, value_name = "FILE")]
    pub input: Vec<PathBuf>,

    /// Output format
    #[arg(short = 'f', long, value_enum, default_value_t = OutputFormat::Human)]
    pub format: OutputFormat,

    /// What to count
    #[arg(short = 'm', long = "mode", value_enum, default_value_t = CountMode::Both)]
    pub mode: CountMode,

    /// Write output to file instead of stdout
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Display mode for multiple files
    #[arg(short = 'd', long = "display", value_enum, default_value_t = DisplayMode::Auto)]
    pub display: DisplayMode,

    /// Exclude content from imported/included files
    #[arg(short = 'e', long = "exclude-imports")]
    pub exclude_imports: bool,

    /// Exit with error if word count exceeds this limit
    #[arg(long, value_name = "N")]
    pub max_words: Option<usize>,

    /// Exit with error if word count is below this limit
    #[arg(long, value_name = "N")]
    pub min_words: Option<usize>,

    /// Exit with error if character count exceeds this limit
    #[arg(long, value_name = "N")]
    pub max_characters: Option<usize>,

    /// Exit with error if character count is below this limit
    #[arg(long, value_name = "N")]
    pub min_characters: Option<usize>,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable output (default)
    Human,
    /// JSON output
    Json,
    /// CSV output
    Csv,
}

#[derive(Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum CountMode {
    /// Count both words and characters
    Both,
    /// Count only words
    Words,
    /// Count only characters
    Characters,
}

#[derive(Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum DisplayMode {
    /// Automatic: detailed for multiple files, simple for single file
    Auto,
    /// Show only totals (no per-file breakdown)
    Total,
    /// Suppress all labels, output only numbers
    Quiet,
    /// Always show detailed breakdown
    Detailed,
}
