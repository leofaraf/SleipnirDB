#[derive(Debug)]
pub enum DatabaseError {
    IOError(std::io::Error),
    ItemNotFound
}

impl From<std::io::Error> for DatabaseError {
    fn from(e: std::io::Error) -> Self {
        DatabaseError::IOError(e)
    }
}