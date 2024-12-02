#[macro_use]
extern crate build_const;

use aoc2024::{day01, day02};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

build_const!("aoc2024");

fn aoc2024_bench(c: &mut Criterion) {
    // c.bench_function("day 1 part 1", |b| b.iter(|| day01::part1(black_box(DAY1))));
    // c.bench_function("day 1 part 2", |b| b.iter(|| day01::part2(black_box(DAY1))));
    c.bench_function("day 2 part 1", |b| b.iter(|| day02::part1(black_box(DAY2))));
    c.bench_function("day 2 part 2", |b| b.iter(|| day02::part2(black_box(DAY2))));
}

criterion_group!(aoc2024, aoc2024_bench);
criterion_main!(aoc2024);
