use std::fs::File;

use header::Header;

use crate::table::save::ExTableBytes;

pub mod header;
pub mod dump;
pub mod load;
pub mod tables;

pub struct ExDatabase<T> {
    path: String,
    header: Header,
    file: File,
    items: Vec<T>
}