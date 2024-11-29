use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bench_quests {
    ($c:expr => $($quest:path: $part1:expr, $part2:expr, $part3:expr);*$(;)?) => {
        $({
            let input1 = include_str!(concat!("../../", stringify!($quest), "/src/part1.txt"));
            $c.bench_function(concat!(stringify!($quest), "_part1"), |b| b.iter(|| ($part1)(black_box(input1))));

            let input2 = include_str!(concat!("../../", stringify!($quest), "/src/part2.txt"));
            $c.bench_function(concat!(stringify!($quest), "_part2"), |b| b.iter(|| ($part2)(black_box(input2))));

            let input3 = include_str!(concat!("../../", stringify!($quest), "/src/part3.txt"));
            $c.bench_function(concat!(stringify!($quest), "_part3"), |b| b.iter(|| ($part3)(black_box(input3))));
        })*
    };
}

fn bench_all(c: &mut Criterion) {
    bench_quests!(c =>
        quest01: quest01::solve_part::<1>, quest01::solve_part::<2>, quest01::solve_part::<3>;
        quest02: quest02::solve_part1, quest02::solve_part2, quest02::solve_part3;
        quest03: |input| quest03::solve(input, false), |input| quest03::solve(input, false), |input| quest03::solve(input, true);
        quest04: quest04::solve_part12, quest04::solve_part12, quest04::solve_part3;
        quest05: quest05::solve_part1, quest05::solve_part2, quest05::solve_part3;
        quest06: quest06::solve_part1, quest06::solve_part2, quest06::solve_part3;
        quest07: quest07::solve_part1, quest07::solve_part2, quest07::solve_part3;
        quest08: quest08::solve_part1, quest08::solve_part2, quest08::solve_part3;
        quest09: quest09::solve_part1, quest09::solve_part2, quest09::solve_part3;
        quest10: quest10::solve_part1, quest10::solve_part2, quest10::solve_part3;
        quest11: quest11::solve_part1, quest11::solve_part2, quest11::solve_part3;
        quest12: quest12::solve_part12, quest12::solve_part12, quest12::solve_part3;
        quest13: quest13::solve_part12, quest13::solve_part12, quest13::solve_part3;
        quest14: quest14::solve_part1, quest14::solve_part2, quest14::solve_part3;
        quest15: quest15::solve_part1, quest15::solve_part2, quest15::solve_part3;
        quest16: quest16::solve_part1, quest16::solve_part2, quest16::solve_part3;
        quest17: quest17::solve_part12, quest17::solve_part12, quest17::solve_part3;
        quest18: quest18::solve_part1, quest18::solve_part2, quest18::solve_part3;
    );
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
