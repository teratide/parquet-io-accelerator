use criterion::{BenchmarkId, Criterion, Throughput};
use parquet_io_accelerator::metadata::thrift::{
    AesGcmCtrV1, AesGcmV1, ColumnChunk, ColumnCryptoMetaData, ColumnMetaData, ColumnOrder,
    CompressionCodec, ConvertedType, DataPageHeader, DataPageHeaderV2, DictionaryPageHeader,
    Encoding, EncryptionAlgorithm, EncryptionWithFooterKey, FieldRepetitionType, FileMetaData,
    IndexPageHeader, IntType, KeyValue, LogicalType, PageEncodingStats, PageHeader, PageType,
    RowGroup, SchemaElement, SortingColumn, Statistics, Type, TypeDefinedOrder,
};
use thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};

pub(super) fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("metadata");

    // FileMetaData
    // todo(mb): get this from a real parquet file instead
    let input = FileMetaData::new(
        2,
        vec![SchemaElement::new(
            Some(Type::Boolean),
            Some(1),
            Some(FieldRepetitionType::Required),
            "boolean".to_string(),
            Some(1234),
            Some(ConvertedType::Int16),
            Some(1234),
            Some(1234),
            Some(0),
            Some(LogicalType::INTEGER(IntType {
                bit_width: 1,
                is_signed: false,
            })),
        )],
        1234,
        vec![RowGroup::new(
            vec![ColumnChunk::new(
                Some("/dev/null".to_string()),
                1234,
                Some(ColumnMetaData::new(
                    Type::Boolean,
                    vec![Encoding::Plain],
                    vec!["a".to_string(), "b".to_string()],
                    CompressionCodec::Brotli,
                    1234,
                    1234,
                    1234,
                    Some(vec![KeyValue::new(
                        "a".to_string(),
                        Some("test".to_string()),
                    )]),
                    1234,
                    Some(1234),
                    Some(1234),
                    Some(Statistics::new(
                        None,
                        None,
                        Some(1234),
                        Some(1234),
                        Some(vec![1, 2, 3, 4]),
                        Some(vec![1, 2, 3, 4]),
                    )),
                    Some(vec![PageEncodingStats::new(
                        PageType::DataPage,
                        Encoding::BitPacked,
                        1234,
                    )]),
                    Some(1234),
                )),
                Some(1234),
                Some(1234),
                Some(1234),
                Some(1234),
                Some(ColumnCryptoMetaData::ENCRYPTIONWITHFOOTERKEY(
                    EncryptionWithFooterKey::new(),
                )),
                Some(vec![1, 2, 3, 4]),
            )],
            12345,
            617,
            Some(vec![SortingColumn::new(0, true, true)]),
            Some(1234),
            Some(1234),
            Some(1),
        )],
        Some(vec![KeyValue::new("a".to_string(), Some("b".to_string()))]),
        Some("rust".to_string()),
        Some(vec![ColumnOrder::TYPEORDER(TypeDefinedOrder::new())]),
        Some(EncryptionAlgorithm::AESGCMV1(AesGcmV1::new(
            Some(vec![1, 2, 3, 4]),
            Some(vec![1, 2, 3, 4]),
            Some(true),
        ))),
        Some(vec![1, 2, 3, 4]),
    );

    // First encode to get encoded size
    let mut buffer = Vec::with_capacity(4096); // todo(mb): figure out a way to correctly pre-allocate
    let mut out_protocol = TCompactOutputProtocol::new(&mut buffer);
    input.write_to_out_protocol(&mut out_protocol).unwrap();
    let encoded_size = buffer.len();

    // Benchmark decoding
    group.throughput(Throughput::Bytes(encoded_size as u64));
    group.bench_with_input(
        BenchmarkId::new("file_meta_data/decode", encoded_size),
        &buffer[..],
        |b, input| {
            b.iter(|| {
                FileMetaData::read_from_in_protocol(&mut TCompactInputProtocol::new(input)).unwrap()
            });
        },
    );

    // Benchmark encoding
    group.throughput(Throughput::Elements(1));
    group.bench_with_input(
        BenchmarkId::new("file_meta_data/encode", 1),
        &input,
        |b, input| {
            b.iter(|| {
                input
                    .write_to_out_protocol(&mut TCompactOutputProtocol::new(&mut buffer))
                    .unwrap()
            });
        },
    );

    // PageHeader
    let input = PageHeader::new(
        PageType::DataPageV2,
        1234,
        1234,
        Some(1234),
        Some(DataPageHeader::new(
            1234,
            Encoding::BitPacked,
            Encoding::ByteStreamSplit,
            Encoding::DeltaByteArray,
            Some(Statistics::new(
                None,
                None,
                Some(1234),
                Some(1234),
                Some(vec![1, 2, 3, 4]),
                Some(vec![1, 2, 3, 4]),
            )),
        )),
        Some(IndexPageHeader::new()),
        Some(DictionaryPageHeader::new(
            1234,
            Encoding::DeltaLengthByteArray,
            Some(false),
        )),
        Some(DataPageHeaderV2::new(
            1234,
            1234,
            1234,
            Encoding::Plain,
            1234,
            1234,
            Some(true),
            Statistics::new(
                None,
                None,
                Some(1234),
                Some(1234),
                Some(vec![1, 2, 3, 4]),
                Some(vec![1, 2, 3, 4]),
            ),
        )),
    );

    // First encode to get encoded size
    let mut buffer = Vec::with_capacity(4096); // todo(mb): figure out a way to correctly pre-allocate
    let mut out_protocol = TCompactOutputProtocol::new(&mut buffer);
    input.write_to_out_protocol(&mut out_protocol).unwrap();
    let encoded_size = buffer.len();

    // Benchmark decoding
    group.throughput(Throughput::Bytes(encoded_size as u64));
    group.bench_with_input(
        BenchmarkId::new("page_header/decode", encoded_size),
        &buffer[..],
        |b, input| {
            b.iter(|| {
                PageHeader::read_from_in_protocol(&mut TCompactInputProtocol::new(input)).unwrap()
            });
        },
    );

    // Benchmark encoding
    group.throughput(Throughput::Elements(1));
    group.bench_with_input(
        BenchmarkId::new("page_header/encode", 1),
        &input,
        |b, input| {
            b.iter(|| {
                input
                    .write_to_out_protocol(&mut TCompactOutputProtocol::new(&mut buffer))
                    .unwrap()
            });
        },
    );

    group.finish()
}
