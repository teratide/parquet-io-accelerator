# Parquet I/O accelerator

## Requirements

- [Rust 1.51+](https://rust-lang.org)
- [Thrift](https://thrift.apache.org/)

## Parquet benchmarks

### [Compression](https://github.com/apache/parquet-format/blob/master/Compression.md)

```
cargo bench compression
```

- [x] Uncompressed
- [x] Snappy
- [x] GZIP
- [x] LZO
- [ ] Brotli
- [x] LZ4
- [x] Zstd

### [Metadata](https://github.com/apache/parquet-format#metadata)

```
cargo bench metadata
```

- [x] FileMetaData
- [x] PageHeader

### [Encodings](https://github.com/apache/parquet-format/blob/master/Encodings.md)

- [ ] Plain
- [ ] Dictionary Encoding
- [ ] Run Length Encoding
- [ ] Bit-packed (deprecated)
- [ ] Delta encoding
- [ ] Delta-length byte array
- [ ] Delta Strings
- [ ] Byte Stream Split
