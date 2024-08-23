use std::{borrow::Cow, collections::HashSet, error::Error, ffi::OsStr, fs::{create_dir_all, remove_dir, remove_file, File}, hash::{self, Hash}, io::Read, path::PathBuf, sync::Arc};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{database::load::LoadExDatabase, table::{save::DumpExTable, ExTable, EX_TABLE_EXTENSION}};

use super::ExDatabase;

#[derive(Debug)]
pub struct ExDatabaseEntry<'a, 'b> {
    pub database: Arc<ExDatabase<'a, 'b>>,
    pub label: String
}

pub trait ExDatabaseTablesUtils<'a, 'b> {
    fn load<T: Serialize + DeserializeOwned + Eq + hash::Hash>(&'b self) -> Result<ExTable<'a, 'b, T>, Box<dyn Error>>;
    fn remove(&self);
}

impl <'a, 'b>ExDatabaseTablesUtils<'a, 'b> for ExDatabaseEntry<'a, 'b> {
    fn load<T: Serialize + DeserializeOwned + Eq + hash::Hash>(&'b self) -> Result<ExTable<'a, 'b, T>, Box<dyn Error>> {
        let save_path = self.database.path.join(self.label.clone() + EX_TABLE_EXTENSION);
        
        match save_path.exists() {
            true => {
                let mut file = File::open(save_path)?;

                let mut data: Vec<u8> = vec![];
                file.read_to_end(&mut data)?;
                ExTable::deserialize_items(data, self.label.clone(), self.database.clone())
            },
            false => {
                let items: HashSet<T> = HashSet::new();

                Ok(ExTable {
                    label: self.label.clone(),
                    database: self.database.clone(),
                    items
                })
            },
        }
    }

    fn remove(&self) {
        remove_file(self.database.path.join(self.label.clone() + EX_TABLE_EXTENSION)).unwrap();
    }
}

#[test]
fn load_table_test() {
    const TEST_DB: &str = "test/load_table_test/";

    let database = ExDatabase::load(TEST_DB).unwrap();

    let entry = ExDatabaseEntry {
        database,
        label: "new".into(),
    };
    let table: ExTable<String> = entry.load().unwrap();
    table.dump().unwrap();
    println!("table: {:?}", table);
    
    let table: ExTable<String> = entry.load().unwrap();
    
    remove_file(TEST_DB.to_string() + "new" + EX_TABLE_EXTENSION).unwrap();
    remove_dir(TEST_DB).unwrap();

    assert_eq!(table.label, "new")
}