//! # Storage
//!
//! Abstractions over `xochitl`´s [data store](#data-store) on the Remarkable 2.
//!
//!
//! ## Types
//! - Custom [`Error`](error::Error) & [`Result`](error::Result) types, enumerating possible failure modes using [SNAFU](snafu::Snafu).
//! - [`Store`](Store): an abstract handle for a `xochitl` data store on disk.
//! - [`Item`](item::Item): An abstract entry in the data store.
//!   Can be queried for metadata or `.try_into()`´ed into a `Collection` or `Document`.
//! - [`Collection`](collection::Collection): A "directory" in `xochitl`.
//! - [`Document`](document::Document): An abstract document with an associated Trait
//!
//! ## Usage
//!
//! ```
//! use unremarkable_notes::storage::FileSystemStore;
//! let store = FileSystemStore::default();
//! assert!(store.path.as_path().ends_with("xochitl/"));
//! ```
//!
//! ## Data Store
//! `xochitl`, Remarkable´s proprietary GUI, stores its data in
//! a directory, containing JSON, EPub, PDF and [Remarkable Lines](#remarkable-lines) files.
//!
//! ## Files
//! - `{notebook_uuid}.metadata`: Deserialized to `Item`, Entry metadata, such as its name and whether it is a `Collection` or a `Document`.
//! - `{notebook_uuid}.content`:
//! - `{notebook_uuid}.pagedata`:
//! - `{notebook_uuid}/{page_uuid}.rm`:
//! - `{notebook_uuid}/{page_uuid}-metadata.json`:
//! - `{notebook_uuid}.thumbnails/{page_uuid}.jpg`:
//! - `{notebook_uuid}.highlights/{page_uuid}.json`:
//! - `{notebook_uuid}.pdf`:
//! - `{notebook_uuid}.epub`:
//! - `{notebook_uuid}.epubindex`:
//! - `{notebook_uuid}.metadata`:
//! - `{notebook_uuid}_{usize}.zip`: Contains a copy of that notebook, without `.metadata`, in a zip file.
//!     Might be related to sync? Going to investigate after implementing zip archives
//!     and notebook diffing.
//!
//!
//! ### Location
//!
//! On the Remarkable 2, the data store is located at `/home/root/.local/share/remarkable/xochitl/`,
//! so we use that as a default, unless `$UNREMARKABLE_STORAGE_PATH` is set during run-time.
//!
//! ## Remarkable Lines
//! Remarkable stores drawings in a custom [file format](https://plasma.ninja/blog/devices/remarkable/binary/format/2017/12/26/reMarkable-lines-file-format.html),
//! which uses the `.rm` extension. We use [lines-are-rusty](https://github.com/ax3l/lines-are-rusty)
//! to parse and render them.

mod error;
mod item;
mod collection;
mod document;

#[doc(inline)]
pub use {
    item::*,
    error::*,
    collection::*,
    document::*,
};

use std::path::{PathBuf, Path};
use uuid::Uuid;

#[derive(Debug)]
pub struct FileSystemStore {
    pub path: PathBuf,
}

pub trait Store: Sized {
    fn new<P>(path: P) -> Result<Self> where P: AsRef<Path>;
    fn all(&self) -> Result<Vec<Item>>;
    fn by_id<I>(&self, id: I) -> Result<Item> where I: AsRef<str>;
    fn by_path<P>(&self, path: P) -> Result<Item> where P: AsRef<Path>;
}

impl Store for FileSystemStore {
    fn new<T>(path: T) -> Result<Self>
        where T: AsRef<Path> {
        Ok(Self::try_from(path.as_ref())?)
    }

    fn all(self: &Self) -> Result<Vec<Item>> {
        let mut result = Vec::new();
        let documents =
            std::fs::read_dir(self.path.as_path()).context(ReadStoreSnafu {
                path: self.path.as_path(),
            })?;
        for document in documents {
            let document = document.context(ReadFileSnafu { path: "" })?; // TODO
            if !document
                .file_name()
                .to_string_lossy()
                .ends_with(".metadata")
            {
                continue;
            }
            result.push(Self::by_path(self, &document.path())?)
        }
        Ok(result)
    }

    fn by_id<I>(self: &Self, id: I) -> Result<Item> where I: AsRef<str> {
        let id = id.as_ref();
        let path = self.path.to_path_buf()
            .join(id)
            .with_extension("metadata");
        Self::by_path(self, path.as_path())
    }

    fn by_path<P>(self: &Self, path: P) -> Result<Item> where P: AsRef<Path> {
        let path = path.as_ref();
        let file = std::fs::File::open(path).context(ReadItemSnafu {
            path
        })?;
        let mut item: Item = serde_json::from_reader(file).context(ParseJsonSnafu {
            path: path.to_path_buf(),
        })?;
        let id: &str = &path
            .with_extension("")
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .expect("Notebook without parseable id.");
        item.id = Uuid::parse_str(id).context(InvalidUuidSnafu {})?;
        Ok(item)
    }
}

impl <'a>TryFrom<&Path> for FileSystemStore {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self> {
        path.metadata().context(ReadStoreSnafu { path })?;
        Ok(Self { path: path.to_path_buf() })
    }
}

impl <'a>Default for FileSystemStore {
    fn default() -> Self {
        let path = std::env::var_os("UNREMARKABLE_STORAGE_PATH")
            .map_or(PathBuf::from("/home/root/.local/share/remarkable/xochitl/"),
                    PathBuf::from);
        Self { path }
    }
}
