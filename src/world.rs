use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

pub struct SimpleWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    main: FileId,
    root: PathBuf,
}

impl SimpleWorld {
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
