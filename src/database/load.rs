use std::{error::Error, fs::{create_dir_all, read_dir, remove_dir, remove_file, File}, path::PathBuf, sync::{Arc, Mutex}};

use crate::table::EX_TABLE_EXTENSION;

use super::{tables::ExDatabaseEntry, ExDatabase};

pub trait LoadExDatabase {
    fn load(path: impl Into<String>) -> Result<Arc<ExDatabase>, Box<dyn Error>>;
}

impl LoadExDatabase for ExDatabase {
    fn load(path: impl Into<String>) -> Result<Arc<ExDatabase>, Box<dyn Error>> {
        let path: String = path.into();
        let path: PathBuf = PathBuf::from(path);

        create_dir_all(&path)?;

        let tables = vec![];

        let mut database = ExDatabase { 
            path: path.clone(),
            table_labels: Arc::new(Mutex::new(tables))
        };

        let db_arc = Arc::new(database);

        for dir_entry in read_dir(&path)? {
            let dir_entry = dir_entry?;
            let file_name = dir_entry.file_name().to_string_lossy().to_string();

            if dir_entry.file_type()?.is_file() & file_name.ends_with(EX_TABLE_EXTENSION) {
                db_arc.add_table_label(
                    ExDatabaseEntry {
                        database: db_arc.clone(),
                        label: file_name[0..file_name.len()-EX_TABLE_EXTENSION.len()].to_string(),
                    }
                );
            }
        }

        Ok(db_arc)
    }
}

#[test]
pub fn test_load() {
    const TEST_DB: &str = "test/test_load";

    let database = ExDatabase::load(TEST_DB).unwrap();

    remove_dir(TEST_DB).unwrap();

    assert_eq!(database.path, PathBuf::from(TEST_DB))
}