use std::{borrow::Cow, collections::HashSet, error::Error, ffi::OsStr, fs::{create_dir_all, remove_dir, remove_file, File}, hash::{self, Hash}, io::Read, path::PathBuf};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::table::{ExTable, save::DumpExTable};

pub struct ExDatabaseEntry(pub PathBuf);

pub trait ExDatabaseTablesUtils {
    fn load<T: Serialize + DeserializeOwned + Eq + hash::Hash>(&self) -> Result<ExTable<T>, Box<dyn Error>>;
    fn remove(&self);
}

impl ExDatabaseTablesUtils for ExDatabaseEntry {
    fn load<T: Serialize + DeserializeOwned + Eq + hash::Hash>(&self) -> Result<ExTable<T>, Box<dyn Error>> {
        let label = match self.0.file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            None => "".to_string(),
        };

        let save_path = self.0.clone();
        
        match self.0.exists() {
            true => {
                let mut file = File::open(&self.0)?;

                let mut data: Vec<u8> = vec![];
                file.read_to_end(&mut data)?;
                ExTable::deserialize_items(data, label, save_path)
            },
            false => {
                let items: HashSet<T> = HashSet::new();

                Ok(ExTable {
                    label,
                    save_path,
                    items
                })
            },
        }
    }

    fn remove(&self) {
        remove_file(&self.0).unwrap();
    }
}

#[test]
fn load_table_test() {
    let dir = PathBuf::from("test/load_table_test/");
    create_dir_all(&dir).unwrap();
    let entry = ExDatabaseEntry(dir.join("new.exdb"));
    let table: ExTable<String> = entry.load().unwrap();
    table.dump().unwrap();
    println!("table: {:?}", table);
    
    let table: ExTable<String> = entry.load().unwrap();
    
    remove_file(entry.0).unwrap();
    remove_dir(dir).unwrap();

    assert_eq!(table.label, "new.exdb")
}