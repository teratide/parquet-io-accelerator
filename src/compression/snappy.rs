use super::Compression;
use snap::raw::{Decoder, Encoder};

pub struct Snappy;

impl Compression for Snappy {
    const NAME: &'static str = "snappy";

    fn compress(input: &[u8], output: &mut [u8]) -> usize {
        Encoder::new().compress(input, output).unwrap()
    }

    fn decompress(input: &[u8], output: &mut [u8]) -> usize {
        Decoder::new().decompress(input, output).unwrap()
    }
}
