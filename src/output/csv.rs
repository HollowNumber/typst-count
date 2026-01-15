//! CSV output formatting.
//!
//! This module provides functions to format count results as CSV (Comma-Separated Values),
//! suitable for importing into spreadsheet applications and data analysis tools.

use crate::cli::{CountMode, DisplayMode};
use crate::counter::Count;
use crate::output::calculate_total;
use std::fmt::Write;

/// Formats count results as CSV.
///
/// Produces CSV output with a header row and data rows. The columns included
/// depend on the counting mode (words, characters, or both).
///
/// # Arguments
///
/// * `results` - Slice of file paths and their counts
/// * `display` - Display mode controlling whether to show individual files or totals
/// * `mode` - What columns to include (words/characters/both)
///
/// # Returns
///
/// A CSV-formatted string with header row and data rows.
pub fn format(results: &[(String, Count)], display: DisplayMode, mode: CountMode) -> String {
    let mut output = String::new();

    writeln!(output, "{}", format_header(mode)).unwrap();

    if display == DisplayMode::Total && results.len() > 1 {
        let total = calculate_total(results);
        write_row(&mut output, "total", &total, mode);
    } else {
        for (name, count) in results {
            write_row(&mut output, name, count, mode);
        }
    }

    output
}

/// Returns the CSV header row based on the counting mode.
///
/// # Arguments
///
/// * `mode` - What columns to include (words/characters/both)
///
/// # Returns
///
/// A static string containing the CSV header row.
const fn format_header(mode: CountMode) -> &'static str {
    match mode {
        CountMode::Both => "file,words,characters",
        CountMode::Words => "file,words",
        CountMode::Characters => "file,characters",
    }
}

/// Writes a single data row to the CSV output.
///
/// # Arguments
///
/// * `output` - Mutable string to append the row to
/// * `name` - File name for the first column
/// * `count` - Count values to include in the row
/// * `mode` - What columns to include (words/characters/both)
fn write_row(output: &mut String, name: &str, count: &Count, mode: CountMode) {
    let row = match mode {
        CountMode::Both => format!("{},{},{}", name, count.words, count.characters),
        CountMode::Words => format!("{},{}", name, count.words),
        CountMode::Characters => format!("{},{}", name, count.characters),
    };
    writeln!(output, "{row}").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_header_both() {
        let header = format_header(CountMode::Both);
        assert_eq!(header, "file,words,characters");
    }

    #[test]
    fn test_format_header_words_only() {
        let header = format_header(CountMode::Words);
        assert_eq!(header, "file,words");
    }

    #[test]
    fn test_format_header_characters_only() {
        let header = format_header(CountMode::Characters);
        assert_eq!(header, "file,characters");
    }

    #[test]
    fn test_write_row_both() {
        let mut output = String::new();
        let count = Count {
            words: 100,
            characters: 500,
        };
        write_row(&mut output, "test.typ", &count, CountMode::Both);
        assert_eq!(output, "test.typ,100,500\n");
    }

    #[test]
    fn test_write_row_words_only() {
        let mut output = String::new();
        let count = Count {
            words: 100,
            characters: 500,
        };
        write_row(&mut output, "test.typ", &count, CountMode::Words);
        assert_eq!(output, "test.typ,100\n");
    }

    #[test]
    fn test_write_row_characters_only() {
        let mut output = String::new();
        let count = Count {
            words: 100,
            characters: 500,
        };
        write_row(&mut output, "test.typ", &count, CountMode::Characters);
        assert_eq!(output, "test.typ,500\n");
    }

    #[test]
    fn test_format_single_file() {
        let results = vec![(
            "test.typ".to_string(),
            Count {
                words: 100,
                characters: 500,
            },
        )];
        let output = format(&results, DisplayMode::Auto, CountMode::Both);
        assert_eq!(output, "file,words,characters\ntest.typ,100,500\n");
    }

    #[test]
    fn test_format_multiple_files() {
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
        ];
        let output = format(&results, DisplayMode::Auto, CountMode::Both);
        assert!(output.starts_with("file,words,characters\n"));
        assert!(output.contains("file1.typ,100,500\n"));
        assert!(output.contains("file2.typ,200,1000\n"));
    }

    #[test]
    fn test_format_display_mode_total() {
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
        ];
        let output = format(&results, DisplayMode::Total, CountMode::Both);
        assert_eq!(output, "file,words,characters\ntotal,300,1500\n");
    }

    #[test]
    fn test_format_words_only() {
        let results = vec![(
            "test.typ".to_string(),
            Count {
                words: 42,
                characters: 200,
            },
        )];
        let output = format(&results, DisplayMode::Auto, CountMode::Words);
        assert_eq!(output, "file,words\ntest.typ,42\n");
        assert!(!output.contains("characters"));
    }

    #[test]
    fn test_format_characters_only() {
        let results = vec![(
            "test.typ".to_string(),
            Count {
                words: 42,
                characters: 200,
            },
        )];
        let output = format(&results, DisplayMode::Auto, CountMode::Characters);
        assert_eq!(output, "file,characters\ntest.typ,200\n");
        assert!(!output.contains("words"));
    }

    #[test]
    fn test_format_total_single_file() {
        let results = vec![(
            "test.typ".to_string(),
            Count {
                words: 100,
                characters: 500,
            },
        )];
        // Total mode with single file doesn't trigger total output (needs len > 1)
        let output = format(&results, DisplayMode::Total, CountMode::Both);
        assert_eq!(output, "file,words,characters\ntest.typ,100,500\n");
    }
}
