pub mod pdf;
pub mod storage;

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use std::path::Path;
//use std::fs::File;
use std::{fmt::Display, fs};
//use lines_are_rusty::{Page, LinesData, render_svg};

use storage::{
    REMARKABLE_NOTEBOOK_STORAGE_PATH,
    error::*,
    item::Item
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum FileType {
    #[serde(rename = "CollectionType")]
    Collection {
        #[serde(flatten)]
        item: Item,
    },
    #[serde(rename = "DocumentType")]
    Document {
        #[serde(flatten)]
        item: Item,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "fileType")]
pub enum ContentType {
    Collection(CollectionContent),
    Notebook(Content),
    EPub(Content),
    PDF(Content),
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // TODO add reference to item for visibleName
                ContentType::Collection(_) => "collection",
                ContentType::Notebook(_) => "notebook",
                ContentType::EPub(_) => "epub",
                ContentType::PDF(_) => "pdf",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CollectionContent {
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub cover_page_number: isize,
    pub document_metadata: serde_json::Value,
    pub dummy_document: bool,
    pub extra_metadata: serde_json::Value,
    pub font_name: String,
    pub format_version: usize,
    #[serde(default)]
    pub last_opened_page: Option<usize>,
    pub line_height: i32,
    pub margins: usize,
    pub orientation: String,
    pub original_page_count: i32,
    pub page_count: usize,
    pub pages: Vec<Uuid>,
    pub page_tags: Vec<serde_json::Value>,
    #[serde(default)]
    pub redirection_page_map: Vec<isize>,
    pub size_in_bytes: String,
    pub tags: Vec<String>,
    pub text_alignment: String,
    pub text_scale: usize,
}

impl FileType {
    pub fn by_path(path: &Path) -> Result<Self> {
        let file = fs::File::open(path).context(ReadItemSnafu {
            path: path.to_path_buf(),
        })?;
        let mut file: FileType = serde_json::from_reader(file).context(ParseJsonSnafu {
            path: path.to_path_buf(),
        })?;
        let id: &str = &path
            .with_extension("")
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .expect("Notebook without parseable id.");
        let mut item: &mut Item = file.item_mut();
        item.id = Uuid::parse_str(id).context(InvalidUuidSnafu {})?;
        Ok(file)
    }

    pub fn by_id(id: String) -> Result<Self> {
        let path = REMARKABLE_NOTEBOOK_STORAGE_PATH
            .join(&id)
            .with_extension("metadata");
        Self::by_path(path.as_path())
    }

    pub fn all() -> Result<Vec<FileType>> {
        let mut result = Vec::new();
        let documents =
            fs::read_dir(REMARKABLE_NOTEBOOK_STORAGE_PATH.as_path()).context(ReadStoreSnafu {
                path: REMARKABLE_NOTEBOOK_STORAGE_PATH.as_path(),
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
            result.push(Self::by_path(&document.path())?)
        }
        Ok(result)
    }

    pub fn item_mut(&mut self) -> &mut Item {
        match self {
            FileType::Collection { item } => item,
            FileType::Document { item } => item,
        }
    }

    pub fn item(&self) -> &Item {
        match self {
            FileType::Collection { item } => item,
            FileType::Document { item } => item,
        }
    }
    pub fn content(&self) -> Result<ContentType> {
        let item = self.item();
        let path = &REMARKABLE_NOTEBOOK_STORAGE_PATH
            .join(item.id.to_string())
            .with_extension("content");
        let file = fs::File::open(path).context(ReadFileSnafu { path })?;
        let json: serde_json::Value =
            serde_json::from_reader(file).context(ParseJsonSnafu { path })?;
        let content_type = match json.get("fileType").and_then(|v| v.as_str()) {
            Some("notebook") | Some("epub") | Some("pdf") => {
                serde_json::from_value(json).context(ParseJsonSnafu { path })?
            }
            None | Some(_) => ContentType::Collection(
                serde_json::from_value(json).context(ParseJsonSnafu { path })?,
            ),
        };
        Ok(content_type)
    }

    //    pub fn content(&self) -> Result<Content> {
    //        let path = REMARKABLE_NOTEBOOK_STORAGE_PATH.join(&self.id.to_string()).with_extension("content");
    //        let file = fs::File::open(&path).context(ReadFileSnafu {path: &path})?;
    //        Ok(serde_json::from_reader(file).context(ParseJsonSnafu {path: &path})?)
    //    }
    //
    //    fn parse_all_pages(&self) -> Result<Vec<Page>> {
    //        let content = self.content()?;
    //        let mut pages = Vec::new();
    //        for page_id in content.pages {
    //            let path = REMARKABLE_NOTEBOOK_STORAGE_PATH
    //                .join(&self.id.to_string())
    //                .join(&page_id.to_string())
    //                .with_extension("rm");
    //            let mut file = fs::File::open(&path).context(ReadFileSnafu {path: &path})?;
    //            pages.append(&mut LinesData::parse(&mut file).context(ParseLinesSnafu { path: &path })?.pages)
    //        }
    //        Ok(pages)
    //    }
    //
    //    pub fn write_pdf(&self, path: &str) -> Result<()> {
    //        let parsed = self.parse_all_pages()?;
    //        let rendered = pdf::render(path, parsed)
    //            .context(WriteFileSnafu { path })?;
    //        Ok(rendered)
    //    }
    //
    //    pub fn write_svg(&self, path: &str, index: usize) -> Result<()> {
    //        let mut output = File::create(path).context(WriteFileSnafu {path})?;
    //        let pages = self.parse_all_pages()?;
    //        let page = pages.get(index).ok_or(Error::InvalidPage { id: self.id.clone(), number: index })?;
    //        let auto_crop = false;
    //        let layer_colors = Default::default();
    //        let distance_threshold = 2.0;
    //        let template = None;
    //        let debug_dump = true;
    //        let rendered = render_svg(&mut output, page, auto_crop, layer_colors, distance_threshold, template, debug_dump)
    //            .context(ParseLinesSnafu {path})?;
    //        Ok(rendered)
    //    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_list_and_parse_all_notebooks() -> Result<()> {
        let files = FileType::all()?;
        assert!(files.len() > 0);
        for file in files {
            let item = file.item();
            let content = file.content()?;
            println!(
                "{} #{}: \"{}\"",
                content, item.id, item.visible_name
            )
        }

        Ok(())
    }

    #[test]
    fn it_fails_on_nonexistant_notebooks() {
        let id = "non-existant";
        let file = FileType::by_id(id.to_string());
        assert!(file.is_err())
    }

    #[test]
    fn it_can_parse_epubs() -> Result<()> {
        let id = "7063a1a0-26e6-4941-aa0e-b8786aaf28bd";
        let file = FileType::by_id(id.to_string())?;
        let item = file.item();
        let content = file.content()?;
        assert_eq!(
            item.id,
            Uuid::parse_str(id).expect("Could not parse test id")
        );
        assert_eq!(item.visible_name, "The Rust Programming Language");
        if let ContentType::EPub(_) = content {
            println!("Content-Type: Epub")
        };
        Ok(())
    }
}
