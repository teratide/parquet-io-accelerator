use criterion::{criterion_group, criterion_main, Criterion};

mod compression;

criterion_group! {
  name = parquet;
  config = Criterion::default();
  targets =
    compression::bench
}
criterion_main!(parquet);
