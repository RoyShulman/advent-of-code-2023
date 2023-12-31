use aoc::{
    day6::{self},
    day8::{self},
    utils::{get_day_input, parse_input},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark_day6(c: &mut Criterion) {
    let input = parse_input(get_day_input("day6"));
    c.bench_function("day6", |b| b.iter(|| day6::part2(black_box(&input))));
}

pub fn benchmark_day8(c: &mut Criterion) {
    let input = parse_input(get_day_input("day8"));
    c.bench_function("day8", |b| b.iter(|| day8::part2(black_box(&input))));
}

// criterion_group!(benches, benchmark_day6);
criterion_group!(benches, benchmark_day8);
criterion_main!(benches);
