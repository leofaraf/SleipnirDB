use find::find_last_offset;

use crate::SleipnirDB;

pub mod find;

pub const DOCUMENTS_OFFSET: usize = 100;
const DOCUMENT_PRIMARY_KEY_OFFSET: usize = 0;
const DOCUMENT_NEXT_DOCUMENT_OFFSET: usize = 8;
const DOCUMENT_CONTENT_LENGHT_OFFSET: usize = 16;
pub const DOCUMENT_CONTENT_OFFSET: usize = 24;

// like Table
// Stored as document in master table
struct Collection {
    name: [u8; 32],
    // link to first document offset
    next_document_offset: usize,
    next_collection_offset: usize
}

// Collection entry
#[derive(Debug)]
pub struct RawDocument {
    pub primary_key: u64,
    pub next_document: u64,
    pub content_lenght: u64,
    // json/bson content
    pub content: Vec<u8>,
}

impl RawDocument {
    fn len(&self) -> usize {
        self.content.len() + 8 + 8 + 8
    }

    pub fn parse(&self) {
        
    }

    pub fn parse_primary_key(db: &mut SleipnirDB, offset: usize) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(
            &db.mmap[offset+DOCUMENT_PRIMARY_KEY_OFFSET
            ..offset+DOCUMENT_NEXT_DOCUMENT_OFFSET]
        );
        u64::from_le_bytes(bytes)
    }

    pub fn parse_next_document(db: &mut SleipnirDB, offset: usize) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(
            &db.mmap[offset+DOCUMENT_NEXT_DOCUMENT_OFFSET
            ..offset+DOCUMENT_CONTENT_LENGHT_OFFSET]
        );
        u64::from_le_bytes(bytes)
    }

    pub fn parse_content_length(db: &mut SleipnirDB, offset: usize) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(
            &db.mmap[offset+DOCUMENT_CONTENT_LENGHT_OFFSET
            ..offset+DOCUMENT_CONTENT_OFFSET]
        );
        u64::from_le_bytes(bytes)
    }

    pub fn parse_content(db: &mut SleipnirDB, offset: usize, content_length: usize) -> Vec<u8> {
        db.mmap[offset + DOCUMENT_CONTENT_OFFSET
                ..offset + DOCUMENT_CONTENT_OFFSET + content_length]
                .to_vec()
    }

    pub fn write_next_document(db: &mut SleipnirDB, offset: usize, next_offset: usize) {
        db.mmap[offset+DOCUMENT_NEXT_DOCUMENT_OFFSET
        ..offset+DOCUMENT_CONTENT_LENGHT_OFFSET]
        .copy_from_slice(&next_offset.to_le_bytes());
    }
}

pub struct Document {
    pub primary_key: u64,
    pub content: Vec<u8>,
}

pub struct Documents {
}

impl Documents {
    pub fn read_all_documents(db: &mut SleipnirDB) -> Result<Vec<RawDocument>, DocumentsError> {
        let mut documents = Vec::new();
        let mut offset = DOCUMENTS_OFFSET;

        while offset < db.mmap.len() {
            if offset + DOCUMENT_CONTENT_OFFSET > db.mmap.len() {
                break;
            }

            let primary_key = RawDocument::parse_primary_key(db, offset);
            let next_document = RawDocument::parse_next_document(db, offset);
            let content_length = RawDocument::parse_content_length(db, offset);

            if offset + DOCUMENT_CONTENT_OFFSET + content_length as usize > db.mmap.len() {
                break;
            }

            let content = RawDocument::parse_content(db, offset, content_length as usize);

            documents.push(RawDocument {
                primary_key,
                next_document,
                content_lenght: content_length,
                content,
            });

            if next_document == 0 {
                break;
            }

            offset = next_document as usize;
        }

        Ok(documents)
    }

    pub fn insert_raw_document(db: &mut SleipnirDB, document: RawDocument, offset: usize)
    -> Result<(), DocumentsError> {
        Self::ensure_capacity(
            db,
            offset + document.len()
        )?;

        db.mmap[offset+DOCUMENT_PRIMARY_KEY_OFFSET
        ..offset+DOCUMENT_NEXT_DOCUMENT_OFFSET]
        .copy_from_slice(&document.primary_key.to_le_bytes());

        RawDocument::write_next_document(db, offset, document.next_document as usize);
        
        db.mmap[offset+DOCUMENT_CONTENT_LENGHT_OFFSET
        ..offset+DOCUMENT_CONTENT_OFFSET]
        .copy_from_slice(&document.content_lenght.to_le_bytes());
        
        db.mmap[offset+DOCUMENT_CONTENT_OFFSET
        ..offset+DOCUMENT_CONTENT_OFFSET+document.content_lenght as usize]
        .copy_from_slice(&document.content);
        
        Ok(())
    }

    pub fn insert_document(db: &mut SleipnirDB, document: Document, table_offset: usize)
    -> Result<(), DocumentsError> {
        let next_offset = db.mmap.len();
        let last_offset = find_last_offset(db, table_offset);

        RawDocument::write_next_document(db, last_offset, next_offset);
        Self::insert_raw_document(db, RawDocument {
            primary_key: document.primary_key,
            next_document: 0,
            content_lenght: document.content.len() as u64,
            content: document.content,
        }, next_offset)?;
        
        Ok(())
    }

    fn ensure_capacity(db: &mut SleipnirDB, size: usize) -> Result<(), DocumentsError> {
        match db.ensure_capacity(size) {
            Ok(_) => Ok(()),
            Err(err) => Err(DocumentsError::DatabaseError(
                format!("Ensure capatiry error: {:?}", err).into()
            )),
        }
    }
}

#[derive(Debug)]
pub enum DocumentsError {
    PrimaryKeyError(String),
    DatabaseError(String)
}