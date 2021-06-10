use criterion::{measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion, Throughput};
use parquet_io_accelerator::compression::{
    Brotli, Compression, Snappy, Uncompressed, Zstd, GZIP, LZ4, LZO,
};
use rand::{prelude::SmallRng, RngCore, SeedableRng};

fn bench_compress<T: Compression>(group: &mut BenchmarkGroup<WallTime>, input: &[u8]) {
    // Allocate output buffer
    let mut compressed_output = vec![0; 2 * input.len()];

    // Benchmark compression
    group.throughput(Throughput::Bytes(input.len() as u64));
    group.bench_with_input(
        BenchmarkId::new(format!("{}/compress", T::NAME), input.len()),
        input,
        |b, input| {
            b.iter(|| {
                T::compress(&input, &mut compressed_output);
            })
        },
    );

    // Get compressed input
    let compressed_size = T::compress(&input, &mut compressed_output);
    let compressed_input = &compressed_output[..compressed_size];

    // Allocate output buffer
    let mut decompressed_output = vec![0; input.len()];

    // Benchmark decompression
    group.throughput(Throughput::Bytes(input.len() as u64));
    group.bench_with_input(
        BenchmarkId::new(format!("{}/decompress", T::NAME), compressed_input.len()),
        compressed_input,
        |b, input| {
            b.iter(|| {
                T::decompress(&input, &mut decompressed_output);
            })
        },
    );
}

pub(super) fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression");

    for &size in [8 * 1024 * 1024].iter() {
        let mut rng = SmallRng::seed_from_u64(1234);
        let mut input = vec![0; size];
        rng.fill_bytes(&mut input);
        input.iter_mut().for_each(|x| {
            *x = (*x).clamp(0, 100);
        });

        bench_compress::<Uncompressed>(&mut group, &input);
        bench_compress::<Snappy>(&mut group, &input);
        bench_compress::<GZIP<1>>(&mut group, &input);
        bench_compress::<LZO>(&mut group, &input);
        bench_compress::<Brotli<4096, 1, 22>>(&mut group, &input);
        bench_compress::<LZ4>(&mut group, &input);
        bench_compress::<Zstd<1>>(&mut group, &input);
    }

    group.finish();
}
