mod uncompressed;
pub use uncompressed::Uncompressed;

mod snappy;
pub use snappy::Snappy;

mod gzip;
pub use gzip::GZIP;

mod lzo;
pub use lzo::LZO;

mod brotli;
pub use self::brotli::Brotli;

mod lz4;
pub use self::lz4::LZ4;

mod zstd;
pub use self::zstd::Zstd;

pub trait Compression {
    const NAME: &'static str;
    fn compress(input: &[u8], output: &mut [u8]) -> usize;
    fn decompress(input: &[u8], output: &mut [u8]) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static [u8] = &[
        1, 2, 3, 4, 5, 42, 42, 42, 42, 42, 42, 42, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ];

    fn validate<T: Compression>(input: &[u8]) {
        // Allocate output buffer
        let mut compressed_output = vec![0; 10 * input.len()];

        // Get compressed input
        let compressed_size = T::compress(&input, &mut compressed_output);
        let compressed_input = &compressed_output[..compressed_size];

        // Allocate output buffer
        let mut decompressed_output = vec![0; input.len()];
        let decompressed_size = T::decompress(compressed_input, &mut decompressed_output);
        assert_eq!(decompressed_size, input.len());
        assert_eq!(decompressed_output, input);
    }

    #[test]
    fn uncompressed() {
        validate::<Uncompressed>(INPUT);
    }

    #[test]
    fn snappy() {
        validate::<Snappy>(INPUT);
    }

    #[test]
    fn gzip() {
        validate::<GZIP<1>>(INPUT);
    }

    #[test]
    fn lzo() {
        validate::<LZO>(INPUT);
    }

    #[test]
    fn brotli() {
        validate::<Brotli<4096, 1, 22>>(INPUT);
    }

    #[test]
    fn lz4() {
        validate::<LZ4>(INPUT);
    }

    #[test]
    fn zstd() {
        validate::<Zstd<1>>(INPUT);
    }
}
