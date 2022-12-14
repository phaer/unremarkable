use serde::{Deserialize, Serialize};

use crate::utils::deserialize_parent;
use super::{Collection, Document};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(skip_deserializing)]
    pub id: uuid::Uuid,
    #[serde(rename="type")]
    pub type_: String,
    pub deleted: bool,
    pub last_modified: String,
    pub metadatamodified: bool,
    pub modified: bool,
    #[serde(deserialize_with = "deserialize_parent")]
    pub parent: Option<uuid::Uuid>,
    pub pinned: bool,
    pub synced: bool,
    pub version: u8,

    pub visible_name: String,
    #[serde(default)]
    pub last_opened: Option<String>,
    #[serde(default)]
    pub last_opened_page: Option<u16>,
}

#[derive(Debug)]
pub enum ItemType {
    Collection(Box<Collection>),
    Document(Box<Document>)
}


impl<'a> core::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.visible_name, self.id)
    }
}
