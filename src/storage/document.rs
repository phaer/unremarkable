use super::{Store, item::Item, error::*};
use snafu::ResultExt;
use serde::{Deserialize, Serialize};
use std::path::Path;
use lines_are_rusty::{Page, LinesData, render_svg};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(flatten)]
    pub metadata: Item,

    #[serde(flatten)]
    pub content: Content,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub cover_page_number: isize,
    pub document_metadata: serde_json::Value,
    pub dummy_document: bool,
    pub extra_metadata: serde_json::Value,
    pub font_name: String,
    pub format_version: usize,
    pub file_type: String,
    #[serde(default)]
    pub last_opened_page: Option<usize>,
    pub line_height: i32,
    pub margins: usize,
    pub orientation: String,
    pub original_page_count: i32,
    pub page_count: usize,
    pub pages: Vec<uuid::Uuid>,
    pub page_tags: Vec<serde_json::Value>,
    #[serde(default)]
    pub redirection_page_map: Vec<isize>,
    pub size_in_bytes: String,
    pub tags: Vec<String>,
    pub text_alignment: String,
    pub text_scale: usize,
}


#[derive(Debug)]
pub enum DocumentType {
    Notebook(Notebook),
    Pdf(Pdf),
    Epub(Epub),
}

impl<'a> core::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} #{}", self.content.file_type, self.metadata.visible_name, self.metadata.id)
    }
}

// TODO maybe move to shared trait for Notebook, PDF, Epub.
impl Document {
    pub fn to_pdf(&self, store: &dyn Store, path: &Path) -> Result<()> {
        let parsed = self.pages(store)?;
        crate::render::render(path, parsed)
            .context(WriteFileSnafu { path })?;
        Ok(())
    }

    pub fn to_svg(&self, store: &dyn Store, path: &Path, page: usize) -> Result<()> {
        let mut output = std::fs::File::create(path).context(WriteFileSnafu {path})?;
        let pages = self.pages(store)?;
        let page = pages.get(page).ok_or(Error::InvalidPage { id: self.metadata.id, page })?;
        let auto_crop = false;
        let layer_colors = Default::default();
        let distance_threshold = 2.0;
        let template = None;
        let debug_dump = true;
        let rendered = render_svg(&mut output, page, auto_crop, layer_colors, distance_threshold, template, debug_dump)
            .context(ParseLinesSnafu {path})?;
        Ok(rendered)

    }
    pub fn pages(&self, store: &dyn Store) -> Result<Vec<Page>> {
        let mut pages = Vec::new();
        for page_id in &self.content.pages {
            let path = &Path::new(&self.metadata.id.to_string())
                .join(&page_id.to_string())
                .with_extension("rm");
            let mut file = store.get_file(path)?;
            pages.append(&mut LinesData::parse(&mut file).context(ParseLinesSnafu { path: &path })?.pages)
        }
        Ok(pages)
    }

//impl FileType {
//    pub fn content(&self) -> Result<ContentType> {
//        let item = self.item();
//        let path = &REMARKABLE_NOTEBOOK_STORAGE_PATH
//            .join(item.id.to_string())
//            .with_extension("content");
//        let file = fs::File::open(path).context(ReadFileSnafu { path })?;
//        let json: serde_json::Value =
//            serde_json::from_reader(file).context(ParseJsonSnafu { path })?;
//        let content_type = match json.get("fileType").and_then(|v| v.as_str()) {
//            Some("notebook") | Some("epub") | Some("pdf") => {
//                serde_json::from_value(json).context(ParseJsonSnafu { path })?
//            }
//            None | Some(_) => ContentType::Collection(
//                serde_json::from_value(json).context(ParseJsonSnafu { path })?,
//            ),
//        };
//        Ok(content_type)
//    }
//}

}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notebook {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pdf {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Epub {}
