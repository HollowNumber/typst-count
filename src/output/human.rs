use crate::cli::{CountMode, DisplayMode};
use crate::counter::Count;
use crate::output::calculate_total;
use std::fmt::Write;

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

fn format_separator(name_width: usize, mode: CountMode) -> String {
    let total_width = match mode {
        CountMode::Both => name_width + 26,
        _ => name_width + 13,
    };
    "─".repeat(total_width)
}

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
