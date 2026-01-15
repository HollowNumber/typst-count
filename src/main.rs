//! Command-line interface for typst-count.
//!
//! This binary provides a CLI tool for counting words and characters in Typst documents.
//! It handles argument parsing, file processing, output formatting, and limit checking.

use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use typst_count::{check_limits, cli, output, process_files};

/// Writes formatted output to a file or stdout.
///
/// If an output path is provided, writes to that file. Otherwise, writes to stdout.
///
/// # Arguments
///
/// * `content` - The content to write
/// * `output_path` - Optional path to output file
///
/// # Errors
///
/// Returns an error if:
/// - The output file cannot be created
/// - Writing to the file or stdout fails
fn write_output(content: &str, output_path: Option<&Path>) -> Result<()> {
    if let Some(path) = output_path {
        let mut file = File::create(path)
            .with_context(|| format!("Failed to create output file: {}", path.display()))?;
        file.write_all(content.as_bytes())
            .with_context(|| format!("Failed to write to output file: {}", path.display()))?;
    } else {
        print!("{content}");
        io::stdout().flush()?;
    }
    Ok(())
}

/// Main entry point for the typst-count CLI tool.
///
/// This function orchestrates the entire counting process:
/// 1. Parses command-line arguments
/// 2. Processes all input files and compiles them
/// 3. Formats the output according to the specified format
/// 4. Writes output to file or stdout
/// 5. Checks count limits and exits with appropriate status code
///
/// # Exit Codes
///
/// - `0`: Success - all files processed and limits satisfied
/// - `1`: Limit violation - counts exceed or fall below specified limits
/// - `2`: Error - compilation failure or other error
fn main() -> Result<()> {
    let args = cli::Cli::parse();

    let results = process_files(&args)?;

    let formatter = output::OutputFormatter::new(args.format, args.mode);
    let output_text = formatter.format_output(&results, args.display);

    write_output(&output_text, args.output.as_deref())?;

    let total = output::calculate_total(&results);
    if let Err(errors) = check_limits(&args, &total) {
        for error in errors {
            eprintln!("Error: {error}");
        }
        process::exit(1);
    }

    Ok(())
}
