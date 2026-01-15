//! Output formatting for word and character count results.
//!
//! This module provides formatters for displaying count results in various formats
//! including human-readable tables, JSON, and CSV. It handles different display modes
//! and counting modes to present the data appropriately.

mod csv;
mod human;
mod json;

use crate::cli::{CountMode, DisplayMode, OutputFormat};
use crate::counter::Count;

/// Formatter for outputting count results in various formats.
///
/// Combines an output format (human/JSON/CSV) with a counting mode (words/characters/both)
/// to produce formatted output strings from count results.
///
/// # Examples
///
/// ```no_run
/// use typst_count::output::OutputFormatter;
/// use typst_count::cli::{OutputFormat, CountMode, DisplayMode};
/// use typst_count::counter::Count;
///
/// let formatter = OutputFormatter::new(OutputFormat::Human, CountMode::Both);
/// let results = vec![("document.typ".to_string(), Count { words: 100, characters: 500 })];
/// let output = formatter.format_output(&results, DisplayMode::Auto);
/// println!("{}", output);
/// ```
pub struct OutputFormatter {
    /// The output format to use (human/JSON/CSV)
    format: OutputFormat,
    /// What to count and display (words/characters/both)
    mode: CountMode,
}

impl OutputFormatter {
    /// Creates a new output formatter with the specified format and counting mode.
    ///
    /// # Arguments
    ///
    /// * `format` - The output format (human-readable, JSON, or CSV)
    /// * `mode` - The counting mode (words, characters, or both)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use typst_count::output::OutputFormatter;
    /// use typst_count::cli::{OutputFormat, CountMode};
    ///
    /// let formatter = OutputFormatter::new(OutputFormat::Human, CountMode::Both);
    /// ```
    #[must_use]
    pub const fn new(format: OutputFormat, mode: CountMode) -> Self {
        Self { format, mode }
    }

    /// Formats count results according to the configured format and mode.
    ///
    /// Takes a slice of file paths and their counts, and produces a formatted string
    /// according to the output format (human/JSON/CSV) and display mode.
    ///
    /// # Arguments
    ///
    /// * `results` - Slice of tuples containing file paths and their counts
    /// * `display` - Display mode controlling output verbosity and style
    ///
    /// # Returns
    ///
    /// A formatted string ready for output to stdout or a file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use typst_count::output::OutputFormatter;
    /// use typst_count::cli::{OutputFormat, CountMode, DisplayMode};
    /// use typst_count::counter::Count;
    ///
    /// let formatter = OutputFormatter::new(OutputFormat::Json, CountMode::Words);
    /// let results = vec![
    ///     ("doc1.typ".to_string(), Count { words: 100, characters: 500 }),
    ///     ("doc2.typ".to_string(), Count { words: 200, characters: 1000 }),
    /// ];
    /// let output = formatter.format_output(&results, DisplayMode::Detailed);
    /// ```
    #[must_use]
    pub fn format_output(&self, results: &[(String, Count)], display: DisplayMode) -> String {
        match self.format {
            OutputFormat::Human => human::format(results, display, self.mode),
            OutputFormat::Json => json::format(results, display, self.mode),
            OutputFormat::Csv => csv::format(results, display, self.mode),
        }
    }
}

/// Calculates the total word and character count across multiple files.
///
/// Sums up all word counts and character counts from the provided results
/// to produce aggregate totals.
///
/// # Arguments
///
/// * `results` - Slice of tuples containing file paths and their counts
///
/// # Returns
///
/// A `Count` struct containing the summed totals of all files.
///
/// # Examples
///
/// ```no_run
/// use typst_count::output::calculate_total;
/// use typst_count::counter::Count;
///
/// let results = vec![
///     ("doc1.typ".to_string(), Count { words: 100, characters: 500 }),
///     ("doc2.typ".to_string(), Count { words: 200, characters: 1000 }),
/// ];
/// let total = calculate_total(&results);
/// assert_eq!(total.words, 300);
/// assert_eq!(total.characters, 1500);
/// ```
#[must_use]
pub fn calculate_total(results: &[(String, Count)]) -> Count {
    Count {
        words: results.iter().map(|(_, c)| c.words).sum(),
        characters: results.iter().map(|(_, c)| c.characters).sum(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_single_file() {
        let results = vec![(
            "file1.typ".to_string(),
            Count {
                words: 100,
                characters: 500,
            },
        )];

        let total = calculate_total(&results);
        assert_eq!(total.words, 100);
        assert_eq!(total.characters, 500);
    }

    #[test]
    fn test_calculate_total_multiple_files() {
        let results = vec![
            (
                "file1.typ".to_string(),
                Count {
                    words: 100,
                    characters: 500,
                },
            ),
            (
                "file2.typ".to_string(),
                Count {
                    words: 200,
                    characters: 1000,
                },
            ),
            (
                "file3.typ".to_string(),
                Count {
                    words: 50,
                    characters: 250,
                },
            ),
        ];

        let total = calculate_total(&results);
        assert_eq!(total.words, 350);
        assert_eq!(total.characters, 1750);
    }

    #[test]
    fn test_calculate_total_empty() {
        let results: Vec<(String, Count)> = vec![];

        let total = calculate_total(&results);
        assert_eq!(total.words, 0);
        assert_eq!(total.characters, 0);
    }

    #[test]
    fn test_calculate_total_zero_counts() {
        let results = vec![
            (
                "file1.typ".to_string(),
                Count {
                    words: 0,
                    characters: 0,
                },
            ),
            (
                "file2.typ".to_string(),
                Count {
                    words: 0,
                    characters: 0,
                },
            ),
        ];

        let total = calculate_total(&results);
        assert_eq!(total.words, 0);
        assert_eq!(total.characters, 0);
    }

    #[test]
    fn test_output_formatter_creation() {
        let formatter = OutputFormatter::new(OutputFormat::Human, CountMode::Both);
        // Just verify it can be created without panicking
        assert_eq!(formatter.mode, CountMode::Both);
    }

    #[test]
    fn test_output_formatter_format_output_single_file() {
        let formatter = OutputFormatter::new(OutputFormat::Human, CountMode::Both);
        let results = vec![(
            "test.typ".to_string(),
            Count {
                words: 42,
                characters: 200,
            },
        )];

        let output = formatter.format_output(&results, DisplayMode::Auto);
        assert!(output.contains("42"));
        assert!(output.contains("200"));
    }
}
