use criterion::{criterion_group, criterion_main, Criterion};

mod compression;
mod metadata;

criterion_group! {
  name = parquet;
  config = Criterion::default();
  targets =
    compression::bench,
    metadata::bench
}
criterion_main!(parquet);
