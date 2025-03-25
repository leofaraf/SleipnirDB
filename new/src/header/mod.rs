use memmap2::MmapMut;

use crate::SleipnirDB;

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

pub fn printinfo(mmap: &mut MmapMut) {
    let mut bytes: [u8; 4] = [0u8; 4];
    bytes.copy_from_slice(&mmap[..4]);
    let magic_number = u32::from_le_bytes(bytes);
    println!("MagicNumber: {}", magic_number);

    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&mmap[VERSION_OFFSET..VERSION_OFFSET+8]);
    let version = String::from_utf8_lossy(&bytes);
    println!("Version: '{}'", version);

    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&mmap[COLLECTIONS_COUNT_OFFSET..COLLECTIONS_COUNT_OFFSET+4]);
    let collections_count = u32::from_le_bytes(bytes);
    println!("CollectionsCount: {}", collections_count);

    let mut offset = COLLECTIONS_OFFSET;
    for _ in 0..collections_count {
        // Read collection name
        let mut name_bytes = [0u8; 32];
        name_bytes.copy_from_slice(&mmap[offset..offset+32]);
        let name = String::from_utf8_lossy(&name_bytes).trim_end_matches('\0').to_string();

        // Read document offset
        let mut doc_offset_bytes = [0u8; 8];
        doc_offset_bytes.copy_from_slice(&mmap[offset+32..offset+40]);
        let document_offset = usize::from_le_bytes(doc_offset_bytes);

        println!(" - Collection: '{}' (Document Offset: {})", name, document_offset);

        offset += 40; // Move to the next collection
    }
}

pub fn insert_collection(mmap: &mut MmapMut, name: String) {
    if name.len() > 32 {
        println!("Collection name is too long!");
        return;
    }

    // if self.database.mmap.len()

    // Read current collections count
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&mmap[COLLECTIONS_COUNT_OFFSET..COLLECTIONS_COUNT_OFFSET + 4]);
    let mut collections_count = u32::from_le_bytes(bytes);

    // Calculate new collection's offset
    let new_collection_offset = COLLECTIONS_OFFSET + (collections_count as usize) * (32 + 8);

    // Write collection name (padded to 32 bytes)
    let mut name_bytes = [0u8; 32];
    name_bytes[..name.len()].copy_from_slice(name.as_bytes());
    mmap[new_collection_offset..new_collection_offset + 32].copy_from_slice(&name_bytes);

    // Write initial document offset (set to 0, meaning empty)
    let document_offset_bytes = 0usize.to_le_bytes();
    mmap[new_collection_offset + 32..new_collection_offset + 40].copy_from_slice(&document_offset_bytes);

    // Update collections count
    collections_count += 1;
    mmap[COLLECTIONS_COUNT_OFFSET..COLLECTIONS_COUNT_OFFSET + 4]
        .copy_from_slice(&collections_count.to_le_bytes());

    println!("Collection '{}' inserted at offset {}", name, new_collection_offset);
}

struct Header {
    magic_number: MagicNumber,
    version: Version,
    collection_count: usize,
    collections: Vec<Collection>
}

impl Header {
    pub fn parse(db: &mut SleipnirDB) -> Result<Self, HeaderParsingError> {
        Self::ensure_capacity(db)?;
        
        Ok(Header {
            magic_number: MagicNumber::get(db)?,
            version: Version::get(db)?,
            collection_count: todo!(),
            collections: todo!(),
        })
    }

    fn ensure_capacity(db: &mut SleipnirDB) -> Result<(), HeaderParsingError> {
        match db.ensure_capacity(100) {
            Ok(_) => Ok(()),
            Err(err) => Err(HeaderParsingError::CapacityError(
                format!("Ensure capatiry error: {:?}", err).into()
            )),
        }
    }
}

pub fn write_database_header(db: &mut SleipnirDB) {
    // MagicNumber
    db.mmap[MAGIC_NUMBER_OFFSET..MAGIC_NUMBER_OFFSET+4].copy_from_slice(&MAGIC_NUMBER.to_le_bytes());
    // Version
    let mut bytes = [0u8; 8];
    bytes[..CURRENT_VERSION.len()].copy_from_slice(CURRENT_VERSION.as_bytes());
    db.mmap[VERSION_OFFSET..VERSION_OFFSET+8].copy_from_slice(&bytes);
    // Collections count
    db.mmap[COLLECTIONS_COUNT_OFFSET..COLLECTIONS_COUNT_OFFSET+4]
        .copy_from_slice(&DEFAULT_COLLECTIONS_COUNT.to_le_bytes());
}

struct MagicNumber(u32);
impl MagicNumber {
    fn get(db: &mut SleipnirDB) -> Result<Self, HeaderParsingError> {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&db.mmap[..4]);
        let magic_number = u32::from_le_bytes(bytes);
        if magic_number!=MAGIC_NUMBER {
            return Err(HeaderParsingError::MagicNumberParsingError(
                format!("This's not database file! ({})", magic_number).into()
            ));
        }
        Ok(MagicNumber(magic_number))
    }
}

struct Version(String);
impl Version {
    fn get(db: &mut SleipnirDB) -> Result<Self, HeaderParsingError> {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&db.mmap[VERSION_OFFSET..VERSION_OFFSET+8]);
        match String::from_utf8(bytes.to_vec()) {
            Ok(value) => Ok(Version(value)),
            Err(_) => Err(HeaderParsingError::VersionParsingError(
                "Cannot parse string version".into()
            )),
        }
    }
}

pub enum HeaderParsingError {
    MagicNumberParsingError(String),
    VersionParsingError(String),
    CollectionsParsingError(String),
    CapacityError(String)
}