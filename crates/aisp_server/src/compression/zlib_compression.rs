use std::io::{Read, Write};

use flate2::read::ZlibDecoder;

use crate::{compression::compression::Compressor, net::net_error::NetError};

pub struct ZlibCompressor();

impl Compressor for ZlibCompressor {
    fn decompress(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<usize, NetError> {
        const HEADER_SIZE: usize = 5;

        if input.len() < HEADER_SIZE {
            return Err(NetError::NeedData);
        }

        let chunk_is_compressed = input[0] != 0;
        let chunk_length =
            u32::from_le_bytes(input[1..5].try_into().expect("failed to get chunk len")) as usize;

        if input.len() < chunk_length + 5 {
            return Err(NetError::NeedData);
        }

        let chunk_data = &input[HEADER_SIZE..HEADER_SIZE + chunk_length];

        if chunk_is_compressed {
            let mut decoder = ZlibDecoder::new(chunk_data);
            decoder
                .read_to_end(output)
                .map_err(|e| NetError::DecompressionError(e.to_string()))?;
        } else {
            output.extend_from_slice(chunk_data);
        }

        Ok(HEADER_SIZE + chunk_length)
    }

    fn compress(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<usize, NetError> {
        const HEADER_SIZE: usize = 5;

        let mut encoder =
            flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(input).expect("Failed to write to zlib");
        let compressed = match encoder.finish() {
            Ok(data) => data,
            Err(err) => return Err(NetError::CompressionError(err.to_string())),
        };

        if compressed.len() <= input.len() {
            output.push(0); // is_compressed 
            output.extend_from_slice(&(input.len() as u32).to_le_bytes());
            output.extend_from_slice(input);

            Ok(HEADER_SIZE + input.len())
        } else {
            output.push(1); // is_compressed
            output.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
            output.extend_from_slice(&compressed);

            Ok(HEADER_SIZE + compressed.len())
        }
    }
}
