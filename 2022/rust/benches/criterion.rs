#[macro_use]
extern crate build_const;

use aoc2022::{day01, day02, day03, day04, day05, day06, day07, day08, day09};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

build_const!("aoc2022");

fn aoc2022_bench(c: &mut Criterion) {
    c.bench_function("day 1 part 1", |b| b.iter(|| day01::part1(black_box(DAY1))));
    c.bench_function("day 1 part 2", |b| b.iter(|| day01::part2(black_box(DAY1))));
    c.bench_function("day 2 part 1", |b| b.iter(|| day02::part1(black_box(DAY2))));
    c.bench_function("day 2 part 2", |b| b.iter(|| day02::part2(black_box(DAY2))));
    c.bench_function("day 3 part 1", |b| b.iter(|| day03::part1(black_box(DAY3))));
    c.bench_function("day 3 part 2", |b| b.iter(|| day03::part2(black_box(DAY3))));
    c.bench_function("day 4 part 1", |b| b.iter(|| day04::part1(black_box(DAY4))));
    c.bench_function("day 4 part 2", |b| b.iter(|| day04::part2(black_box(DAY4))));
    c.bench_function("day 5 part 1", |b| b.iter(|| day05::part1(black_box(DAY5))));
    c.bench_function("day 5 part 2", |b| b.iter(|| day05::part2(black_box(DAY5))));
    c.bench_function("day 6 part 1", |b| b.iter(|| day06::part1(black_box(DAY6))));
    c.bench_function("day 6 part 2", |b| b.iter(|| day06::part2(black_box(DAY6))));
    c.bench_function("day 7 part 1", |b| b.iter(|| day07::part1(black_box(DAY7))));
    c.bench_function("day 7 part 2", |b| b.iter(|| day07::part2(black_box(DAY7))));
    c.bench_function("day 8 part 1", |b| b.iter(|| day08::part1(black_box(DAY8))));
    c.bench_function("day 8 part 2", |b| b.iter(|| day08::part2(black_box(DAY8))));
    c.bench_function("day 9 part 1", |b| b.iter(|| day09::part1(black_box(DAY9))));
    c.bench_function("day 9 part 2", |b| b.iter(|| day09::part2(black_box(DAY9))));
}

criterion_group!(aoc2022, aoc2022_bench);
criterion_main!(aoc2022);
