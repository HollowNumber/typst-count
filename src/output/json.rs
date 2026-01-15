//! JSON output formatting.
//!
//! This module provides functions to format count results as JSON,
//! suitable for machine processing and integration with other tools.

use crate::cli::{CountMode, DisplayMode};
use crate::counter::Count;
use crate::output::calculate_total;

/// Formats count results as JSON.
///
/// Produces valid JSON output, either as a single object for one file
/// or as an array of objects for multiple files.
///
/// # Arguments
///
/// * `results` - Slice of file paths and their counts
/// * `display` - Display mode controlling output structure
/// * `mode` - What to include in the output (words/characters/both)
///
/// # Returns
///
/// A JSON string representing the count results.
pub fn format(results: &[(String, Count)], display: DisplayMode, mode: CountMode) -> String {
    if results.len() == 1 || display == DisplayMode::Total {
        let total = calculate_total(results);
        format_single(&total, mode)
    } else {
        format_array(results, mode)
    }
}

/// Formats a single count as a JSON object.
///
/// # Arguments
///
/// * `count` - The count to format
/// * `mode` - What fields to include (words/characters/both)
fn format_single(count: &Count, mode: CountMode) -> String {
    match mode {
        CountMode::Both => {
            format!(
                r#"{{"words":{},"characters":{}}}"#,
                count.words, count.characters
            )
        }
        CountMode::Words => format!(r#"{{"words":{}}}"#, count.words),
        CountMode::Characters => format!(r#"{{"characters":{}}}"#, count.characters),
    }
}

/// Formats multiple counts as a JSON array.
///
/// # Arguments
///
/// * `results` - Slice of file paths and their counts
/// * `mode` - What fields to include in each object (words/characters/both)
fn format_array(results: &[(String, Count)], mode: CountMode) -> String {
    let mut output = String::from("[\n");
    for (i, (name, count)) in results.iter().enumerate() {
        let comma = if i < results.len() - 1 { "," } else { "" };
        let entry = format_entry(name, count, mode, comma);
        output.push_str(&entry);
        output.push('\n');
    }
    output.push(']');
    output
}

/// Formats a single entry in a JSON array.
///
/// # Arguments
///
/// * `name` - File name to include in the object
/// * `count` - Count values to include
/// * `mode` - What fields to include (words/characters/both)
/// * `comma` - Trailing comma for array formatting
fn format_entry(name: &str, count: &Count, mode: CountMode, comma: &str) -> String {
    match mode {
        CountMode::Both => {
            format!(
                r#"  {{"file":"{}","words":{},"characters":{}}}{}"#,
                name, count.words, count.characters, comma
            )
        }
        CountMode::Words => {
            format!(
                r#"  {{"file":"{}","words":{}}}{}"#,
                name, count.words, comma
            )
        }
        CountMode::Characters => {
            format!(
                r#"  {{"file":"{}","characters":{}}}{}"#,
                name, count.characters, comma
            )
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
        let output = format_single(&count, CountMode::Both);
        assert_eq!(output, r#"{"words":100,"characters":500}"#);
    }

    #[test]
    fn test_format_single_words_only() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let output = format_single(&count, CountMode::Words);
        assert_eq!(output, r#"{"words":100}"#);
    }

    #[test]
    fn test_format_single_characters_only() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let output = format_single(&count, CountMode::Characters);
        assert_eq!(output, r#"{"characters":500}"#);
    }

    #[test]
    fn test_format_entry_both() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let entry = format_entry("test.typ", &count, CountMode::Both, ",");
        assert_eq!(
            entry,
            r#"  {"file":"test.typ","words":100,"characters":500},"#
        );
    }

    #[test]
    fn test_format_entry_no_comma() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let entry = format_entry("test.typ", &count, CountMode::Both, "");
        assert_eq!(
            entry,
            r#"  {"file":"test.typ","words":100,"characters":500}"#
        );
    }

    #[test]
    fn test_format_entry_words_only() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let entry = format_entry("test.typ", &count, CountMode::Words, "");
        assert_eq!(entry, r#"  {"file":"test.typ","words":100}"#);
    }

    #[test]
    fn test_format_entry_characters_only() {
        let count = Count {
            words: 100,
            characters: 500,
        };
        let entry = format_entry("test.typ", &count, CountMode::Characters, "");
        assert_eq!(entry, r#"  {"file":"test.typ","characters":500}"#);
    }

    #[test]
    fn test_format_array() {
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
        let output = format_array(&results, CountMode::Both);
        assert!(output.starts_with("[\n"));
        assert!(output.ends_with(']'));
        assert!(output.contains(r#""file":"file1.typ""#));
        assert!(output.contains(r#""file":"file2.typ""#));
        assert!(output.contains(r#""words":100"#));
        assert!(output.contains(r#""words":200"#));
        assert!(output.contains(r#""characters":500"#));
        assert!(output.contains(r#""characters":1000"#));
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
        assert_eq!(output, r#"{"words":100,"characters":500}"#);
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
        assert!(output.starts_with("[\n"));
        assert!(output.contains(r#""file":"file1.typ""#));
        assert!(output.contains(r#""file":"file2.typ""#));
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
        // Should show only total as single object
        assert_eq!(output, r#"{"words":300,"characters":1500}"#);
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
        assert_eq!(output, r#"{"words":42}"#);
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
        assert_eq!(output, r#"{"characters":200}"#);
        assert!(!output.contains("words"));
    }
}
