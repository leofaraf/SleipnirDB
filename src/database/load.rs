use std::{error::Error, fs::{remove_file, File}, path::PathBuf};

use crate::table::save::ExTableBytes;

use super::{header::Header, ExDatabase};

pub trait LoadExDatabase<T> {
    fn load(path: impl Into<String>) -> Result<ExDatabase<T>, Box<dyn Error>>;
}

impl <T>LoadExDatabase<T> for ExDatabase<T> {
    fn load(path: impl Into<String>) -> Result<ExDatabase<T>, Box<dyn Error>> {
        let path: String = path.into();

        let mut file = match File::open(path.clone()) {
            Ok(file) => file,
            Err(_) => File::create(path.clone())?,
        };

        let header = match Header::read(&mut file) {
            Ok(header) => header,
            Err(_) => {
                let header = Header::new(0);
                header.write(&mut file)?;
                header
            },
        };

        Ok(ExDatabase { 
            path: path.into(),
            header,
            file,
            items: vec![]
        })
    }
}

#[test]
pub fn test_load() {
    const TEST_DB: &str = "test.exdb";

    use crate::database::header::MAGICAL_NUMBER;

    let database: ExDatabase<String> = ExDatabase::load(TEST_DB).unwrap();

    remove_file(TEST_DB).unwrap();

    assert_eq!(database.header.magic_number, MAGICAL_NUMBER)
}