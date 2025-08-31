use std::io::{Read, Result};

use crate::io::endianness::Endianness;

pub trait ReadExt: Read {
    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0u8; size_of::<u8>()];
        self.read(&mut buf)?;

        return Ok(buf[0] as i8);
    }
    fn read_i16_endian(&mut self, endian: Endianness) -> Result<i16> {
        let mut buf = [0u8; size_of::<i16>()];
        self.read(&mut buf)?;

        let value = match endian {
            Endianness::LittleEndian => i16::from_le_bytes(buf),
            Endianness::BigEndian => i16::from_be_bytes(buf),
        };

        return Ok(value);
    }
    fn read_i32_endian(&mut self, endian: Endianness) -> Result<i32> {
        let mut buf = [0u8; size_of::<i32>()];
        self.read(&mut buf)?;

        let value = match endian {
            Endianness::LittleEndian => i32::from_le_bytes(buf),
            Endianness::BigEndian => i32::from_be_bytes(buf),
        };

        return Ok(value);
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; size_of::<u8>()];
        self.read(&mut buf)?;

        return Ok(buf[0]);
    }
    fn read_u16_endian(&mut self, endian: Endianness) -> Result<u16> {
        let mut buf = [0u8; size_of::<u16>()];
        self.read(&mut buf)?;

        let value = match endian {
            Endianness::LittleEndian => u16::from_le_bytes(buf),
            Endianness::BigEndian => u16::from_be_bytes(buf),
        };

        return Ok(value);
    }
    fn read_u32_endian(&mut self, endian: Endianness) -> Result<u32> {
        let mut buf = [0u8; size_of::<u32>()];
        self.read(&mut buf)?;

        let value = match endian {
            Endianness::LittleEndian => u32::from_le_bytes(buf),
            Endianness::BigEndian => u32::from_be_bytes(buf),
        };

        return Ok(value);
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
        return Ok(string);
    }
}

impl<R: Read> ReadExt for R {}
