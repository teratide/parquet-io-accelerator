use super::Compression;
use std::io::Read;
use zstd::stream::read::{Decoder, Encoder};

pub struct Zstd<const COMPRESSION_LEVEL: i32>;

impl<const COMPRESSION_LEVEL: i32> Compression for Zstd<COMPRESSION_LEVEL> {
    const NAME: &'static str = "zstd";

    fn compress(input: &[u8], output: &mut [u8]) -> usize {
        Encoder::with_buffer(input, COMPRESSION_LEVEL)
            .unwrap()
            .read(output)
            .unwrap()
    }

    fn decompress(input: &[u8], output: &mut [u8]) -> usize {
        Decoder::new(input).unwrap().read(output).unwrap()
    }
}
