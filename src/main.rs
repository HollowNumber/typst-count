use anyhow::{Context, Result, bail};
use clap::Parser;
use std::path::{Path, PathBuf};
use typst::LibraryExt;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::introspection::Introspector;
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, World};

#[derive(Parser)]
#[command(name = "typst-count")]
#[command(about = "Count words and characters in Typst documents", long_about = None)]
struct Cli {
    /// Typst document file to count
    file: PathBuf,

    /// Count only words
    #[arg(short = 'w', long)]
    words: bool,

    /// Count only characters
    #[arg(short = 'c', long, conflicts_with = "words")]
    characters: bool,

    /// Exclude imported files from count
    #[arg(short = 'e', long)]
    exclude_imports: bool,
}

struct SimpleWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    main: FileId,
    root: PathBuf,
}

impl SimpleWorld {
    fn new(main_path: &Path) -> Result<Self> {
        // Get the absolute path of the main file first
        let main_path = main_path
            .canonicalize()
            .context("Failed to canonicalize main file path")?;

        let root = main_path
            .parent()
            .context("Main file has no parent directory")?
            .to_path_buf();

        // Create FileId using new_fake for a simple case
        let vpath = VirtualPath::new(main_path.file_name().context("Main file has no filename")?);
        let main = FileId::new_fake(vpath);

        Ok(Self {
            library: LazyHash::new(Library::builder().build()),
            book: LazyHash::new(FontBook::new()),
            main,
            root,
        })
    }
}

impl World for SimpleWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        let path = if id.vpath().as_rootless_path().is_absolute() {
            id.vpath().as_rootless_path().to_path_buf()
        } else {
            self.root.join(id.vpath().as_rootless_path())
        };

        let content = std::fs::read_to_string(&path).map_err(|e| FileError::from_io(e, &path))?;

        Ok(Source::new(id, content))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let path = if id.vpath().as_rootless_path().is_absolute() {
            id.vpath().as_rootless_path().to_path_buf()
        } else {
            self.root.join(id.vpath().as_rootless_path())
        };

        let content = std::fs::read(&path).map_err(|e| FileError::from_io(e, &path))?;
        Ok(Bytes::new(content))
    }

    fn font(&self, _index: usize) -> Option<Font> {
        None
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        Some(Datetime::from_ymd(2024, 1, 1).unwrap())
    }
}

fn count_text(
    introspector: &Introspector,
    exclude_imports: bool,
    main_file_id: FileId,
) -> (usize, usize) {
    let mut total_words = 0;
    let mut total_chars = 0;

    for element in introspector.all() {
        // If exclude_imports is enabled, skip elements not from the main file
        if exclude_imports
            && let Some(file_id) = element.span().id()
            && file_id != main_file_id
        {
            continue;
        }

        let text = element.plain_text();

        if !text.is_empty() {
            total_chars += text.chars().count();
            total_words += text.split_whitespace().count();
        }
    }

    (total_words, total_chars)
}

fn process_file(path: &Path, exclude_imports: bool) -> Result<(usize, usize)> {
    let world = SimpleWorld::new(path)?;
    let main_file_id = world.main();

    // Compile the document
    let result = typst::compile(&world);

    let document: PagedDocument = match result.output {
        Ok(document) => document,
        Err(errors) => {
            bail!("Failed to compile {}: {:?}", path.display(), errors)
        }
    };

    let counts = count_text(&document.introspector, exclude_imports, main_file_id);
    Ok(counts)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let (words, chars) = process_file(&cli.file, cli.exclude_imports)?;

    let show_both = !cli.words && !cli.characters;

    if show_both || cli.characters {
        println!("Characters: {chars}");
    }
    if show_both || cli.words {
        println!("Words: {words}");
    }

    Ok(())
}
