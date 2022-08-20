use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use ssh2::Sftp;

const REMARKABLE_NOTEBOOK_STORAGE_PATH: &str = "/home/root/.local/share/remarkable/xochitl/";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NotebookMeta {
    deleted: bool,
    last_modified: String,
    #[serde(default)]
    last_opened: Option<String>,
    #[serde(default)]
    last_opened_page: Option<u16>,
    metadatamodified: bool,
    modified: bool,
    parent: String,
    pinned: bool,
    synced: bool,
    #[serde(rename = "type")]
    type_: String,
    version: u8,
    visible_name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notebook {
    name: String,
    path: PathBuf,
    metadata: NotebookMeta
}

pub fn list_notebooks(
    sftp: Sftp
) -> Result<Vec<Notebook>> {
    let path = Path::new(REMARKABLE_NOTEBOOK_STORAGE_PATH);
    let mut result = Vec::new();
    let files = sftp.readdir(path).context("Could not list files in storage directory")?;
    for (path_buffer, _file_stat) in files {
        if path_buffer.extension().map_or (false, |v| v == "metadata") {
            let metadata_file = sftp
                .open(&path_buffer)
                .with_context(|| format!("Could not read metadata {:?}", path_buffer))?;
            let metadata: NotebookMeta = serde_json::from_reader(metadata_file)
                .with_context(|| format!("Could not parse metadata at {:?}", path_buffer))?;
            let notebook = Notebook {
                name: metadata.visible_name.clone(),
                path: path_buffer.with_extension(""),
                metadata
            };
            result.push(notebook);
       }
    }
    Ok(result)
}
