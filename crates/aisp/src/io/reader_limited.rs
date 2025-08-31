use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};

use crate::io::endianness::{EndianAware, Endianness};

pub struct LimitedReader<R> {
    base: R,
    start_pos: usize,
    pos: usize,
    size: usize,
}

impl<R: Read + Seek> LimitedReader<R> {
    pub fn new(mut base: R, size: usize) -> Self {
        // let start_pos = base.seek(SeekPos::Current(0));
        let start_pos = base.stream_position().expect("failed to query start pos");

        Self {
            base,
            start_pos: start_pos as usize,
            pos: 0,
            size,
        }
    }
}

impl<R: Read + Seek> Read for LimitedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let to_read = buf.len();
        if self.pos + to_read > self.size {
            return Err(Error::new(ErrorKind::InvalidInput, "Read after end!"));
        }

        let read_count = self.base.read(buf)?;

        self.pos += read_count;

        Ok(read_count)
    }
}

impl<R: Seek> Seek for LimitedReader<R> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let abs_offset_128 = match pos {
            SeekFrom::Start(position) => position as i128,
            SeekFrom::Current(offset) => (self.pos as i128) + (offset as i128),
            SeekFrom::End(offset) => (self.size as i128) + (offset as i128),
        };

        if (abs_offset_128 >> 64) != 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Seek overflwo or underflow",
            ));
        }

        let abs_offset = abs_offset_128 as usize;

        if abs_offset > self.size {
            return Err(Error::new(ErrorKind::InvalidInput, "Seeked over end"));
        }

        let target_offset = self.base.seek(SeekFrom::Start(
            self.start_pos
                .checked_add(abs_offset)
                .expect("seek overflow") as u64,
        ))?;

        self.pos = (target_offset as usize)
            .checked_sub(self.start_pos)
            .expect("seek undefflow");

        Ok(self.pos as u64)
    }
}

impl<R: EndianAware> EndianAware for LimitedReader<R> {
    fn set_endian(&mut self, endian: Endianness) {
        self.base.set_endian(endian);
    }
    fn endian(&self) -> Endianness {
        self.base.endian()
    }
}
