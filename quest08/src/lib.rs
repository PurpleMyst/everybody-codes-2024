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

pub fn solve_part3(input: &str) -> usize {
    const INITIAL_BLOCKS: usize = 202400000000;
    const ACOLYTES: usize = 10;

    let priests = input.trim().parse::<usize>().unwrap();
    let mut blocks_remaining = INITIAL_BLOCKS;
    let mut thickness = 1;
    let mut heights = Vec::new();

    for n in odd_numbers() {
        let blocks_for_this_layer = n * thickness;

        heights.iter_mut().for_each(|h| *h += thickness);
        heights.push(thickness);

        if blocks_remaining >= blocks_for_this_layer {
            blocks_remaining -= blocks_for_this_layer;
        } else {
            let total_blocks = calculate_total_blocks(&heights, priests, ACOLYTES);
            let would_need = total_blocks - INITIAL_BLOCKS;
            return would_need;
        }

        thickness = (thickness * priests) % ACOLYTES + ACOLYTES;
    }

    unreachable!();
}

fn calculate_total_blocks(heights: &[usize], priests: usize, acolytes: usize) -> usize {
    let width = heights.len() * 2 - 1;
    let mut total_blocks = heights[0] + heights.iter().skip(1).sum::<usize>() * 2;
    heights.iter().enumerate().rev().skip(1).for_each(|(i, height)| {
        let removed = (width * priests * *height) % acolytes;
        total_blocks -= (if i == 0 { 1 } else { 2 }) * removed;
    });
    total_blocks
}
