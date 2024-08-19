use header::Header;

pub mod header;
pub mod save;
pub mod load;
pub mod tables;

pub struct ExDatabase {
    path: String,
    header: Header
}