use std::fs::File;
use std::path::Path;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use lines_are_rusty::{Page, LinesData};
use crate::pdf;

const REMARKABLE_NOTEBOOK_STORAGE_PATH: &str = "/home/root/.local/share/remarkable/xochitl/";
//const REMARKABLE_NOTEBOOK_STORAGE_PATH: &str = "/home/phaer/src/remarkable/xochitl/";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookMeta {
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
pub struct Notebook {
    pub name: String,
    pub id: String,
    pub metadata: NotebookMeta
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookContent {
    pub page_count: u32,
    pub pages: Vec<String>,
}

impl Notebook {
    pub fn content(&self) -> Result<NotebookContent> {
        let path = Path::new(REMARKABLE_NOTEBOOK_STORAGE_PATH).join(&self.id).with_extension("content");
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn parse_all(&self) -> Result<Vec<Page>> {
        let content = self.content()?;
        let mut pages = Vec::new();
        for page_id in content.pages {
            let path = Path::new(REMARKABLE_NOTEBOOK_STORAGE_PATH)
                .join(&self.id)
                .join(&page_id)
                .with_extension("rm");
            let mut file = File::open(path)?;
          pages.append(&mut LinesData::parse(&mut file).context("Failed to parse lines data")?.pages)
        }
        Ok(pages)
    }

    pub fn to_pdf(&self, output: &str) -> Result<()> {
        Ok(pdf::render(output, self.parse_all()?)?)
    }
}

pub fn list_notebooks(
) -> Result<Vec<Notebook>> {
    let mut result = Vec::new();
    let walker = WalkDir::new(REMARKABLE_NOTEBOOK_STORAGE_PATH)
        .into_iter()
        .filter(|e| e.as_ref().map_or(false, |e| e.file_name().to_string_lossy().ends_with(".metadata")));

    for entry in walker {
        let path = entry.as_ref().expect("Invalid files should have been filtered above").path();
        let notebook = get_notebook_by_path(path)?;
        result.push(notebook);
    }
    Ok(result)
}

pub fn get_notebook_by_path(path: &Path) -> Result<Notebook> {
        let file = File::open(path)
            .with_context(|| format!("Could not read metadata {:?}", path))?;

        let metadata: NotebookMeta = serde_json::from_reader(file)
            .with_context(|| format!("Could not parse metadata at {:?}", path))?;
        Ok(Notebook {
            name: metadata.visible_name.clone(),
            id: path.with_extension("").file_name().map(|n| n.to_string_lossy().into()).expect("Notebook without parseable id"),
            metadata
        })
}

pub fn get_notebook_by_id(id: String) -> Result<Notebook> {
    let path = Path::new(REMARKABLE_NOTEBOOK_STORAGE_PATH).join(&id).with_extension("metadata");
    get_notebook_by_path(path.as_path())
}
