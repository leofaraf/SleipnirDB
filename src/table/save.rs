use super::ExTable;

pub trait DumpExTable {
    fn dump() -> Result<(), ()>;

    fn serialize();
    fn deserialize();
}

pub trait ExTableBytes {
    fn as_bytes(&self) -> &[u8];
}