use crate::{
    compression::{lzp_compression::LzpCompressor, zlib_compression::ZlibCompressor},
    net::net_error::NetError,
};

pub enum CompressionType {
    None,
    Lzp(LzpCompressor),
    Zlib(ZlibCompressor),
}

pub trait Compressor {
    fn decompress(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<usize, NetError>;
    fn compress(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<usize, NetError>;
}

impl Compressor for CompressionType {
    fn decompress(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<usize, NetError> {
        match self {
            CompressionType::None => {
                output.extend_from_slice(input);
                Ok(input.len())
            }
            CompressionType::Lzp(compression) => compression.decompress(input, output),
            CompressionType::Zlib(compression) => compression.decompress(input, output),
        }
    }
    fn compress(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<usize, NetError> {
        match self {
            CompressionType::None => {
                output.extend_from_slice(input);
                Ok(input.len())
            }
            CompressionType::Lzp(compression) => compression.compress(input, output),
            CompressionType::Zlib(compression) => compression.compress(input, output),
        }
    }
}
