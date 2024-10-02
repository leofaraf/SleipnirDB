use std::{collections::HashSet, hash, sync::{Arc, Mutex}};

use serde::{de::DeserializeOwned, Serialize};

use crate::database::ExDatabase;

use super::ExTable;

pub trait ContainsExTable<T>
where T: Eq + hash::Hash + Serialize + DeserializeOwned {
    fn contains_item(&self, item: T) -> bool;
}

impl <T: Eq + hash::Hash + Serialize + DeserializeOwned>ContainsExTable<T> for ExTable<T> {
    fn contains_item(&self, item: T) -> bool {
        let lock = self.items.lock().unwrap();
        lock.contains(&item)
    }
}

#[test]
fn contains_item() {
    const TEST_DB: &str = "test/contains_table_test/";
    const TEST: &str = "test";

    let table: ExTable<String> = ExTable {
        database: ExDatabase {
            path: TEST_DB.into(),
            table_labels: Arc::new(Mutex::new(vec![])),
        }.into(),
        label: "add_table".into(),
        items: Arc::new(Mutex::new(HashSet::new())),
    };

    let mut lock = table.items.lock().unwrap();
    lock.insert(TEST.into());

    // Drop lock to use another lock in `contains_item` below.
    drop(lock);

    assert_eq!(table.contains_item(TEST.into()), true)
}