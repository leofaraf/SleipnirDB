use std::{error::Error, fs::{File, OpenOptions}, os::windows::fs::MetadataExt};

use header::{printinfo, write_database_header};
use memmap2::{MmapMut, MmapOptions};

pub mod header;

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
        file.set_len(1000)?;
        
        let mmap = unsafe { MmapOptions::new().map_mut(&file)? };
        
        let mut db = SleipnirDB { 
            mmap,
            file,
            path: path.into(),
        };
        // init headers
        if lenght == 0 {
            write_database_header(&mut db);
        } else {
            get_magic_number(&mut db)?;
        }
        Ok(db)
    }

    fn ensure_capacity(&mut self, required_size: usize) -> Result<(), Box<dyn Error>> {
        let current_size = self.file.metadata()?.file_size() as usize;
        if required_size > current_size {
            let new_size = (required_size * 2).max(current_size + 1024); // Expand by 2x or at least 1KB
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

    printinfo(&mut con.database.mmap);
}