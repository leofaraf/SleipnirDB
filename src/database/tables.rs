use std::{borrow::Cow, collections::HashSet, error::Error, ffi::OsStr, fs::{create_dir_all, remove_dir, remove_file, File}, hash::{self, Hash}, io::Read, path::PathBuf, sync::{Arc, Mutex}};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{database::load::LoadExDatabase, table::{save::DumpExTable, ExTable, EX_TABLE_EXTENSION}};

use super::ExDatabase;

#[derive(Debug)]
pub struct ExDatabaseEntry {
    pub database: Arc<ExDatabase>,
    pub label: String
}

pub trait ExDatabaseTablesUtils {
    fn load<T: Serialize + DeserializeOwned + Eq + hash::Hash>(&self) -> Result<ExTable<T>, Box<dyn Error>>;
    fn remove(&self);
}

impl ExDatabaseTablesUtils for ExDatabaseEntry {
    fn load<T: Serialize + DeserializeOwned + Eq + hash::Hash>(&self) -> Result<ExTable<T>, Box<dyn Error>> {
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
                    items: Arc::new(Mutex::new(items))
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