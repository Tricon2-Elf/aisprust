use std::io::{Result, Seek, SeekFrom, Write};

use crate::{
    io::endianness::{EndianAware, Endianness},
    util::scramble::Scramble,
};

pub struct AiScrambleWriter<R> {
    base: R,
    // start_pos: usize,
    scramble: Box<dyn Scramble>,
}

impl<R: Write + Seek> AiScrambleWriter<R> {
    pub fn new(base: R, scramble: Box<dyn Scramble>) -> Self {
        // let start_pos = base.seek(SeekPos::Current(0));
        // let start_pos = base.stream_position().expect("Faield to get position") as usize;

        Self {
            base,
            // start_pos: start_pos,
            scramble,
        }
    }
}

impl<R: Write + Seek> Write for AiScrambleWriter<R> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // let start_offset = self.base.stream_position()? - (self.start_pos as u64);

        let mut new_buf = buf.to_vec();

        self.scramble.scramble(&mut new_buf);
        // for i in 0..new_buf.len() {
        //     let idx = (start_offset + (i as u64)) % (self.scramble.len() as u64);
        //     new_buf[i] = new_buf[i].wrapping_add(self.scramble[idx as usize]);
        // }

        self.base.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.base.flush()
    }
}

impl<R: Seek> Seek for AiScrambleWriter<R> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.base.seek(pos)
    }
}

impl<R: EndianAware> EndianAware for AiScrambleWriter<R> {
    fn set_endian(&mut self, endian: Endianness) {
        self.base.set_endian(endian);
    }
    fn endian(&self) -> Endianness {
        self.base.endian()
    }
}
