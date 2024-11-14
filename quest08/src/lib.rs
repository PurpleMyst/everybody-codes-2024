fn odd_numbers() -> impl Iterator<Item = usize> {
    (1..).map(|n| 2 * n - 1)
}

pub fn solve_part1(input: &str) -> impl std::fmt::Display {
    let mut blocks_remaining = input.trim().parse::<usize>().unwrap();
    for n in odd_numbers() {
        if blocks_remaining > n {
            blocks_remaining -= n;
        } else {
            let would_need = n - blocks_remaining;
            return n * would_need;
        }
    }

    unreachable!()
}

pub fn solve_part2(input: &str) -> impl std::fmt::Display {
    const ACOLYTES: usize = 1111;

    let priests = input.trim().parse::<usize>().unwrap();
    let mut blocks = 20240000;

    let mut thickness = 1;

    for n in odd_numbers() {
        let n = n * thickness;
        if blocks > n {
            blocks -= n;
        } else {
            let would_need = n - blocks;
            return would_need * n / thickness;
        }

        thickness = (thickness * priests) % ACOLYTES;
    }

    unreachable!();
}

mod part3;
pub use part3::solve_part3;
