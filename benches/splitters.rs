// lot of copy and pasting from:
// https://bheisler.github.io/criterion.rs/book/getting_started.html
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use splitter::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Splitters");
    let sample_text = "word1 word2";

    group.bench_function("Basic", |b| {
        b.iter(|| remove_through_first_char(black_box(sample_text), ' '))
    });
    group.bench_function("V1", |b| {
        b.iter(|| remove_through_first_char_variant_1(black_box(sample_text), ' '))
    });
    group.bench_function("V2", |b| {
        b.iter(|| remove_through_first_char_variant_2(black_box(sample_text), ' '))
    });
    group.bench_function("V3", |b| {
        b.iter(|| remove_through_first_char_variant_3(black_box(sample_text), ' '))
    });
    group.bench_function("V4", |b| {
        b.iter(|| remove_through_first_char_variant_4(black_box(sample_text), ' '))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
