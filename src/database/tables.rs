use crate::table::{information::ExTableInformation, ExTable};

use super::ExDatabase;

pub trait ExDatabaseTablesUtils {
    fn tables_information() -> Vec<ExTableInformation>;
    fn create_table<T, S>(label: S) -> Result<ExTable<T>, ()>
        where S: Into<String>;
    fn load_table<T>(table_type: T) -> Result<ExTable<T>, ()>;
    fn remove_table();
}