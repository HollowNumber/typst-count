//! Human-readable output formatting.
//!
//! This module provides functions to format count results in a human-readable
//! table format with proper alignment and separators.

use crate::cli::{CountMode, DisplayMode};
use crate::counter::Count;
use crate::output::calculate_total;
use std::fmt::Write;

/// Formats count results in human-readable format.
///
/// Produces nicely formatted output with proper alignment, either as a simple
/// count display for single files or as a table with breakdown for multiple files.
///
/// # Arguments
///
/// * `results` - Slice of file paths and their counts
/// * `display` - Display mode controlling verbosity
/// * `mode` - What to count and display (words/characters/both)
///
/// # Returns
///
/// A formatted string ready for display to the user.
pub fn format(results: &[(String, Count)], display: DisplayMode, mode: CountMode) -> String {
    let show_breakdown = match display {
        DisplayMode::Auto => results.len() > 1,
        DisplayMode::Detailed => true,
        DisplayMode::Total | DisplayMode::Quiet => false,
    };

    if show_breakdown {
        format_table(results, display == DisplayMode::Quiet, mode)
    } else {
        let total = calculate_total(results);
        format_single(&total, display == DisplayMode::Quiet, mode)
    }
}

/// Formats a single count result.
///
/// Used when displaying results for a single file or when showing only totals.
///
/// # Arguments
///
/// * `count` - The count to format
/// * `quiet` - If true, omit labels and output only numbers
/// * `mode` - What to display (words/characters/both)
fn format_single(count: &Count, quiet: bool, mode: CountMode) -> String {
    match (mode, quiet) {
        (CountMode::Both, false) => {
            format!(
                " Words:      {}\n Characters: {}",
                count.words, count.characters
            )
        }
        (CountMode::Both, true) => format!("{} {}", count.words, count.characters),
        (CountMode::Words, false) => format!(" Words:      {}", count.words),
        (CountMode::Words, true) => format!("{}", count.words),
        (CountMode::Characters, false) => format!(" Characters: {}", count.characters),
        (CountMode::Characters, true) => format!("{}", count.characters),
    }
}

/// Formats multiple count results as a table.
///
/// Creates a formatted table with columns for file names and counts,
/// including a separator line and totals row.
///
/// # Arguments
///
/// * `results` - Slice of file paths and their counts
/// * `quiet` - If true, omit headers and separators
/// * `mode` - What to display (words/characters/both)
fn format_table(results: &[(String, Count)], quiet: bool, mode: CountMode) -> String {
    let mut output = String::new();
    let max_name_len = results.iter().map(|(n, _)| n.len()).max().unwrap_or(0);
    let name_width = max_name_len.max(4);

    if !quiet {
        writeln!(output, "{}", format_header(name_width, mode)).unwrap();
        writeln!(output, "{}", format_separator(name_width, mode)).unwrap();
    }

    for (name, count) in results {
        writeln!(
            output,
            "{}",
            format_row(name, count, name_width, quiet, mode)
        )
        .unwrap();
    }

    if !quiet {
        writeln!(output, "{}", format_separator(name_width, mode)).unwrap();
        let total = calculate_total(results);
        write!(
            output,
            "{}",
            format_row("Total", &total, name_width, false, mode)
        )
        .unwrap();
    }

    output
}

/// Formats the table header row.
///
/// # Arguments
///
/// * `name_width` - Width to allocate for the file name column
/// * `mode` - What columns to include (words/characters/both)
fn format_header(name_width: usize, mode: CountMode) -> String {
    match mode {
        CountMode::Both => {
            format!(
                "{:<width$} {:>12} {:>12}",
                "File",
                "Words",
                "Characters",
                width = name_width
            )
        }
        CountMode::Words => {
            format!("{:<width$} {:>12}", "File", "Words", width = name_width)
        }
        CountMode::Characters => {
            format!(
                "{:<width$} {:>12}",
                "File",
                "Characters",
                width = name_width
            )
        }
    }
}

/// Formats a separator line for the table.
///
/// # Arguments
///
/// * `name_width` - Width of the file name column
/// * `mode` - What columns are included (affects total width)
fn format_separator(name_width: usize, mode: CountMode) -> String {
    let total_width = match mode {
        CountMode::Both => name_width + 26,
        _ => name_width + 13,
    };
    "─".repeat(total_width)
}

/// Formats a single row in the table.
///
/// # Arguments
///
/// * `name` - Name to display in the first column (file name or "Total")
/// * `count` - Count values to display
/// * `name_width` - Width to allocate for the name column
/// * `quiet` - If true, omit the name column and output only numbers
/// * `mode` - What columns to include (words/characters/both)
fn format_row(
    name: &str,
    count: &Count,
    name_width: usize,
    quiet: bool,
    mode: CountMode,
) -> String {
    if quiet {
        match mode {
            CountMode::Both => format!("{} {}", count.words, count.characters),
            CountMode::Words => format!("{}", count.words),
            CountMode::Characters => format!("{}", count.characters),
        }
    } else {
        match mode {
            CountMode::Both => {
                format!(
                    "{:<width$} {:>12} {:>12}",
                    name,
                    count.words,
                    count.characters,
                    width = name_width
                )
            }
            CountMode::Words => {
                format!("{:<width$} {:>12}", name, count.words, width = name_width)
            }
            CountMode::Characters => {
                format!(
                    "{:<width$} {:>12}",
                    name,
                    count.characters,
                    width = name_width
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_single_both() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let output = format_single(&count, false, CountMode::Both);
        assert!(output.contains("100"));
        assert!(output.contains("500"));
        assert!(output.contains("Words"));
        assert!(output.contains("Characters"));
    }

    #[test]
    fn test_format_single_words_only() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let output = format_single(&count, false, CountMode::Words);
        assert!(output.contains("100"));
        assert!(!output.contains("500"));
        assert!(output.contains("Words"));
    }

    #[test]
    fn test_format_single_characters_only() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let output = format_single(&count, false, CountMode::Characters);
        assert!(!output.contains("100"));
        assert!(output.contains("500"));
        assert!(output.contains("Characters"));
    }

    #[test]
    fn test_format_single_quiet() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let output = format_single(&count, true, CountMode::Both);
        assert_eq!(output, "100 500");
    }

    #[test]
    fn test_format_single_quiet_words() {
        let count = Count {
            words: 42,
            characters: 500,
        };
        let output = format_single(&count, true, CountMode::Words);
        assert_eq!(output, "42");
    }

    #[test]
    fn test_format_table_multiple_files() {
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
        let output = format_table(&results, false, CountMode::Both);
        assert!(output.contains("file1.typ"));
        assert!(output.contains("file2.typ"));
        assert!(output.contains("100"));
        assert!(output.contains("200"));
        assert!(output.contains("500"));
        assert!(output.contains("1000"));
        assert!(output.contains("Total"));
        assert!(output.contains("300")); // total words
        assert!(output.contains("1500")); // total characters
    }

    #[test]
    fn test_format_table_quiet() {
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
        let output = format_table(&results, true, CountMode::Both);
        assert!(!output.contains("File"));
        assert!(!output.contains("Total"));
        assert!(output.contains("100 500"));
        assert!(output.contains("200 1000"));
    }

    #[test]
    fn test_format_header_both() {
        let header = format_header(10, CountMode::Both);
        assert!(header.contains("File"));
        assert!(header.contains("Words"));
        assert!(header.contains("Characters"));
    }

    #[test]
    fn test_format_header_words_only() {
        let header = format_header(10, CountMode::Words);
        assert!(header.contains("File"));
        assert!(header.contains("Words"));
        assert!(!header.contains("Characters"));
    }

    #[test]
    fn test_format_separator() {
        let sep = format_separator(10, CountMode::Both);
        assert!(sep.contains("─"));
        // Each "─" character is 3 bytes in UTF-8
        // Total width = 10 + 26 = 36 characters, but 108 bytes
        assert_eq!(sep.chars().count(), 36); // 36 characters

        let sep_words = format_separator(10, CountMode::Words);
        assert_eq!(sep_words.chars().count(), 23); // 23 characters
    }

    #[test]
    fn test_format_row_normal() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let row = format_row("test.typ", &count, 10, false, CountMode::Both);
        assert!(row.contains("test.typ"));
        assert!(row.contains("100"));
        assert!(row.contains("500"));
    }

    #[test]
    fn test_format_row_quiet() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let row = format_row("test.typ", &count, 10, true, CountMode::Both);
        assert_eq!(row, "100 500");
        assert!(!row.contains("test.typ"));
    }

    #[test]
    fn test_format_display_mode_auto_single_file() {
        let results = vec![(
            "test.typ".to_string(),
            Count {
                words: 100,
                characters: 500,
            },
        )];
        let output = format(&results, DisplayMode::Auto, CountMode::Both);
        // Should use simple format for single file
        assert!(output.contains("100"));
        assert!(output.contains("500"));
        assert!(!output.contains("Total")); // No total line for single file
    }

    #[test]
    fn test_format_display_mode_auto_multiple_files() {
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
        // Should use table format for multiple files
        assert!(output.contains("file1.typ"));
        assert!(output.contains("file2.typ"));
        assert!(output.contains("Total"));
    }

    #[test]
    fn test_format_display_mode_detailed() {
        let results = vec![(
            "test.typ".to_string(),
            Count {
                words: 100,
                characters: 500,
            },
        )];
        let output = format(&results, DisplayMode::Detailed, CountMode::Both);
        // Should use table format even for single file
        assert!(output.contains("test.typ"));
        assert!(output.contains("Total"));
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
        // Should show only total, no breakdown
        assert!(!output.contains("file1.typ"));
        assert!(!output.contains("file2.typ"));
        assert!(output.contains("300")); // total words
        assert!(output.contains("1500")); // total characters
    }

    #[test]
    fn test_format_display_mode_quiet() {
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
        let output = format(&results, DisplayMode::Quiet, CountMode::Both);
        // Should show only numbers, no labels
        assert_eq!(output.trim(), "300 1500");
    }
}
