//! A library for counting words and characters in Typst documents.
//!
//! This crate provides functionality to compile Typst documents and count the
//! words and characters in the rendered output. It works by:
//!
//! 1. Compiling Typst documents using the Typst compiler
//! 2. Traversing the compiled document's element tree
//! 3. Extracting plain text from rendered elements
//! 4. Counting words (by whitespace) and characters
//!
//! # Features
//!
//! - Count words and characters from compiled Typst documents
//! - Handle imported and included files
//! - Multiple output formats (human-readable, JSON, CSV)
//! - CI/CD integration with limit checking
//!
//! # Examples
//!
//! ```no_run
//! use typst_count::compile_document;
//! use std::path::Path;
//!
//! let path = Path::new("document.typ");
//! let count = compile_document(path, false).unwrap();
//! println!("Words: {}, Characters: {}", count.words, count.characters);
//! ```
#[allow(clippy::multiple_crate_versions)]
pub mod cli;
pub mod counter;
pub mod output;
pub mod world;

use anyhow::{Context, Result};
use cli::Cli;
use counter::Count;
use std::path::Path;
use typst::{World, layout::PagedDocument};

/// Compiles a Typst document and counts its words and characters.
///
/// This function loads a Typst document, compiles it using the Typst compiler,
/// and extracts word and character counts from the rendered output.
///
/// # Arguments
///
/// * `path` - Path to the Typst document file
/// * `exclude_imports` - If `true`, only counts content from the main file,
///   excluding imported/included files
///
/// # Returns
///
/// A `Count` struct containing word and character counts, or an error if
/// compilation fails.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The document fails to compile
/// - There are syntax errors in the Typst code
///
/// # Examples
///
/// ```no_run
/// use typst_count::compile_document;
/// use std::path::Path;
///
/// // Count all content including imports
/// let count = compile_document(Path::new("document.typ"), false)?;
///
/// // Count only the main file
/// let count = compile_document(Path::new("document.typ"), true)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn compile_document(path: &Path, exclude_imports: bool) -> Result<Count> {
    let world = world::SimpleWorld::new(path)
        .with_context(|| format!("Failed to load {}", path.display()))?;
    let main_file_id = world.main();

    let result = typst::compile(&world);
    let document: PagedDocument = result
        .output
        .map_err(|errors| anyhow::anyhow!("Failed to compile {}: {:?}", path.display(), errors))?;

    Ok(counter::count_document(
        &document.introspector,
        exclude_imports,
        main_file_id,
    ))
}

/// Processes multiple Typst files and returns their counts.
///
/// Compiles each input file specified in the CLI arguments and collects
/// the word and character counts for each file.
///
/// # Arguments
///
/// * `args` - Command-line arguments containing input files and options
///
/// # Returns
///
/// A vector of tuples, each containing a file path (as a string) and its
/// corresponding `Count`, or an error if any file fails to compile.
///
/// # Errors
///
/// Returns an error if any of the input files:
/// - Cannot be read
/// - Fails to compile
/// - Contains syntax errors
///
/// # Examples
///
/// ```no_run
/// use typst_count::{process_files, cli::Cli};
/// use clap::Parser;
///
/// let args = Cli::parse();
/// let results = process_files(&args)?;
///
/// for (path, count) in results {
///     println!("{}: {} words", path, count.words);
/// }
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn process_files(args: &Cli) -> Result<Vec<(String, Count)>> {
    args.input
        .iter()
        .map(|path| {
            compile_document(path, args.exclude_imports)
                .map(|count| (path.display().to_string(), count))
        })
        .collect()
}

/// Checks if word and character counts are within specified limits.
///
/// Validates that the total counts meet any minimum or maximum limits
/// specified in the CLI arguments. This is useful for CI/CD pipelines
/// to enforce document length requirements.
///
/// # Arguments
///
/// * `args` - Command-line arguments containing limit specifications
/// * `total` - The total count to check against limits
///
/// # Returns
///
/// - `Ok(())` if all limits are satisfied
/// - `Err(Vec<String>)` containing error messages for each violated limit
///
/// # Limit Checks
///
/// The following limits are checked if specified:
/// - `max_words` - Maximum allowed word count
/// - `min_words` - Minimum required word count
/// - `max_characters` - Maximum allowed character count
/// - `min_characters` - Minimum required character count
///
/// # Examples
///
/// ```no_run
/// use typst_count::{check_limits, cli::Cli, counter::Count};
/// use clap::Parser;
///
/// let args = Cli::parse();
/// let total = Count { words: 500, characters: 2500 };
///
/// match check_limits(&args, &total) {
///     Ok(()) => println!("All limits satisfied"),
///     Err(errors) => {
///         for error in errors {
///             eprintln!("Limit violation: {}", error);
///         }
///     }
/// }
/// ```
pub fn check_limits(args: &Cli, total: &Count) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if let Some(max) = args.max_words
        && total.words > max
    {
        errors.push(format!(
            "Word count exceeds maximum ({} > {})",
            total.words, max
        ));
    }

    if let Some(min) = args.min_words
        && total.words < min
    {
        errors.push(format!(
            "Word count below minimum ({} < {})",
            total.words, min
        ));
    }

    if let Some(max) = args.max_characters
        && total.characters > max
    {
        errors.push(format!(
            "Character count exceeds maximum ({} > {})",
            total.characters, max
        ));
    }

    if let Some(min) = args.min_characters
        && total.characters < min
    {
        errors.push(format!(
            "Character count below minimum ({} < {})",
            total.characters, min
        ));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{Cli, CountMode, DisplayMode, OutputFormat};

    fn make_test_cli() -> Cli {
        Cli {
            input: vec![],
            format: OutputFormat::Human,
            mode: CountMode::Both,
            output: None,
            display: DisplayMode::Auto,
            exclude_imports: false,
            max_words: None,
            min_words: None,
            max_characters: None,
            min_characters: None,
        }
    }

    #[test]
    fn test_check_limits_no_limits() {
        let args = make_test_cli();
        let count = Count {
            words: 100,
            characters: 500,
        };

        assert!(check_limits(&args, &count).is_ok());
    }

    #[test]
    fn test_check_limits_max_words_ok() {
        let mut args = make_test_cli();
        args.max_words = Some(200);
        let count = Count {
            words: 100,
            characters: 500,
        };

        assert!(check_limits(&args, &count).is_ok());
    }

    #[test]
    fn test_check_limits_max_words_exceeded() {
        let mut args = make_test_cli();
        args.max_words = Some(50);
        let count = Count {
            words: 100,
            characters: 500,
        };

        let result = check_limits(&args, &count);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("exceeds maximum"));
        assert!(errors[0].contains("100 > 50"));
    }

    #[test]
    fn test_check_limits_min_words_ok() {
        let mut args = make_test_cli();
        args.min_words = Some(50);
        let count = Count {
            words: 100,
            characters: 500,
        };

        assert!(check_limits(&args, &count).is_ok());
    }

    #[test]
    fn test_check_limits_min_words_below() {
        let mut args = make_test_cli();
        args.min_words = Some(200);
        let count = Count {
            words: 100,
            characters: 500,
        };

        let result = check_limits(&args, &count);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("below minimum"));
        assert!(errors[0].contains("100 < 200"));
    }

    #[test]
    fn test_check_limits_max_characters_ok() {
        let mut args = make_test_cli();
        args.max_characters = Some(1000);
        let count = Count {
            words: 100,
            characters: 500,
        };

        assert!(check_limits(&args, &count).is_ok());
    }

    #[test]
    fn test_check_limits_max_characters_exceeded() {
        let mut args = make_test_cli();
        args.max_characters = Some(300);
        let count = Count {
            words: 100,
            characters: 500,
        };

        let result = check_limits(&args, &count);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("exceeds maximum"));
        assert!(errors[0].contains("500 > 300"));
    }

    #[test]
    fn test_check_limits_min_characters_ok() {
        let mut args = make_test_cli();
        args.min_characters = Some(100);
        let count = Count {
            words: 100,
            characters: 500,
        };

        assert!(check_limits(&args, &count).is_ok());
    }

    #[test]
    fn test_check_limits_min_characters_below() {
        let mut args = make_test_cli();
        args.min_characters = Some(1000);
        let count = Count {
            words: 100,
            characters: 500,
        };

        let result = check_limits(&args, &count);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("below minimum"));
        assert!(errors[0].contains("500 < 1000"));
    }

    #[test]
    fn test_check_limits_multiple_violations() {
        let mut args = make_test_cli();
        args.max_words = Some(50);
        args.min_words = Some(200);
        args.max_characters = Some(300);
        args.min_characters = Some(1000);
        let count = Count {
            words: 100,
            characters: 500,
        };

        let result = check_limits(&args, &count);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        // Should have 4 violations: max_words exceeded, min_words not met,
        // max_characters exceeded, min_characters not met
        assert_eq!(errors.len(), 4);
    }

    #[test]
    fn test_check_limits_boundary_values() {
        let mut args = make_test_cli();
        args.max_words = Some(100);
        args.min_words = Some(100);
        let count = Count {
            words: 100,
            characters: 500,
        };

        // Exactly at the boundary should be OK
        assert!(check_limits(&args, &count).is_ok());
    }

    #[test]
    fn test_check_limits_mixed_ok_and_violations() {
        let mut args = make_test_cli();
        args.max_words = Some(200); // OK
        args.min_words = Some(50); // OK
        args.max_characters = Some(300); // Violation
        args.min_characters = Some(100); // OK
        let count = Count {
            words: 100,
            characters: 500,
        };

        let result = check_limits(&args, &count);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Character count exceeds maximum"));
    }
}
