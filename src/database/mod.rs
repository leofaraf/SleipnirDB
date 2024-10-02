use std::{path::PathBuf, sync::{Arc, Mutex}};

use serde::Serialize;
use tables::ExDatabaseEntry;

// pub mod header;
pub mod dump;
pub mod load;
pub mod tables;

#[derive(Debug)]
pub struct ExDatabase {
    pub path: PathBuf,
    pub table_labels: Arc<Mutex<Vec<ExDatabaseEntry>>>
}

impl ExDatabase {
    fn add_table_label(&self, entry: ExDatabaseEntry) {
        let mut lock = self.table_labels.lock().unwrap();
        lock.push(entry);
    }
}