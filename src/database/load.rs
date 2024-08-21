use std::{error::Error, fs::{create_dir_all, read_dir, remove_dir, remove_file, File}, path::PathBuf};

use super::{tables::ExDatabaseEntry, ExDatabase};

pub trait LoadExDatabase {
    fn load(path: impl Into<String>) -> Result<ExDatabase, Box<dyn Error>>;
}

impl LoadExDatabase for ExDatabase {
    fn load(path: impl Into<String>) -> Result<ExDatabase, Box<dyn Error>> {
        let path: String = path.into();
        let path: PathBuf = PathBuf::from(path);

        create_dir_all(&path)?;

        let mut tables = vec![];

        for dir_entry in read_dir(&path)? {
            let dir_entry = dir_entry?;

            if dir_entry.file_type()?.is_file() {
                tables.push(
                    ExDatabaseEntry(path.to_path_buf())
                );
            }
        }

        Ok(ExDatabase { 
            path,
            items: tables
        })
    }
}

#[test]
pub fn test_load() {
    const TEST_DB: &str = "test/test_load";

    let database: ExDatabase = ExDatabase::load(TEST_DB).unwrap();

    remove_dir(TEST_DB).unwrap();

    assert_eq!(database.path, PathBuf::from(TEST_DB))
}