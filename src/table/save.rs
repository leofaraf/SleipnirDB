use std::{collections::HashSet, error::Error, fs::File, hash, io::Write, path::PathBuf, sync::{Arc, Mutex}};

use serde::{de::DeserializeOwned, Serialize};

use crate::database::ExDatabase;

use super::{ExTable, EX_TABLE_EXTENSION};

pub trait DumpExTable<T>
where T: Eq + hash::Hash + Serialize + DeserializeOwned {
    fn dump(&self) -> Result<(), Box<dyn Error>>;

    fn serialize_items(&self) -> Result<Vec<u8>, Box<dyn Error>> ;
    fn deserialize_items(data: Vec<u8>, label: String, database: Arc<ExDatabase>) -> Result<ExTable<T>, Box<dyn Error>>;
}

impl <T: Eq + hash::Hash + Serialize + DeserializeOwned>DumpExTable<T> for ExTable<T> {
    fn dump(&self) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(self.database.path.join(self.label.clone() + EX_TABLE_EXTENSION))?;
        file.write_all(&self.serialize_items()?)?;
        file.flush()?;

        Ok(())
    }

    fn serialize_items(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(rmp_serde::to_vec(&self.items)?)
    }

    fn deserialize_items(data: Vec<u8>, label: String, database: Arc<ExDatabase>) -> Result<ExTable<T>, Box<dyn Error>> {
        let items = rmp_serde::from_slice::<HashSet<T>>(&data)?;
        Ok(ExTable { label, database, items })
    }
}