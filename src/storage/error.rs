//! Custom [`Error`](Error) & [`Result`](Result) types using [SNAFU](snafu::Snafu) to bundle all storage-related
//! errors and provide context.

use snafu::Snafu;
use std::path::PathBuf;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Page #{} does not exist in {}", page, id))]
    InvalidPage { page: usize, id: uuid::Uuid },
    #[snafu(display("Unable to read file at  {}: {}", path.display(), source))]
    ReadFile {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("Unable to write file at  {}: {}", path.display(), source))]
    WriteFile {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("Unable to parse json at {}: {}", path.display(), source))]
    ParseJson {
        source: serde_json::Error,
        path: PathBuf,
    },
    #[snafu(display("Unable to read xochitl store at {}: {}", path.display(), source))]
    ReadStore {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("Unable to parse remarkable lines at {}: {}", path.display(), source))]
    ParseLines {
        source: lines_are_rusty::Error,
        path: PathBuf,
    },
    #[snafu(display("Invalid uuid: {}", source))]
    InvalidUuid { source: uuid::Error },

    #[snafu(display("Invalid type for #{}: {}", id, type_))]
    InvalidItemType { id: String, type_: String },
}

pub type Result<T> = core::result::Result<T, Error>;
