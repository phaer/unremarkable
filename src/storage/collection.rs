use serde::{Deserialize, Serialize};
use super::item::Item;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(flatten)]
    pub metadata: Item,

    #[serde(flatten)]
    pub content: Content,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub tags: Vec<String>,
}

impl<'a> core::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "collection: {} #{}", self.metadata.visible_name, self.metadata.id)
    }
}
