use criterion::{criterion_group, criterion_main, Criterion};

fn bench_all(c: &mut Criterion) {
    // Benchmarks for quest01
    let input1_q1 = include_str!("../../quest01/src/part1.txt");
    c.bench_function("quest01_part1", |b| {
        b.iter(|| quest01::solve_part::<1>(input1_q1))
    });

    let input2_q1 = include_str!("../../quest01/src/part2.txt");
    c.bench_function("quest01_part2", |b| {
        b.iter(|| quest01::solve_part::<2>(input2_q1))
    });

    let input3_q1 = include_str!("../../quest01/src/part3.txt");
    c.bench_function("quest01_part3", |b| {
        b.iter(|| quest01::solve_part::<3>(input3_q1))
    });

    // Benchmarks for quest02
    let input1_q2 = include_str!("../../quest02/src/part1.txt");
    c.bench_function("quest02_part1", |b| {
        b.iter(|| quest02::solve_part1(input1_q2))
    });

    let input2_q2 = include_str!("../../quest02/src/part2.txt");
    c.bench_function("quest02_part2", |b| {
        b.iter(|| quest02::solve_part2(input2_q2))
    });

    let input3_q2 = include_str!("../../quest02/src/part3.txt");
    c.bench_function("quest02_part3", |b| {
        b.iter(|| quest02::solve_part3(input3_q2))
    });

    // Benchmarks for quest03
    let input1_q3 = include_str!("../../quest03/src/part1.txt");
    c.bench_function("quest03_part1", |b| {
        b.iter(|| quest03::solve(input1_q3, false))
    });

    let input2_q3 = include_str!("../../quest03/src/part2.txt");
    c.bench_function("quest03_part2", |b| {
        b.iter(|| quest03::solve(input2_q3, false))
    });

    let input3_q3 = include_str!("../../quest03/src/part3.txt");
    c.bench_function("quest03_part3", |b| {
        b.iter(|| quest03::solve(input3_q3, true))
    });

    // Benchmarks for quest04
    let input1_q4 = include_str!("../../quest04/src/part1.txt");
    c.bench_function("quest04_part1", |b| {
        b.iter(|| quest04::solve_part12(input1_q4))
    });

    let input2_q4 = include_str!("../../quest04/src/part2.txt");
    c.bench_function("quest04_part2", |b| {
        b.iter(|| quest04::solve_part12(input2_q4))
    });

    let input3_q4 = include_str!("../../quest04/src/part3.txt");
    c.bench_function("quest04_part3", |b| {
        b.iter(|| quest04::solve_part3(input3_q4))
    });
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
