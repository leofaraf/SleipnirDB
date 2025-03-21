use std::{error::Error, fs::OpenOptions};

use memmap2::{MmapMut, MmapOptions};

// To check that it's database file
const MAGIC_NUMBER: u32 = 0x14841488;
const MAGIC_NUMBER_OFFSET: usize = 0;

const VERSION_OFFSET: usize = 4;
const CURRENT_VERSION: &str = "0.0.1";

const DEFAULT_COLLECTIONS_COUNT: u32 = 0;
const COLLECTIONS_COUNT_OFFSET: usize = 12;

const COLLECTIONS_OFFSET: usize = 16;

// like Table
struct Collection {
    name: [u8; 32],
    // link to first document offset
    document_offset: usize
}

// Collection entry
struct Document {
    primary_key: usize,
    // json/bson content
    content: Vec<u8>,
    next_document_offset: usize,
}


struct SleipnirDB {
    mmap: MmapMut,
    path: String,
}

impl SleipnirDB {
    fn embedded(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .read(true)
            .open(path)?;
        file.set_len(20)?;
        
        let mmap = unsafe { MmapOptions::new().map_mut(&file)? };
        
        let lenght = mmap.len();
        let mut db = SleipnirDB { 
            mmap,
            path: path.into(),
        };
        // init headers
        if lenght == 0 {
            db.write_database_header();
        } else {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&db.mmap[..4]);
            let magic_number = u32::from_le_bytes(bytes);
            if magic_number!=MAGIC_NUMBER {
                return Err("This's not database file!".into());
            }
        }
        Ok(db)
    }

    pub fn write_database_header(&mut self) {
        // MagicNumber
        self.mmap[MAGIC_NUMBER_OFFSET..MAGIC_NUMBER_OFFSET+4].copy_from_slice(&MAGIC_NUMBER.to_le_bytes());
        // Version
        let mut bytes = [0u8; 8];
        bytes[..CURRENT_VERSION.len()].copy_from_slice(CURRENT_VERSION.as_bytes());
        self.mmap[VERSION_OFFSET..VERSION_OFFSET+8].copy_from_slice(&bytes);
        // Collections count
        self.mmap[COLLECTIONS_COUNT_OFFSET..COLLECTIONS_COUNT_OFFSET+4]
            .copy_from_slice(&DEFAULT_COLLECTIONS_COUNT.to_le_bytes());
    }
}

struct Connection<'a> {
    database: &'a SleipnirDB
}

impl <'a>Connection<'a> {
    fn get_connection(database: &'a SleipnirDB) -> Self {
        Self {
            database,
        }
    }

    fn printinfo(&mut self) {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.database.mmap[..4]);
        let magic_number = u32::from_le_bytes(bytes);
        println!("MagicNumber: {}", magic_number);

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.database.mmap[VERSION_OFFSET..VERSION_OFFSET+8]);
        let version = String::from_utf8_lossy(&bytes);
        println!("Version: '{}'", version);

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.database.mmap[COLLECTIONS_COUNT_OFFSET..COLLECTIONS_COUNT_OFFSET+4]);
        let collections_count = u32::from_le_bytes(bytes);
        println!("CollectionsCount: {}", collections_count);
    }

    fn insert_collection(&mut self, name: String) {}
    fn remove_collection(&mut self, name: String) {}
    fn printcollections(&mut self) {}
    fn aggrigate(&mut self) {}
    // ETC. (find, insert...)
}

fn get_connection(db: &SleipnirDB) -> Connection {
    Connection::get_connection(&db)
}

fn main() {
    let db = SleipnirDB::embedded("store.db").unwrap();
    let mut con = get_connection(&db);
    con.printinfo();

    // output:
    // MagicNumber: 344200328
    // Version: '0.0.1'
    // CollectionsCount: 0
}