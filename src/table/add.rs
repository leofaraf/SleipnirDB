use std::{collections::{hash_set, HashSet}, error::Error, hash, ops::DerefMut, sync::{Arc, Mutex}};

use serde::{de::DeserializeOwned, Serialize};

use crate::{database::ExDatabase, table};

use super::ExTable;

pub trait AddExTable<T>
where T: Eq + hash::Hash + Serialize + DeserializeOwned {
    fn add_item(&mut self, item: T) -> bool;
}

impl <T: Eq + hash::Hash + Serialize + DeserializeOwned>AddExTable<T> for ExTable<T> {
    fn add_item(&mut self, item: T) -> bool {
        self.items.insert(item)
    }
}

#[test]
fn add_item() {
    const TEST_DB: &str = "test/add_table_test/";
    const TEST: &str = "test";

    let mut table: ExTable<String> = ExTable {
        database: ExDatabase {
            path: TEST_DB.into(),
            table_labels: Arc::new(Mutex::new(vec![])),
        }.into(),
        label: "add_table".into(),
        items: HashSet::new(),
    };

    table.add_item(TEST.into());
    
    let item = table.items.get(TEST).unwrap();
    println!("item: {}", item);

    assert_eq!(&TEST, item)
}