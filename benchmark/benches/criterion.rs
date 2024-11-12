use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_quest01(c: &mut Criterion) {
    let input1 = include_str!("../../quest01/src/part1.txt");
    c.bench_function("quest01_part1", |b| {
        b.iter(|| quest01::solve_part::<1>(black_box(input1)))
    });

    let input2 = include_str!("../../quest01/src/part2.txt");
    c.bench_function("quest01_part2", |b| {
        b.iter(|| quest01::solve_part::<2>(black_box(input2)))
    });

    let input3 = include_str!("../../quest01/src/part3.txt");
    c.bench_function("quest01_part3", |b| {
        b.iter(|| quest01::solve_part::<3>(black_box(input3)))
    });
}

fn bench_quest02(c: &mut Criterion) {
    let input1 = include_str!("../../quest02/src/part1.txt");
    c.bench_function("quest02_part1", |b| {
        b.iter(|| quest02::solve_part1(black_box(input1)))
    });

    let input2 = include_str!("../../quest02/src/part2.txt");
    c.bench_function("quest02_part2", |b| {
        b.iter(|| quest02::solve_part2(black_box(input2)))
    });

    let input3 = include_str!("../../quest02/src/part3.txt");
    c.bench_function("quest02_part3", |b| {
        b.iter(|| quest02::solve_part3(black_box(input3)))
    });
}

fn bench_quest03(c: &mut Criterion) {
    let input1 = include_str!("../../quest03/src/part1.txt");
    c.bench_function("quest03_part1", |b| {
        b.iter(|| quest03::solve(black_box(input1), black_box(false)))
    });

    let input2 = include_str!("../../quest03/src/part2.txt");
    c.bench_function("quest03_part2", |b| {
        b.iter(|| quest03::solve(black_box(input2), black_box(false)))
    });

    let input3 = include_str!("../../quest03/src/part3.txt");
    c.bench_function("quest03_part3", |b| {
        b.iter(|| quest03::solve(black_box(input3), black_box(true)))
    });
}

fn bench_quest04(c: &mut Criterion) {
    let input1 = include_str!("../../quest04/src/part1.txt");
    c.bench_function("quest04_part1", |b| {
        b.iter(|| quest04::solve_part12(black_box(input1)))
    });

    let input2 = include_str!("../../quest04/src/part2.txt");
    c.bench_function("quest04_part2", |b| {
        b.iter(|| quest04::solve_part12(black_box(input2)))
    });

    let input3 = include_str!("../../quest04/src/part3.txt");
    c.bench_function("quest04_part3", |b| {
        b.iter(|| quest04::solve_part3(black_box(input3)))
    });
}

fn bench_quest05(c: &mut Criterion) {
    let input1 = include_str!("../../quest05/src/part1.txt");
    c.bench_function("quest05_part1", |b| {
        b.iter(|| quest05::solve_part1(black_box(input1)))
    });

    let input2 = include_str!("../../quest05/src/part2.txt");
    c.bench_function("quest05_part2", |b| {
        b.iter(|| quest05::solve_part2(black_box(input2)))
    });

    let input3 = include_str!("../../quest05/src/part3.txt");
    c.bench_function("quest05_part3", |b| {
        b.iter(|| quest05::solve_part3(black_box(input3)))
    });
}

fn bench_quest06(c: &mut Criterion) {
    let input1 = include_str!("../../quest06/src/part1.txt");
    c.bench_function("quest06_part1", |b| {
        b.iter(|| quest06::solve::<false>(black_box(input1)))
    });

    let input2 = include_str!("../../quest06/src/part2.txt");
    c.bench_function("quest06_part2", |b| {
        b.iter(|| quest06::solve::<false>(black_box(input2)))
    });

    let input3 = include_str!("../../quest06/src/part3.txt");
    c.bench_function("quest06_part3", |b| {
        b.iter(|| quest06::solve::<true>(black_box(input3)))
    });
}

fn bench_all(c: &mut Criterion) {
    bench_quest01(c);
    bench_quest02(c);
    bench_quest03(c);
    bench_quest04(c);
    bench_quest05(c);
    bench_quest06(c);
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
