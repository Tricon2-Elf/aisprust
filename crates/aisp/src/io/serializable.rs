use std::io::Result;

use crate::io::reader::EndianReader;
use crate::io::writer::EndianWriter;

pub trait Serializable {
    fn deserialize<R: EndianReader>(&mut self, reader: &mut R) -> Result<()>;

    fn serialize<R: EndianWriter>(&self, writer: &mut R) -> Result<()>;
}
