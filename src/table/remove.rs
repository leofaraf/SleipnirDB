use std::{collections::HashSet, hash, sync::{Arc, Mutex}};

use serde::{de::DeserializeOwned, Serialize};

use crate::{database::ExDatabase, error};

use super::ExTable;

pub trait RemoveExTable<T>
where T: Eq + hash::Hash + Serialize + DeserializeOwned {
    fn remove_item(&mut self, item: &T) -> Result<(), error::DatabaseError>;
}

impl <T: Eq + hash::Hash + Serialize + DeserializeOwned>RemoveExTable<T> for ExTable<T> {
    fn remove_item(&mut self, item: &T) -> Result<(), error::DatabaseError> {
        if self.items.remove(item) {
            Ok(())
        } else {
            Err(error::DatabaseError::ItemNotFound)
        }
    }
}

#[test]
fn remove_item() {
    const TEST_DB: &str = "test/remove_item_table_test/";
    const TEST: &str = "test";

    let mut table: ExTable<String> = ExTable {
        database: ExDatabase {
            path: TEST_DB.into(),
            table_labels: Arc::new(Mutex::new(vec![])),
        }.into(),
        label: "remove_item_table".into(),
        items: HashSet::new(),
    };

    table.items.insert(TEST.into());

    table.remove_item(&TEST.into()).unwrap();

    assert_eq!(
        table.items.len(),
        0
    )
}