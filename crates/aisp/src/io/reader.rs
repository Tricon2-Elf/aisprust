use std::io::{Read, Result, Seek, SeekFrom};

use crate::io::endianness::{EndianAware, Endianness};

pub struct AiReader<R> {
    base: R,
    // pos: usize,
    endian: Endianness,
}

pub trait EndianReader: Read + Seek + EndianAware {
    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0u8; size_of::<u8>()];
        self.read(&mut buf)?;

        Ok(buf[0] as i8)
    }
    fn read_i16(&mut self) -> Result<i16> {
        let mut buf = [0u8; size_of::<i16>()];
        self.read(&mut buf)?;

        let value = match self.endian() {
            Endianness::LittleEndian => i16::from_le_bytes(buf),
            Endianness::BigEndian => i16::from_be_bytes(buf),
        };

        Ok(value)
    }
    fn read_i32(&mut self) -> Result<i32> {
        let mut buf = [0u8; size_of::<i32>()];
        self.read(&mut buf)?;

        let value = match self.endian() {
            Endianness::LittleEndian => i32::from_le_bytes(buf),
            Endianness::BigEndian => i32::from_be_bytes(buf),
        };

        Ok(value)
    }

    fn read_i64(&mut self) -> Result<i64> {
        let mut buf = [0u8; size_of::<i64>()];
        self.read(&mut buf)?;

        let value = match self.endian() {
            Endianness::LittleEndian => i64::from_le_bytes(buf),
            Endianness::BigEndian => i64::from_be_bytes(buf),
        };

        Ok(value)
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; size_of::<u8>()];
        self.read(&mut buf)?;

        Ok(buf[0])
    }
    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0u8; size_of::<u16>()];
        self.read(&mut buf)?;

        let value = match self.endian() {
            Endianness::LittleEndian => u16::from_le_bytes(buf),
            Endianness::BigEndian => u16::from_be_bytes(buf),
        };

        Ok(value)
    }
    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; size_of::<u32>()];
        self.read(&mut buf)?;

        let value = match self.endian() {
            Endianness::LittleEndian => u32::from_le_bytes(buf),
            Endianness::BigEndian => u32::from_be_bytes(buf),
        };

        Ok(value)
    }

    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0u8; size_of::<u64>()];
        self.read(&mut buf)?;

        let value = match self.endian() {
            Endianness::LittleEndian => u64::from_le_bytes(buf),
            Endianness::BigEndian => u64::from_be_bytes(buf),
        };

        Ok(value)
    }

    fn read_f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read_u32()?))
    }
    fn read_f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(self.read_u64()?))
    }

    fn read_string(&mut self) -> Result<String> {
        let mut string = String::new();

        loop {
            let byte = self.read_u8()?;

            if byte == 0 {
                break;
            }
            string.push(byte as char);
        }
        Ok(string)
    }

    fn read_ai_string_unicode(&mut self) -> Result<String> {
        let str_len = self.read_u8()?;

        let mut buf = vec![0u16; str_len as usize];
        for i in 0..buf.len() {
            buf[i] = self.read_u16()?;
        }

        let string = String::from_utf16(&buf).expect("Failed to parse utf16 string");
        Ok(string)
    }
}
impl<T: Read + Seek + EndianAware> EndianReader for T {}

impl<R: Read + Seek> AiReader<R> {
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

impl<R> EndianAware for AiReader<R> {
    fn set_endian(&mut self, endian: Endianness) {
        self.endian = endian;
    }

    fn endian(&self) -> Endianness {
        self.endian
    }
}

impl<R: Read> Read for AiReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.base.read(buf)
    }
}
impl<R: Seek> Seek for AiReader<R> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.base.seek(pos)
    }
}
