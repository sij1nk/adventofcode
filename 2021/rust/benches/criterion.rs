#[macro_use]
extern crate build_const;

use aoc2021::{day01, day02, day03 /*%IMPORT%*/};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

build_const!("aoc2021");

fn aoc2021_bench(c: &mut Criterion) {
    c.bench_function("day 1 part 1", |b| b.iter(|| day01::part1(black_box(DAY1))));
    c.bench_function("day 1 part 2", |b| b.iter(|| day01::part2(black_box(DAY1))));
    c.bench_function("day 2 part 1", |b| b.iter(|| day02::part1(black_box(DAY2))));
    c.bench_function("day 2 part 2", |b| b.iter(|| day02::part2(black_box(DAY2))));
    c.bench_function("day 3 part 1", |b| b.iter(|| day03::part1(black_box(DAY3))));
    c.bench_function("day 3 part 2", |b| b.iter(|| day03::part2(black_box(DAY3))));
    /*%CALL%*/
}

criterion_group!(aoc2021, aoc2021_bench);
criterion_main!(aoc2021);
