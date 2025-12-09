use criterion::{criterion_group, criterion_main, Criterion};
use day08::{
    read_input,
    solution::{part1, part2},
};
use std::hint::black_box;

fn bench(c: &mut Criterion) {
    let input = read_input().unwrap();

    c.bench_function("day 8 part 1", |b| {
        b.iter(|| part1(black_box(&input), black_box(1000)))
    });
    c.bench_function("day 8 part 2", |b| b.iter(|| part2(black_box(&input))));
}

criterion_group!(day08, bench);
criterion_main!(day08);
