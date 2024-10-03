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
        self.items.contains(&item)
    }
}

#[test]
fn contains_item() {
    const TEST_DB: &str = "test/contains_table_test/";
    const TEST: &str = "test";

    let mut table: ExTable<String> = ExTable {
        database: ExDatabase {
            path: TEST_DB.into(),
            table_labels: Arc::new(Mutex::new(vec![])),
        }.into(),
        label: "contains_table".into(),
        items: HashSet::new(),
    };

    table.items.insert(TEST.into());

    assert_eq!(table.contains_item(TEST.into()), true)
}