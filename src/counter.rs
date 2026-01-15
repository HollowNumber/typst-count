//! Document counting logic for Typst documents.
//!
//! This module provides functionality to count words and characters in compiled
//! Typst documents by traversing the document's element tree and extracting
//! rendered text content.

use typst::introspection::Introspector;
use typst::math::EquationElem;
use typst::model::{EmphElem, StrongElem};
use typst::syntax::FileId;
use typst::text::{OverlineElem, RawElem, StrikeElem, SubElem, SuperElem, UnderlineElem};

/// Result of counting words and characters in a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Count {
    /// Total number of words in the document.
    ///
    /// Words are counted by splitting on whitespace, which works well for
    /// space-separated languages but may not be accurate for languages like
    /// Chinese or Japanese where words are not separated by spaces.
    pub words: usize,

    /// Total number of characters in the document.
    ///
    /// This includes all rendered characters including spaces and punctuation,
    /// but excludes markup syntax that doesn't appear in the rendered output.
    pub characters: usize,
}

/// Counts words and characters in a compiled Typst document.
///
/// This function traverses all elements in the document using the introspector
/// and extracts plain text content. It handles the following cases:
///
/// - **Text styling**: Skips styling elements (bold, italic, etc.) to avoid
///   double-counting since their text is already included in parent elements.
/// - **Math equations**: Skips mathematical notation to avoid counting math symbols as words.
/// - **Imports**: Optionally excludes text from imported/included files.
/// - **Rendered content**: Only counts text that appears in the final rendered
///   document, ignoring code, comments, and markup syntax.
///
/// # Arguments
///
/// * `introspector` - The Typst introspector providing access to document elements
/// * `exclude_imports` - If `true`, only counts text from the main file
/// * `main_file_id` - File ID of the main document (used when `exclude_imports` is `true`)
///
/// # Returns
///
/// A `Count` struct containing the word and character counts.
///
/// # Examples
///
/// ```ignore
/// use typst_count::count_document;
///
/// let count = count_document(&introspector, false, main_file_id);
/// println!("Words: {}, Characters: {}", count.words, count.characters);
/// ```
///
/// # Counting Method
///
/// - **Words**: Split by Unicode whitespace (equivalent to Rust's `split_whitespace()`)
/// - **Characters**: Total Unicode scalar values (equivalent to Rust's `chars().count()`)
///
/// # Avoiding Double-Counting
///
/// Typst's document tree includes both container elements and their styled children.
/// For example, `*bold text*` creates:
/// - A paragraph element containing "bold text"
/// - A `strong` element also containing "bold text"
///
/// To avoid counting the same text twice, we skip known styling elements whose
/// content is already included in their parent elements.
pub fn count_document(
    introspector: &Introspector,
    exclude_imports: bool,
    main_file_id: FileId,
) -> Count {
    let mut words = 0;
    let mut characters = 0;

    for element in introspector.all() {
        // Skip elements from imported/included files if requested
        if exclude_imports
            && let Some(file_id) = element.span().id()
            && file_id != main_file_id
        {
            continue;
        }

        // Skip styling elements to avoid double-counting.
        // These elements' text is already included in their parent elements
        // (typically paragraphs or other text containers).
        if is_styling_element(element) {
            continue;
        }

        let text = element.plain_text();
        if !text.is_empty() {
            characters += text.chars().count();
            words += text.split_whitespace().count();
        }
    }

    Count { words, characters }
}

/// Checks if an element is a text styling element that should be skipped during counting.
///
/// Text styling elements (like bold, italic, underline) wrap text content but don't
/// add new text. Their content is already included in parent elements, so counting
/// them would result in double-counting.
///
/// # Arguments
///
/// * `element` - The element to check
///
/// # Returns
///
/// `true` if the element is a styling element that should be skipped, `false` otherwise.
///
/// # Styling Elements
///
/// The following element types are considered styling elements:
/// - `strong` - Bold text (`*bold*`)
/// - `emph` - Italic/emphasis text (`_italic_`)
/// - `underline` - Underlined text
/// - `strike` - Strikethrough text
/// - `overline` - Overlined text
/// - `sub` - Subscript text
/// - `super` - Superscript text
/// - `highlight` - Highlighted text
/// - `equation` - Math equations (`$...$` or `$ ... $`)
///
/// # Examples
///
/// ```ignore
/// if is_styling_element(&element) {
///     // Skip this element to avoid double-counting
///     continue;
/// }
/// ```
fn is_styling_element(element: &typst::foundations::Content) -> bool {
    element.is::<StrongElem>()
        || element.is::<EmphElem>()
        || element.is::<UnderlineElem>()
        || element.is::<StrikeElem>()
        || element.is::<OverlineElem>()
        || element.is::<SubElem>()
        || element.is::<SuperElem>()
        || element.is::<EquationElem>() // Skip math equations
        || element.is::<RawElem>()
        || element.func().name() == "highlight" // highlight doesn't have a public struct
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_struct_creation() {
        let count = Count {
            words: 42,
            characters: 256,
        };
        assert_eq!(count.words, 42);
        assert_eq!(count.characters, 256);
    }

    #[test]
    fn test_count_equality() {
        let count1 = Count {
            words: 10,
            characters: 50,
        };
        let count2 = Count {
            words: 10,
            characters: 50,
        };
        let count3 = Count {
            words: 11,
            characters: 50,
        };

        assert_eq!(count1, count2);
        assert_ne!(count1, count3);
    }
}
