use std::io::{Read, Result, Seek, SeekFrom};

use crate::{
    io::endianness::{EndianAware, Endianness},
    util::scramble::{Scramble, ScrambleImpl},
};

pub struct AiScrambleReader<R> {
    base: R,
    start_pos: usize,
    scramble: ScrambleImpl,
}

impl<R: Read + Seek> AiScrambleReader<R> {
    pub fn new(mut base: R, scramble: ScrambleImpl) -> Self {
        // let start_pos = base.seek(SeekPos::Current(0));
        let start_pos = base.stream_position().expect("Faield to get position") as usize;

        Self {
            base,
            start_pos,
            scramble,
        }
    }
}

impl<R: Read + Seek> Read for AiScrambleReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let start_offset = self.base.stream_position()? - (self.start_pos as u64);
        let read_count = self.base.read(buf)?;

        self.scramble.unscramble_base(buf, start_offset as usize);

        // for i in 0..read_count {
        //     let idx = (start_offset + (i as u64)) % (self.scramble.len() as u64);
        //     buf[i] = buf[i].wrapping_sub(self.scramble[idx as usize]);
        // }

        Ok(read_count)
    }
}

impl<R: Seek> Seek for AiScrambleReader<R> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.base.seek(pos)
    }
}

impl<R: EndianAware> EndianAware for AiScrambleReader<R> {
    fn set_endian(&mut self, endian: Endianness) {
        self.base.set_endian(endian);
    }
    fn endian(&self) -> Endianness {
        self.base.endian()
    }
}
