use super::Compression;

pub struct Uncompressed;

impl Compression for Uncompressed {
    const NAME: &'static str = "uncompressed";

    fn compress(input: &[u8], output: &mut [u8]) -> usize {
        output[..input.len()].copy_from_slice(input);
        input.len()
    }

    fn decompress(input: &[u8], output: &mut [u8]) -> usize {
        output[..input.len()].copy_from_slice(input);
        input.len()
    }
}
