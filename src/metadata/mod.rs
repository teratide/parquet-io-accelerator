#[path = "parquet.rs"]
pub mod thrift;

#[cfg(test)]
mod tests {
    use super::thrift::FileMetaData;
    use crate::metadata::thrift::{
        AesGcmV1, ColumnChunk, ColumnCryptoMetaData, ColumnMetaData, ColumnOrder, CompressionCodec,
        ConvertedType, DataPageHeader, DataPageHeaderV2, DictionaryPageHeader, Encoding,
        EncryptionAlgorithm, EncryptionWithFooterKey, FieldRepetitionType, IndexPageHeader,
        IntType, KeyValue, LogicalType, PageEncodingStats, PageHeader, PageType, RowGroup,
        SchemaElement, SortingColumn, Statistics, Type, TypeDefinedOrder,
    };
    use ::thrift::protocol::TCompactOutputProtocol;
    use thrift::protocol::TCompactInputProtocol;

    #[test]
    fn file_metadata_roundtrip() {
        let file_metadata = FileMetaData::new(
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
        let mut buffer = vec![];
        let mut out_protocol = TCompactOutputProtocol::new(&mut buffer);
        file_metadata
            .write_to_out_protocol(&mut out_protocol)
            .unwrap();

        let mut in_protocol = TCompactInputProtocol::new(&buffer[..]);
        let out_file_metadata = FileMetaData::read_from_in_protocol(&mut in_protocol).unwrap();

        assert_eq!(file_metadata, out_file_metadata);
    }

    #[test]
    fn page_header_roundtrip() {
        let page_header = PageHeader::new(
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

        let mut buffer = vec![];
        let mut out_protocol = TCompactOutputProtocol::new(&mut buffer);
        page_header
            .write_to_out_protocol(&mut out_protocol)
            .unwrap();

        let mut in_protocol = TCompactInputProtocol::new(&buffer[..]);
        let out_page_header = PageHeader::read_from_in_protocol(&mut in_protocol).unwrap();

        assert_eq!(page_header, out_page_header);
    }
}
