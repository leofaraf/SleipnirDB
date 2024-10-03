use exdb::{database::{load::LoadExDatabase, tables::{ExDatabaseEntry, ExDatabaseTablesUtils}, ExDatabase}, table::{add::AddExTable, ExTable}};

const DATABASE_PATH: &str = "test/hello/";
const DATABASE_LABEL: &str = "hello";

fn main() {
    let database = ExDatabase::load(DATABASE_PATH).unwrap();
    
    let mut hello_table: ExTable<String> = ExDatabaseEntry {
        database,
        label: DATABASE_LABEL.to_string(),
    }.load().unwrap();

    hello_table.add_item("hello".into()).unwrap();
    hello_table.add_item("world".into()).unwrap();

    println!("Items: {:?}.", hello_table.items)
}