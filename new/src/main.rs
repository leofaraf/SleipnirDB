#[cfg(target_pointer_width = "32")]
compile_error!("This program requires a 64-bit system because file offsets use u64.");

use std::{error::Error, fs::{File, OpenOptions}, os::windows::fs::MetadataExt};

use documents::{find::find_last_offset, Document, Documents, RawDocument};
use header::{Header, HeaderError};
use memmap2::{MmapMut, MmapOptions};

pub mod header;
pub mod documents;

pub struct SleipnirDB {
    mmap: MmapMut,
    file: File,
    path: String,
}

impl SleipnirDB {
    fn embedded(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        let lenght = file.metadata().unwrap().file_size();

        let mmap = unsafe { MmapOptions::new().map_mut(&file)? };

        let mut db = SleipnirDB { 
            mmap,
            file,
            path: path.into(),
        };
        
        if lenght == 0 {
            match Header::create(&mut db) {
                Ok(header) => {
                    println!("Header created: {:?}", header)
                },
                Err(HeaderError::DatabaseError(err)) => println!("DBerr: {}", err),
                Err(err) => print!("Header parsing error: {:?}", err)
            };
        } else {
            match Header::parse(&mut db) {
                Ok(header) => {
                    println!("Header parsed: {:?}", header)
                },
                Err(HeaderError::DatabaseError(err)) => println!("DBerr: {}", err),
                Err(err) => print!("Header parsing error: {:?}", err)
            };
        }

        Ok(db)
    }

    fn ensure_capacity(&mut self, required_size: usize) -> Result<(), Box<dyn Error>> {
        let current_size = self.mmap.len();
        if required_size > current_size {
            let new_size = required_size; // Expand by 2x or at least 1KB
            self.file.set_len(new_size as u64)?; // Resize file

            // Remap memory
            self.mmap = unsafe { MmapOptions::new().map_mut(&self.file)? };
            println!("Database resized to {} bytes", new_size);
        }
        Ok(())
    }
}

struct Connection<'a> {
    database: &'a mut SleipnirDB
}

impl <'a>Connection<'a> {
    fn get_connection(database: &'a mut SleipnirDB) -> Self {
        Self {
            database,
        }
    }

    fn remove_collection(&mut self, name: String) {}
    fn printheader(&mut self) {
    }
    fn aggrigate(&mut self) {}
    // ETC. (find, insert...)
}

fn get_connection(db: &mut SleipnirDB) -> Connection {
    Connection::get_connection(db)
}

fn main() {
    let mut db = SleipnirDB::embedded("store.db").unwrap();
    let con = get_connection(&mut db);

    let content = "content".as_bytes().to_vec();
    for i in 0..100 {
        Documents::insert_document(
            &mut db, 
            Document {
                primary_key: i,
                content: content.clone(),
            },
            100
        ).unwrap();
    }

    let docs = Documents::read_all_documents(&mut db).unwrap();
    println!("docs: {:?}", docs);

    let last = find_last_offset(&mut db, 100);
    println!("last: {}", last);
}