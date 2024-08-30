use benchmark_json::{serde_read_rustdoc, simd_read_rustdoc};
use criterion::{criterion_group, criterion_main};
use criterion::{BatchSize, BenchmarkId, Criterion};
use std::hint::black_box;

const LINTS: &[&str] = &[
    "enum_repr_int_changed",
    "trait_newly_sealed",
    "struct_pub_field_missing",
    "broken_rustdoc",
];

const CSC_ROOT_PATH: &str = "/home/david/oss/cargo-semver-checks/";

fn bench_read_rustdocs(c: &mut Criterion) {
    let mut group = c.benchmark_group("ReadRustDoc");
    for p in LINTS.into_iter() {
        let full_path = format!(
            "{}/localdata/test_data/{}/old/rustdoc.json",
            CSC_ROOT_PATH, p
        );
        let file_data = std::fs::read(full_path.clone()).expect("Failed to read file");
        let file_data_string =
            std::fs::read_to_string(full_path.clone()).expect("Failed to read file");
        group.bench_with_input(BenchmarkId::new("SIMD", p), &file_data, |b, file_data| {
            b.iter_batched(
                || (file_data.clone()),
                |mut data| black_box(simd_read_rustdoc(black_box(&mut data))),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(
            BenchmarkId::new("SERDE", p),
            &file_data_string,
            |b, file_data_string| {
                b.iter_batched(
                    || (file_data_string.clone()),
                    |data| black_box(serde_read_rustdoc(black_box(&data))),
                    BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_read_rustdocs);
criterion_main!(benches);
