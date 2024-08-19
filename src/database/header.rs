use std::io::{Read, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub const MAGICAL_NUMBER: u32 = 0xECEDEADB;

#[derive(Debug)]
pub struct Header {
    pub magic_number: u32,
    pub version: u32,
    pub num_tables: u32,
    pub offset_table: Vec<u64>,
    // Add other metadata fields as needed
}

impl Header {
    pub fn new(num_tables: usize) -> Self {
        Header {
            magic_number: MAGICAL_NUMBER, // Replace with your magic number
            version: 1,
            num_tables: num_tables as u32,
            offset_table: Vec::with_capacity(num_tables),
            // Initialize other metadata fields
        }
    }

    pub fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u32::<LittleEndian>(self.magic_number)?;
        writer.write_u32::<LittleEndian>(self.version)?;
        writer.write_u32::<LittleEndian>(self.num_tables)?;
        for offset in &self.offset_table {
            writer.write_u64::<LittleEndian>(*offset)?;
        }
        
        Ok(())
    }

    pub fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let magic_number = reader.read_u32::<LittleEndian>()?;
        let version = reader.read_u32::<LittleEndian>()?;
        let num_tables = reader.read_u32::<LittleEndian>()?;

        let mut offset_table = Vec::with_capacity(num_tables as usize);
        for _ in 0..num_tables {
            offset_table.push(reader.read_u64::<LittleEndian>()?);
        }

        Ok(Header {
            magic_number,
            version,
            num_tables,
            offset_table,
        })
    }
}