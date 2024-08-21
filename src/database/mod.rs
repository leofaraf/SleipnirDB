use std::{path::PathBuf, sync::{Arc, Mutex}};

use tables::ExDatabaseEntry;

// pub mod header;
pub mod dump;
pub mod load;
pub mod tables;

pub struct ExDatabase {
    path: PathBuf,
    items: Vec<ExDatabaseEntry>
}