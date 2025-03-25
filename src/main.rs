use std::{error::Error, fs::{File, OpenOptions}, os::windows::fs::MetadataExt};

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
    database: &'a mut SleipnirDB
}

impl <'a>Connection<'a> {
    fn get_connection(database: &'a mut SleipnirDB) -> Self {
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

        let mut offset = COLLECTIONS_OFFSET;
        for _ in 0..collections_count {
            // Read collection name
            let mut name_bytes = [0u8; 32];
            name_bytes.copy_from_slice(&self.database.mmap[offset..offset+32]);
            let name = String::from_utf8_lossy(&name_bytes).trim_end_matches('\0').to_string();

            // Read document offset
            let mut doc_offset_bytes = [0u8; 8];
            doc_offset_bytes.copy_from_slice(&self.database.mmap[offset+32..offset+40]);
            let document_offset = usize::from_le_bytes(doc_offset_bytes);

            println!(" - Collection: '{}' (Document Offset: {})", name, document_offset);

            offset += 40; // Move to the next collection
        }
    }

    fn insert_collection(&mut self, name: String) {
        if name.len() > 32 {
            println!("Collection name is too long!");
            return;
        }

        // if self.database.mmap.len()

        // Read current collections count
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.database.mmap[COLLECTIONS_COUNT_OFFSET..COLLECTIONS_COUNT_OFFSET + 4]);
        let mut collections_count = u32::from_le_bytes(bytes);

        // Calculate new collection's offset
        let new_collection_offset = COLLECTIONS_OFFSET + (collections_count as usize) * (32 + 8);

        // Write collection name (padded to 32 bytes)
        let mut name_bytes = [0u8; 32];
        name_bytes[..name.len()].copy_from_slice(name.as_bytes());
        self.database.mmap[new_collection_offset..new_collection_offset + 32].copy_from_slice(&name_bytes);

        // Write initial document offset (set to 0, meaning empty)
        let document_offset_bytes = 0usize.to_le_bytes();
        self.database.mmap[new_collection_offset + 32..new_collection_offset + 40].copy_from_slice(&document_offset_bytes);

        // Update collections count
        collections_count += 1;
        self.database.mmap[COLLECTIONS_COUNT_OFFSET..COLLECTIONS_COUNT_OFFSET + 4]
            .copy_from_slice(&collections_count.to_le_bytes());

        println!("Collection '{}' inserted at offset {}", name, new_collection_offset);
    }
    fn remove_collection(&mut self, name: String) {}
    fn printcollections(&mut self) {}
    fn aggrigate(&mut self) {}
    // ETC. (find, insert...)
}

fn get_connection(db: &mut SleipnirDB) -> Connection {
    Connection::get_connection(db)
}

fn main() {
    let mut db = SleipnirDB::embedded("store.db").unwrap();
    let mut con = get_connection(&mut db);

    // con.insert_collection("users".to_string());
    // con.insert_collection("orders".to_string());
    // con.insert_collection("products".to_string());

    con.printinfo();

    // output
    // Collection 'users' inserted at offset 16
    // Collection 'orders' inserted at offset 56
    // Collection 'products' inserted at offset 96
    // MagicNumber: 344200328
    // Version: '0.0.1'
    // CollectionsCount: 3
    //  - Collection: 'users' (Document Offset: 0)
    //  - Collection: 'orders' (Document Offset: 0)
    //  - Collection: 'products' (Document Offset: 0)
}