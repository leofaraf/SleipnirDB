use std::{borrow::Borrow, collections::HashSet, hash, ops::DerefMut, sync::{Arc, Mutex}};

use serde::{de::DeserializeOwned, Serialize};

use crate::{database::ExDatabase, error, table::contains::ContainsExTable};

use super::ExTable;

pub trait QueryExTable<T>
where T: Eq + hash::Hash + Serialize + DeserializeOwned {
    fn query_item<Q: PartialEq, V: Fn(&T) -> &Q>(
        &self,
        value: V,
        query: Q,
    ) -> Result<&T, error::DatabaseError>;
}

impl <T: Eq + hash::Hash + Serialize + DeserializeOwned>QueryExTable<T> for ExTable<T> {
    fn query_item<Q: PartialEq, V: Fn(&T) -> &Q>(
        &self,
        value: V,
        query: Q,
    ) -> Result<&T, error::DatabaseError> {
        for item in self.items.iter() {
            if value(item) == &query {
                return Ok(item);
            }
        }
    
        Err(error::DatabaseError::ItemNotFound)
    }
}

#[test]
fn query_item() {
    const TEST_DB: &str = "test/query_item_table_test/";
    const TEST: &str = "test";

    let mut table: ExTable<String> = ExTable {
        database: ExDatabase {
            path: TEST_DB.into(),
            table_labels: Arc::new(Mutex::new(vec![])),
        }.into(),
        label: "query_item_table".into(),
        items: HashSet::new(),
    };

    table.items.insert(TEST.into());

    assert_eq!(
        table.query_item(|i| i, TEST.into()).unwrap(),
        &TEST.to_string()
    )
}