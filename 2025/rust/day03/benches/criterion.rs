use criterion::{criterion_group, criterion_main, Criterion};
use day03::{
    read_input,
    solution::{part1, part2},
};
use std::hint::black_box;

fn bench(c: &mut Criterion) {
    let input = read_input().unwrap();

    c.bench_function("day 3 part 1", |b| b.iter(|| part1(black_box(&input))));
    c.bench_function("day 3 part 2", |b| b.iter(|| part2(black_box(&input))));
}

criterion_group!(day03, bench);
criterion_main!(day03);
