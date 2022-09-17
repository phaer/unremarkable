use super::{Store, item::Item, error::*};
use serde::{Deserialize, Serialize};
use std::path::Path;
use lines_are_rusty::{Page, LinesData};

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


trait DocumentLike {
    fn to_pdf<T>(&self, store: &dyn Store, output: &Path) -> Result<()>;
    fn to_svg<T>(&self, store: &dyn Store, output: &Path, page: usize) -> Result<()>;
    fn pages(&self, store: &dyn Store) -> Result<Vec<Page>>;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notebook {}


// TODO maybe implement for Notebook, PDF, Epub, not Document
impl DocumentLike for Document {
    fn to_pdf<T>(&self, store: &dyn Store, output: &Path) -> Result<()> {
        unimplemented!();
    }
    fn to_svg<T>(&self, store: &dyn Store, output: &Path, page: usize) -> Result<()> {
        unimplemented!();
    }
    fn pages(&self, store: &dyn Store) -> Result<Vec<Page>> {
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pdf {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Epub {}
