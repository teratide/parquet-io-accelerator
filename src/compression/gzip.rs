use super::Compression;
use flate2::bufread::{GzDecoder, GzEncoder};
use std::io::Read;

pub struct GZIP<const COMPRESSION_LEVEL: u32>;

impl<const COMPRESSION_LEVEL: u32> Compression for GZIP<COMPRESSION_LEVEL> {
    const NAME: &'static str = "GZIP";

    fn compress(input: &[u8], output: &mut [u8]) -> usize {
        GzEncoder::new(input, flate2::Compression::new(COMPRESSION_LEVEL))
            .read(output)
            .unwrap()
    }

    fn decompress(input: &[u8], output: &mut [u8]) -> usize {
        GzDecoder::new(input).read(output).unwrap()
    }
}
