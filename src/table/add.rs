use std::{collections::{hash_set, HashSet}, error::Error, hash, ops::DerefMut, sync::{Arc, Mutex}};

use serde::{de::DeserializeOwned, Serialize};

use crate::{database::ExDatabase, error, table};

use super::ExTable;

pub trait AddExTable<T>
where T: Eq + hash::Hash + Serialize + DeserializeOwned {
    fn add_item(&mut self, item: T) -> Result<(), error::DatabaseError>;
}

impl <T: Eq + hash::Hash + Serialize + DeserializeOwned>AddExTable<T> for ExTable<T> {
    fn add_item(&mut self, item: T) -> Result<(), error::DatabaseError> {
        self.items.insert(item);
        Ok(())
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