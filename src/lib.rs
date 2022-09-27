#![doc(test(attr(deny(warnings))))]
#![doc(issue_tracker_base_url = "https://github.com/phaer/unremarkable/issues/")]

pub mod config;
pub mod storage;
pub mod sync;
pub mod pdf;

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::storage::{Store, FileSystemStore, ItemType, error::Result};

    #[test]
    fn it_can_list_and_parse_all_notebooks() -> Result<()> {
        let store = FileSystemStore::default();
        let items = store.all()?;
        assert!(items.len() > 0);
        for item in items {
           match store.load(&item.id.to_string())? {
                ItemType::Collection(c) => println!("as collection: {:?}", c),
                ItemType::Document(d) => println!("as document: {:?}", d),
            }
        }
        Ok(())
    }

    #[test]
    fn it_fails_on_nonexistant_notebooks() {
        let id = "non-existant";
        let store = FileSystemStore::default();
        let file = store.by_id(id);
        assert!(file.is_err())
    }

    #[test]
    fn it_can_parse_epubs() -> Result<()> {
        let id = "7063a1a0-26e6-4941-aa0e-b8786aaf28bd";
        let store = FileSystemStore::default();
        let item = store.by_id(id)?;
        assert_eq!(
            item.id,
            Uuid::parse_str(id).expect("Could not parse test id")
        );
        assert_eq!(item.visible_name, "The Rust Programming Language");
        Ok(())
    }
}
