use crate::cli::{CountMode, DisplayMode};
use crate::counter::Count;
use crate::output::calculate_total;

pub fn format(results: &[(String, Count)], display: DisplayMode, mode: CountMode) -> String {
    if results.len() == 1 || display == DisplayMode::Total {
        let total = calculate_total(results);
        format_single(&total, mode)
    } else {
        format_array(results, mode)
    }
}

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
