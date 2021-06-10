use super::Compression;
use lz4_flex::block;

pub struct LZ4;

impl Compression for LZ4 {
    const NAME: &'static str = "LZ4";

    fn compress(input: &[u8], output: &mut [u8]) -> usize {
        block::compress_into(input, output, 0).unwrap()
    }

    fn decompress(input: &[u8], output: &mut [u8]) -> usize {
        block::decompress_into(input, output, 0).unwrap()
    }
}
