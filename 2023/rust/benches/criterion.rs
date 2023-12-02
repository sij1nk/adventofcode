#[macro_use]
extern crate build_const;

#[rustfmt::skip]
use aoc2023::{day1, /*%criterion.rs_import%*/};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

build_const!("aoc2023");

fn aoc2023_bench(c: &mut Criterion) {
    c.bench_function("day 1 part 1", |b| b.iter(|| day1::part1(black_box(DAY1))));
    c.bench_function("day 1 part 2", |b| b.iter(|| day1::part2(black_box(DAY1))));
    /*%criterion.rs_call%*/
}

criterion_group!(aoc2023, aoc2023_bench);
criterion_main!(aoc2023);
