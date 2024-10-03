use std::{collections::HashSet, hash, sync::{Arc, Mutex}};

use serde::{de::DeserializeOwned, Serialize};

use crate::{database::ExDatabase, error};

use super::{add::AddExTable, remove::RemoveExTable, ExTable};

pub trait UpdateExTable<T>
where T: Eq + hash::Hash + Serialize + DeserializeOwned {
    fn update_item(&mut self, item: &T, new: T) -> Result<(), error::DatabaseError>;
}

impl <T: Eq + hash::Hash + Serialize + DeserializeOwned>UpdateExTable<T> for ExTable<T> {
    fn update_item(&mut self, item: &T, new: T) -> Result<(), error::DatabaseError> {
        self.remove_item(item)?;
        self.add_item(new)?;

        Ok(())
    }
}

#[test]
fn update_item() {
    const TEST_DB: &str = "test/update_item_table_test/";

    let mut table: ExTable<i32> = ExTable {
        database: ExDatabase {
            path: TEST_DB.into(),
            table_labels: Arc::new(Mutex::new(vec![])),
        }.into(),
        label: "update_item_table".into(),
        items: HashSet::new(),
    };

    table.add_item(10).unwrap();
    table.update_item(&10, 11).unwrap();

    assert_eq!(
        table.items.contains(&11),
        true
    )
}