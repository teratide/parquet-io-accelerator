use super::Compression;
use brotli::{CompressorReader, Decompressor};
use std::io::Read;

pub struct Brotli<
    const BUFFER_SIZE: usize,
    const COMPRESSION_QUALITY: u32,
    const LG_WINDOW_SIZE: u32,
>;

impl<const BUFFER_SIZE: usize, const COMPRESSION_QUALITY: u32, const LG_WINDOW_SIZE: u32>
    Compression for Brotli<BUFFER_SIZE, COMPRESSION_QUALITY, LG_WINDOW_SIZE>
{
    const NAME: &'static str = "Brotli";

    fn compress(input: &[u8], output: &mut [u8]) -> usize {
        CompressorReader::new(input, BUFFER_SIZE, COMPRESSION_QUALITY, LG_WINDOW_SIZE)
            .read(output)
            .unwrap()
    }

    fn decompress(input: &[u8], output: &mut [u8]) -> usize {
        Decompressor::new(input, BUFFER_SIZE).read(output).unwrap()
    }
}
