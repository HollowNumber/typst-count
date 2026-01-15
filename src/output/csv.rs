use crate::cli::{CountMode, DisplayMode};
use crate::counter::Count;
use crate::output::calculate_total;
use std::fmt::Write;

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

const fn format_header(mode: CountMode) -> &'static str {
    match mode {
        CountMode::Both => "file,words,characters",
        CountMode::Words => "file,words",
        CountMode::Characters => "file,characters",
    }
}

fn write_row(output: &mut String, name: &str, count: &Count, mode: CountMode) {
    let row = match mode {
        CountMode::Both => format!("{},{},{}", name, count.words, count.characters),
        CountMode::Words => format!("{},{}", name, count.words),
        CountMode::Characters => format!("{},{}", name, count.characters),
    };
    writeln!(output, "{row}").unwrap();
}
