use std::{fs, hint::black_box};

use criterion::{Criterion, criterion_group, criterion_main};
use day2_part2::do_aoc;

fn criterion_benchmark(c: &mut Criterion) {
    let file = fs::read_to_string("input").unwrap();

    c.bench_function("day2_part1", |b| b.iter(|| do_aoc(black_box(file.clone()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
