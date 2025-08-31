use crate::{compression::compression::Compressor, net::net_error::NetError};

pub struct LzpCompressor();

impl Compressor for LzpCompressor {
    fn decompress(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<usize, NetError> {
        todo!("TODO: implement");

        Err(NetError::Generic("TODO: implement".into()))
    }
    fn compress(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<usize, NetError> {
        todo!("TODO: implement");

        Err(NetError::Generic("TODO: implement".into()))
    }
}
