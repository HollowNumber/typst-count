use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use typst_count::{check_limits, cli, output, process_files};

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
