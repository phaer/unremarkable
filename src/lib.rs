use std::fs::File;
use std::{fs, fmt::Display};
use std::path::{Path, PathBuf};
use snafu::prelude::*;
use serde::{Deserialize, Serialize};
use lines_are_rusty::{Page, LinesData, render_svg};

pub mod pdf;

lazy_static::lazy_static! {
    static ref REMARKABLE_NOTEBOOK_STORAGE_PATH: PathBuf =
        std::env::var_os("REMARKABLE_NOTEBOOK_STORAGE_PATH")
        .map_or(PathBuf::from("/home/root/.local/share/remarkable/xochitl/"),
                PathBuf::from);
}

type Result<T> = core::result::Result<T, Error>;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Page #{} does not exist in {}", number, id))]
    InvalidPage {
        number: usize,
        id: String
    },
    #[snafu(display("Unable to read file at  {}: {}", path.display(), source))]
    ReadFile { source: std::io::Error, path: PathBuf },
    #[snafu(display("Unable to write file at  {}: {}", path.display(), source))]
    WriteFile { source: std::io::Error, path: PathBuf },
    #[snafu(display("Unable to read metadata from {}: {}", path.display(), source))]
    ReadMetadata { source: std::io::Error, path: PathBuf },
    #[snafu(display("Unable to parse json at {}: {}", path.display(), source))]
    ParseJson { source: serde_json::Error, path: PathBuf },
    #[snafu(display("Unable to read xochitl store at {}: {}", path.display(), source))]
    ReadStore { source: std::io::Error, path: PathBuf },
    #[snafu(display("Unable to parse remarkable lines at {}: {}", path.display(), source))]
    ParseLines { source: lines_are_rusty::Error, path: PathBuf },


}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Notebook,
    EPub,
    PDF
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(skip_deserializing)]
    pub id: String,
    pub deleted: bool,
    pub last_modified: String,
    #[serde(default)]
    pub last_opened: Option<String>,
    #[serde(default)]
    pub last_opened_page: Option<u16>,
    pub metadatamodified: bool,
    pub modified: bool,
    pub parent: String,
    pub pinned: bool,
    pub synced: bool,
    #[serde(rename = "type")]
    pub type_: String,
    pub version: u8,
    pub visible_name: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub cover_page_number: usize,
    pub document_metadata: serde_json::Value,
    pub dummy_document: bool,
    pub extra_metadata: serde_json::Value,
    pub file_type: ContentType,
    pub font_name: String,
    pub format_version: usize,
    #[serde(default)]
    pub last_opened_page: Option<usize>,
    pub line_height: i32,
    pub margins: usize,
    pub orientation: String,
    pub original_page_count: i32,
    pub page_count: usize,
    pub pages: Vec<String>,
    pub page_tags: Vec<String>,
    pub redirection_page_map: Vec<usize>,
    pub size_in_bytes: String,
    pub tags: Vec<String>,
    pub text_alignment: String,
    pub text_scale: usize,
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.visible_name, self.id)
    }
}

impl Metadata {
    pub fn by_path(path: &Path) -> Result<Self> {
        let file = fs::File::open(path)
            .context(ReadMetadataSnafu { path: path.to_path_buf() })?;

        let mut metadata: Metadata = serde_json::from_reader(file)
            .context(ParseJsonSnafu {path: path.to_path_buf()})?;
        metadata.id = path
            .with_extension("")
            .file_name()
            .map(|n| n.to_string_lossy().into())
            .expect("Notebook without parseable id.");
        Ok(metadata)
    }

    pub fn by_id(id: String) -> Result<Self> {
        let path = REMARKABLE_NOTEBOOK_STORAGE_PATH
            .join(&id)
            .with_extension("metadata");
        Self::by_path(path.as_path())
    }

    pub fn all() -> Result<Vec<Metadata>> {
        let mut result = Vec::new();
        let documents = fs::read_dir(REMARKABLE_NOTEBOOK_STORAGE_PATH.as_path())
            .context(ReadStoreSnafu { path: REMARKABLE_NOTEBOOK_STORAGE_PATH.as_path()})?;
        for document in documents {
            let document = document.context(ReadFileSnafu { path: "" })?; // TODO
            if !document.file_name().to_string_lossy().ends_with(".metadata") {
                continue;
            }
            result.push(Self::by_path(&document.path())?)
        }
        Ok(result)
    }

    pub fn content(&self) -> Result<Content> {
        let path = REMARKABLE_NOTEBOOK_STORAGE_PATH.join(&self.id).with_extension("content");
        let file = fs::File::open(&path).context(ReadFileSnafu {path: &path})?;
        Ok(serde_json::from_reader(file).context(ParseJsonSnafu {path: &path})?)
    }

    fn parse_all_pages(&self) -> Result<Vec<Page>> {
        let content = self.content()?;
        let mut pages = Vec::new();
        for page_id in content.pages {
            let path = REMARKABLE_NOTEBOOK_STORAGE_PATH
                .join(&self.id)
                .join(&page_id)
                .with_extension("rm");
            let mut file = fs::File::open(&path).context(ReadFileSnafu {path: &path})?;
            pages.append(&mut LinesData::parse(&mut file).context(ParseLinesSnafu { path: &path })?.pages)
        }
        Ok(pages)
    }

    pub fn write_pdf(&self, path: &str) -> Result<()> {
        let parsed = self.parse_all_pages()?;
        let rendered = pdf::render(path, parsed)
            .context(WriteFileSnafu { path })?;
        Ok(rendered)
    }

    pub fn write_svg(&self, path: &str, index: usize) -> Result<()> {
        let mut output = File::create(path).context(WriteFileSnafu {path})?;
        let pages = self.parse_all_pages()?;
        let page = pages.get(index).ok_or(Error::InvalidPage { id: self.id.clone(), number: index })?;
        let auto_crop = false;
        let layer_colors = Default::default();
        let distance_threshold = 2.0;
        let template = None;
        let debug_dump = true;
        let rendered = render_svg(&mut output, page, auto_crop, layer_colors, distance_threshold, template, debug_dump)
            .context(ParseLinesSnafu {path})?;
        Ok(rendered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_list_notebooks() -> Result<()> {
        let metadatas = Metadata::all()?;
        // TODO
        assert_eq!(metadatas.len(), 26);
        Ok(())
    }

    #[test]
    fn it_fails_on_nonexistant_notebooks() {
        let id = "non-existant";
        let metadata = Metadata::by_id(id.to_string());
        println!("{:?}", metadata);
        assert!(metadata.is_err())
    }

    #[test]
    fn it_can_parse_epubs() -> Result<()> {
        let id = "7063a1a0-26e6-4941-aa0e-b8786aaf28bd";
        let metadata = Metadata::by_id(id.to_string())?;
        let content = metadata.content()?;
        assert_eq!(metadata.id, id);
        assert_eq!(metadata.visible_name, "The Rust Programming Language");
        assert_eq!(content.file_type, ContentType::EPub);

        Ok(())
    }
}
