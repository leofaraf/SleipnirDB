use crate::SleipnirDB;

const DOCUMENTS_OFFSET: usize = 100;
const DOCUMENT_PRIMARY_KEY_OFFSET: usize = 0;
const DOCUMENT_NEXT_DOCUMENT_OFFSET: usize = 8;
const DOCUMENT_CONTENT_LENGHT_OFFSET: usize = 16;
const DOCUMENT_CONTENT_OFFSET: usize = 24;

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
pub struct Document {
    pub primary_key: u64,
    pub next_document_offset: u64,
    pub content_lenght: u64,
    // json/bson content
    pub content: Vec<u8>,
}

impl Document {
    fn len(&self) -> usize {
        self.content.len() + 8 + 8 + 8
    }
}

pub struct Documents {
}

impl Documents {
    pub fn read_all_documents(db: &SleipnirDB) -> Result<Vec<Document>, DocumentsError> {
        let mut documents = Vec::new();
        let mut offset = DOCUMENTS_OFFSET;

        while offset < db.mmap.len() {
            if offset + DOCUMENT_CONTENT_OFFSET > db.mmap.len() {
                break;
            }

            let primary_key = u64::from_le_bytes(db.mmap[offset + DOCUMENT_PRIMARY_KEY_OFFSET
                ..offset + DOCUMENT_NEXT_DOCUMENT_OFFSET]
                .try_into()
                .unwrap());

            let next_document_offset = u64::from_le_bytes(db.mmap[offset + DOCUMENT_NEXT_DOCUMENT_OFFSET
                ..offset + DOCUMENT_CONTENT_LENGHT_OFFSET]
                .try_into()
                .unwrap());

            let content_length = u64::from_le_bytes(db.mmap[offset + DOCUMENT_CONTENT_LENGHT_OFFSET
                ..offset + DOCUMENT_CONTENT_OFFSET]
                .try_into()
                .unwrap());

            if offset + DOCUMENT_CONTENT_OFFSET + content_length as usize > db.mmap.len() {
                break;
            }

            let content = db.mmap[offset + DOCUMENT_CONTENT_OFFSET
                ..offset + DOCUMENT_CONTENT_OFFSET + content_length as usize]
                .to_vec();

            documents.push(Document {
                primary_key,
                next_document_offset,
                content_lenght: content_length,
                content,
            });

            if next_document_offset == 0 {
                break;
            }

            offset = next_document_offset as usize;
        }

        Ok(documents)
    }

    pub fn insert_document(db: &mut SleipnirDB, document: Document, offset: usize) -> Result<(), DocumentsError> {
        let document_offset = offset;
        
        Self::ensure_capacity(
            db,
            document_offset + document.len()
        )?;

        db.mmap[document_offset+DOCUMENT_PRIMARY_KEY_OFFSET
        ..document_offset+DOCUMENT_NEXT_DOCUMENT_OFFSET]
        .copy_from_slice(&document.primary_key.to_le_bytes());

        db.mmap[document_offset+DOCUMENT_NEXT_DOCUMENT_OFFSET
        ..document_offset+DOCUMENT_CONTENT_LENGHT_OFFSET]
        .copy_from_slice(&document.next_document_offset.to_le_bytes());
        
        db.mmap[document_offset+DOCUMENT_CONTENT_LENGHT_OFFSET
        ..document_offset+DOCUMENT_CONTENT_OFFSET]
        .copy_from_slice(&document.content_lenght.to_le_bytes());
        
        db.mmap[document_offset+DOCUMENT_CONTENT_OFFSET
        ..document_offset+DOCUMENT_CONTENT_OFFSET+document.content_lenght as usize]
        .copy_from_slice(&document.content);
        
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
    DatabaseError(String)
}