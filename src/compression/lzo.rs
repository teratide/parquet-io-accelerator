use super::Compression;
use rust_lzo::LZOContext;

pub struct LZO;

impl Compression for LZO {
    const NAME: &'static str = "LZO";

    fn compress(input: &[u8], output: &mut [u8]) -> usize {
        LZOContext::new().compress_to_slice(input, output).0.len()
    }

    fn decompress(input: &[u8], output: &mut [u8]) -> usize {
        LZOContext::decompress_to_slice(input, output).0.len()
    }
}
