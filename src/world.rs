//! A minimal implementation of Typst's World trait for document compilation.
//!
//! This module provides a simple world implementation that allows loading and
//! compiling Typst documents from the filesystem. It handles file resolution,
//! source loading, and provides the minimal context needed for compilation.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

/// A minimal implementation of Typst's `World` trait for standalone compilation.
///
/// This struct provides the bare minimum functionality needed to compile Typst
/// documents. It handles file system access, source loading, and maintains
/// references to the Typst standard library.
///
/// # Limitations
///
/// - Does not load system fonts (returns `None` for font queries)
/// - Uses a fixed date for compilation reproducibility
/// - Resolves files relative to the main document's directory
///
/// # Examples
///
/// ```no_run
/// use typst_count::world::SimpleWorld;
/// use typst::World;
/// use std::path::Path;
///
/// let world = SimpleWorld::new(Path::new("document.typ"))?;
/// let main_id = world.main();
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct SimpleWorld {
    /// The Typst standard library
    library: LazyHash<Library>,
    /// Font book (currently empty as we don't load fonts)
    book: LazyHash<FontBook>,
    /// File ID of the main document
    main: FileId,
    /// Root directory for resolving relative paths
    root: PathBuf,
}

impl SimpleWorld {
    /// Creates a new `SimpleWorld` for compiling a Typst document.
    ///
    /// This function initializes the compilation environment by:
    /// 1. Canonicalizing the main file path
    /// 2. Setting the root directory to the file's parent directory
    /// 3. Creating a virtual path for the main file
    /// 4. Initializing the Typst standard library
    ///
    /// # Arguments
    ///
    /// * `main_path` - Path to the main Typst document to compile
    ///
    /// # Returns
    ///
    /// A new `SimpleWorld` instance ready for compilation, or an error if
    /// the file cannot be found or has no parent directory.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file path cannot be canonicalized (file doesn't exist)
    /// - The file has no parent directory
    /// - The file has no filename component
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use typst_count::world::SimpleWorld;
    /// use std::path::Path;
    ///
    /// let world = SimpleWorld::new(Path::new("document.typ"))?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn new(main_path: &Path) -> Result<Self> {
        let main_path = main_path
            .canonicalize()
            .context("Failed to find input file")?;

        let root = main_path
            .parent()
            .context("Input file has no parent directory")?
            .to_path_buf();

        let vpath = VirtualPath::new(
            main_path
                .file_name()
                .context("Input file has no filename")?,
        );
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
    /// Returns a reference to the Typst standard library.
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    /// Returns a reference to the font book.
    ///
    /// Note: This implementation returns an empty font book as we don't
    /// load system fonts for word counting purposes.
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    /// Returns the file ID of the main document.
    fn main(&self) -> FileId {
        self.main
    }

    /// Loads the source code for a given file ID.
    ///
    /// This method resolves the file path (either absolute or relative to the
    /// root directory) and reads the file contents as a UTF-8 string.
    ///
    /// # Arguments
    ///
    /// * `id` - The file ID to load
    ///
    /// # Returns
    ///
    /// A `Source` object containing the file's content and ID, or a file error
    /// if the file cannot be read.
    fn source(&self, id: FileId) -> FileResult<Source> {
        let path = if id.vpath().as_rootless_path().is_absolute() {
            id.vpath().as_rootless_path().to_path_buf()
        } else {
            self.root.join(id.vpath().as_rootless_path())
        };

        let content = std::fs::read_to_string(&path).map_err(|e| FileError::from_io(e, &path))?;

        Ok(Source::new(id, content))
    }

    /// Loads binary data for a given file ID.
    ///
    /// This method resolves the file path and reads the file contents as raw bytes.
    /// Used for loading images, fonts, and other binary assets referenced by the document.
    ///
    /// # Arguments
    ///
    /// * `id` - The file ID to load
    ///
    /// # Returns
    ///
    /// A `Bytes` object containing the file's binary content, or a file error
    /// if the file cannot be read.
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let path = if id.vpath().as_rootless_path().is_absolute() {
            id.vpath().as_rootless_path().to_path_buf()
        } else {
            self.root.join(id.vpath().as_rootless_path())
        };

        let content = std::fs::read(&path).map_err(|e| FileError::from_io(e, &path))?;
        Ok(Bytes::new(content))
    }

    /// Returns a font at the given index.
    ///
    /// This implementation always returns `None` as we don't load system fonts.
    /// For word counting purposes, font information is not needed since we only
    /// extract text content from the compiled document.
    fn font(&self, _index: usize) -> Option<Font> {
        None
    }

    /// Returns the current date for compilation.
    ///
    /// This implementation returns a fixed date (2024-01-01) for reproducibility.
    /// The date is used by Typst's `datetime.today()` function but doesn't affect
    /// word counting results.
    ///
    /// # Arguments
    ///
    /// * `_offset` - UTC offset in hours (ignored in this implementation)
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        Some(Datetime::from_ymd(2024, 1, 1).unwrap())
    }
}
