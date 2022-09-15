//! # Storage
//!
//! Abstractions over `xochitl`´s [data store](#data-store) on the Remarkable 2.
//!
//!
//! ## Types
//! - Custom [`Error`](error::Error) & [`Result`](error::Result) types, enumerating possible failure modes using [SNAFU](snafu::Snafu).
//! - [`Item`](item::Item): An abstract entry in the data store.
//!   Can be queried for metadata or `.try_into()`´ed into a `Collection` or `Document`.
//! - [`Collection`](collection::Collection): A "directory" in `xochitl`.
//! - [`Document`](document::Document): An abstract document.
//!
//!
//!
//! ## Data Store
//! `xochitl`, Remarkable´s proprietary GUI for the tablet, stores it's data in
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
//! so we hard-code that at compile-time, unless `$REMARKABLE_NOTEBOOK_STORAGE_PATH` is set.
//!
//! ## Remarkable Lines
//! Remarkable stores drawings in a custom [file format](https://plasma.ninja/blog/devices/remarkable/binary/format/2017/12/26/reMarkable-lines-file-format.html),
//! which uses the `.rm` extension. We use [lines-are-rusty](https://github.com/ax3l/lines-are-rusty)
//! to parse and render them.

pub mod error;
pub mod item;

use std::path::PathBuf;

lazy_static::lazy_static! {
    pub static ref REMARKABLE_NOTEBOOK_STORAGE_PATH: PathBuf =
        std::env::var_os("REMARKABLE_NOTEBOOK_STORAGE_PATH")
        .map_or(PathBuf::from("/home/root/.local/share/remarkable/xochitl/"),
                PathBuf::from);
}
