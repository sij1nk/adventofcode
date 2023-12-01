#[macro_use]
extern crate build_const;

use aoc2022::day01;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

build_const!("aoc2022");

fn aoc2022_bench(c: &mut Criterion) {
    c.bench_function("day 1 part 1", |b| b.iter(|| day01::part1(black_box(DAY1))));
    c.bench_function("day 1 part 2", |b| b.iter(|| day01::part2(black_box(DAY1))));
}

criterion_group!(aoc2022, aoc2022_bench);
criterion_main!(aoc2022);
