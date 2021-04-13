use criterion::{black_box, criterion_group, criterion_main, Criterion};

use splitter::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Splitters");
    let sample_text = "word1 word2";

    let functions = [
        // ("original", remove_through_first_char),
        ("v1", remove_through_first_char_variant_1 as fn(&str, char) -> &str),
        ("v2", remove_through_first_char_variant_2),
        ("v3", remove_through_first_char_variant_3),
        ("v4", remove_through_first_char_variant_4),
    ];

    group.bench_function("original", |b| {
        b.iter(|| remove_through_first_char(black_box(sample_text), ' '))
    });
    for &(name, function) in &functions {
        group.bench_function(name, |b| {
            b.iter(|| function(black_box(sample_text), ' '))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
