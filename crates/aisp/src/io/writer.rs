use std::io::{Error, ErrorKind, Result, Seek, SeekFrom, Write};

use crate::io::endianness::{EndianAware, Endianness};

pub struct AiWriter<R> {
    base: R,
    // pos: usize,
    endian: Endianness,
}

pub trait EndianWriter: Write + Seek + EndianAware {
    fn write_i8(&mut self, value: i8) -> Result<()> {
        let buf = [value as u8; size_of::<i8>()];
        self.write_all(&buf)?;

        Ok(())
    }
    fn write_i16(&mut self, value: i16) -> Result<()> {
        let buf = match self.endian() {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)?;

        Ok(())
    }

    fn write_i32(&mut self, value: i32) -> Result<()> {
        let buf = match self.endian() {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)?;

        Ok(())
    }
    fn write_i64(&mut self, value: i64) -> Result<()> {
        let buf = match self.endian() {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)?;

        Ok(())
    }

    fn write_u8(&mut self, value: u8) -> Result<()> {
        let buf = [value; size_of::<u8>()];
        self.write_all(&buf)?;

        Ok(())
    }
    fn write_u16(&mut self, value: u16) -> Result<()> {
        let buf = match self.endian() {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)?;

        Ok(())
    }

    fn write_u32(&mut self, value: u32) -> Result<()> {
        let buf = match self.endian() {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)?;

        Ok(())
    }
    fn write_u64(&mut self, value: u64) -> Result<()> {
        let buf = match self.endian() {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)?;

        Ok(())
    }
    // fn write_string(&mut self) -> Result<()> {
    //     let mut string = String::new();
    //
    //     loop {
    //         let byte = self.write_u8()?;
    //
    //         if byte == 0 {
    //             break;
    //         }
    //         string.push(byte as char);
    //     }
    //     return Ok(());
    // }
    //
    fn write_ai_string_unicode(&mut self, value: &String) -> Result<()> {
        if value.len() > 0xFF {
            return Err(Error::new(ErrorKind::InvalidInput, "string to long"));
        }

        self.write_u8(value.len() as u8)?;
        for e in value.encode_utf16() {
            self.write_u16(e)?;
        }

        Ok(())
    }
}
impl<T: Write + Seek + EndianAware> EndianWriter for T {}

impl<R: Write + Seek> AiWriter<R> {
    pub fn new(reader: R, endian: Endianness) -> Self {
        Self {
            base: reader,
            // pos: 0,
            endian,
        }
    }

    pub fn new_le(reader: R) -> Self {
        Self::new(reader, Endianness::LittleEndian)
    }

    pub fn new_be(reader: R) -> Self {
        Self::new(reader, Endianness::BigEndian)
    }
}

impl<R> EndianAware for AiWriter<R> {
    fn set_endian(&mut self, endian: Endianness) {
        self.endian = endian;
    }

    fn endian(&self) -> Endianness {
        self.endian
    }
}

impl<R: Write> Write for AiWriter<R> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.base.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.base.flush()
    }
}

impl<R: Seek> Seek for AiWriter<R> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.base.seek(pos)
    }
}
