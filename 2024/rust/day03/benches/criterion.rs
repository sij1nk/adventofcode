use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day03::{
    read_input,
    solution::{part1, part2},
};

fn bench(c: &mut Criterion) {
    let input = read_input().unwrap();

    c.bench_function("part 1", |b| b.iter(|| part1(black_box(&input))));
    c.bench_function("part 2", |b| b.iter(|| part2(black_box(&input))));
}

criterion_group!(day03, bench);
criterion_main!(day03);
