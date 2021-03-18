#[macro_use]
extern crate build_const;

use aoc2020::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

build_const!("aoc2020");

fn aoc2020_bench(c: &mut Criterion) {
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
    c.bench_function("day 9 part 1", |b| {
        b.iter(|| day09::part1(black_box(DAY9), 25))
    });
    c.bench_function("day 9 part 2", |b| {
        b.iter(|| day09::part2(black_box(DAY9), 25))
    });
    c.bench_function("day 10 part 1", |b| {
        b.iter(|| day10::part1(black_box(DAY10)))
    });
    c.bench_function("day 10 part 2", |b| {
        b.iter(|| day10::part2(black_box(DAY10)))
    });
    c.bench_function("day 11 part 1", |b| {
        b.iter(|| day11::part1(black_box(DAY11)))
    });
    c.bench_function("day 11 part 2", |b| {
        b.iter(|| day11::part2(black_box(DAY11)))
    });
    c.bench_function("day 12 part 1", |b| {
        b.iter(|| day12::part1(black_box(DAY12)))
    });
    c.bench_function("day 12 part 2", |b| {
        b.iter(|| day12::part2(black_box(DAY12)))
    });
}

criterion_group!(aoc2020, aoc2020_bench);
criterion_main!(aoc2020);
