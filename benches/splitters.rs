use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use splitter::*;

fn make_text(len: usize) -> String {
    use std::iter::repeat;

    let mut s = String::with_capacity(len);
    // Start with some 'a' characters.
    s.extend(repeat('a').take(len / 2));
    // Then add a single ' '.
    s.push(' ');
    let remaining = len - 1 - (len / 2);
    // Then fill the remaining space with more 'a's.
    s.extend(repeat('a').take(remaining));

    debug_assert_eq!(s.len(), s.capacity());
    debug_assert_eq!(s.len(), len);
    s
}

pub fn criterion_benchmark(c: &mut Criterion) {
    const KB: usize = 1024;

    let mut group = c.benchmark_group("Splitters");

    let functions = [
        // ("original", remove_through_first_char),
        (
            "v1",
            remove_through_first_char_variant_1 as fn(&str, char) -> &str,
        ),
        ("v2", remove_through_first_char_variant_2),
        ("v3", remove_through_first_char_variant_3),
        ("v4", remove_through_first_char_variant_4),
        ("v5", remove_through_first_char_variant_5),
    ];

    for &len in &[4, 8, 16, 256, KB, 4 * KB, 16 * KB, 32 * KB] {
        let text = make_text(len);
        group.throughput(Throughput::Bytes(len as u64));
        // For moderate sizes, benchmark the slow function.
        if len <= 16 * KB {
            group.bench_with_input(BenchmarkId::new("original", len), &len, |b, _len| {
                b.iter(|| remove_through_first_char(black_box(&text), ' '))
            });
        }
        // Benchmark the fast functions.
        for &(name, function) in &functions {
            group.bench_with_input(BenchmarkId::new(name, len), &len, |b, _len| {
                b.iter(|| function(black_box(&text), ' '))
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
