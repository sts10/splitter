// lot of copy and pasting from:
// https://bheisler.github.io/criterion.rs/book/getting_started.html
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// extern crate aho_corasick;
// use aho_corasick::AhoCorasick;
use std::str;

fn remove_through_first_char(l: &str, ch: char) -> String {
    if l.contains(ch) {
        let mut word_vec = l.split(ch).collect::<Vec<&str>>();
        word_vec.remove(0);
        return word_vec.join(&ch.to_string());
    } else {
        l.to_string()
    }
}

fn remove_through_first_char_variant_1(s: &str, ch: char) -> &str {
    let mut split = s.splitn(2, ch);
    let before_ch = split
        .next()
        .expect("there should always be at least one part");
    match split.next() {
        Some(after_ch) => after_ch,
        None => before_ch,
    }
}

fn remove_through_first_char_variant_2(s: &str, ch: char) -> &str {
    // This is perhaps cleaner, but it performs an (unnecessary) allocation
    // for the Vec, although a very small one (just two fat pointers).
    let split: Vec<&str> = s.splitn(2, ch).collect();
    match &split[..] {
        [_before_ch, after_ch] => after_ch,
        [whole] => whole,
        _ => unreachable!("Not one or two parts?"),
    }
}

fn remove_through_first_char_variant_3(s: &str, ch: char) -> &str {
    // This does not use str::splitn(), but probably has an extra bounds check.
    // But perhaps the compiler will optmize it out? I don't know, you'd need
    // to measure.
    match s.find(ch) {
        None => s, // not found => return the whole string
        Some(pos) => &s[pos + 1..],
    }
}

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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
