use crate::SleipnirDB;

use super::{RawDocument, DOCUMENT_CONTENT_OFFSET};

pub fn find_exact() {
    
}

pub fn find_last_offset(db: &mut SleipnirDB, first_offset: usize) -> usize {
    let mut offset = first_offset;

    while offset < db.mmap.len() {
        if offset + DOCUMENT_CONTENT_OFFSET > db.mmap.len() {
            break;
        }

        let next_document = RawDocument::parse_next_document(db, offset);
        let content_length = RawDocument::parse_content_length(db, offset);

        if offset + DOCUMENT_CONTENT_OFFSET + content_length as usize > db.mmap.len() {
            break;
        }

        if next_document == 0 {
            break;
        }

        offset = next_document as usize;
    }

    offset
}