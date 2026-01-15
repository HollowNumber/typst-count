pub mod cli;
pub mod counter;
pub mod output;
pub mod world;

use anyhow::{Context, Result};
use cli::Cli;
use counter::Count;
use std::path::Path;
use typst::{World, layout::PagedDocument};

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

pub fn process_files(args: &Cli) -> Result<Vec<(String, Count)>> {
    args.input
        .iter()
        .map(|path| {
            compile_document(path, args.exclude_imports)
                .map(|count| (path.display().to_string(), count))
        })
        .collect()
}

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
