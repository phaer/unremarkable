use serde::{de::IntoDeserializer, Deserialize, Serialize};

use super::{Collection, Document, error::*};

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
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
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
    Collection(Collection),
    Document(Document)
}


impl<'a> core::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.visible_name, self.id)
    }
}


// https://github.com/serde-rs/serde/issues/1425#issuecomment-462282398
fn deserialize_empty_string_as_none<'de, D, T>(de: D) -> core::result::Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_deref();
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}
