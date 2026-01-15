mod csv;
mod human;
mod json;

use crate::cli::{CountMode, DisplayMode, OutputFormat};
use crate::counter::Count;

pub struct OutputFormatter {
    format: OutputFormat,
    mode: CountMode,
}

impl OutputFormatter {
    #[must_use]
    pub const fn new(format: OutputFormat, mode: CountMode) -> Self {
        Self { format, mode }
    }

    #[must_use]
    pub fn format_output(&self, results: &[(String, Count)], display: DisplayMode) -> String {
        match self.format {
            OutputFormat::Human => human::format(results, display, self.mode),
            OutputFormat::Json => json::format(results, display, self.mode),
            OutputFormat::Csv => csv::format(results, display, self.mode),
        }
    }
}

#[must_use]
pub fn calculate_total(results: &[(String, Count)]) -> Count {
    Count {
        words: results.iter().map(|(_, c)| c.words).sum(),
        characters: results.iter().map(|(_, c)| c.characters).sum(),
    }
}
