const INITIAL_BLOCKS: usize = 202400000;
const ACOLYTES: usize = 10;

pub fn solve_part3(input: &str) -> usize {
    let priests = input.trim().parse::<usize>().unwrap();

    // Find the cycle of thicknesses, which must exists seeing as ACOLYTES is pretty small.
    let cycle = {
        let mut thickness = priests % ACOLYTES + ACOLYTES;

        let mut cycle = vec![thickness];
        loop {
            let next = (thickness * priests) % ACOLYTES + ACOLYTES;
            thickness = next;
            if cycle.contains(&next) {
                break;
            }
            cycle.push(next);
        }
        cycle
    };

    // Find out how much thickness each cycle adds.
    let thickness_per_cycle = cycle.iter().sum::<usize>();

    // How many blocks are needed for the first cycle?
    let blocks_for_first_cycle = cycle
        .iter()
        .enumerate()
        .map(|(i, thickness)| ((2 * 1 + 1) + i) * thickness)
        .sum::<usize>();

    // How many additional blocks are needed per cycle for each cycle? (Think of this like acceleration)
    let additional_blocks_per_cycle_per_cycle = cycle
        .iter()
        .enumerate()
        .map(|(i, thickness)| (2 * 5 + 1 + i) * thickness)
        .sum::<usize>()
        - blocks_for_first_cycle;

    // Find the minimum number of layers needed to reach to overtake the initial blocks.
    let layers = find_minimum_layer(&cycle, blocks_for_first_cycle, additional_blocks_per_cycle_per_cycle);

    // Simulate the columns and find out how many extra blocks are needed.
    let column_zero_height = layers / cycle.len() * thickness_per_cycle + cycle[0] + 1;
    let heights =
        [column_zero_height, column_zero_height - 1]
            .into_iter()
            .chain(
                (0..layers - 1)
                    .zip(cycle.iter().cycle())
                    .scan(column_zero_height - 1, |cur_height, (_, t)| {
                        *cur_height -= *t;
                        Some(*cur_height)
                    }),
            );
    let total_blocks = calculate_total_blocks((layers + 1) * 2 - 1, heights.clone(), priests, ACOLYTES);
    let would_need = total_blocks - INITIAL_BLOCKS;
    return would_need;
}

fn find_minimum_layer(
    cycle: &Vec<usize>,
    blocks_for_first_cycle: usize,
    additional_blocks_per_cycle_per_cycle: usize,
) -> usize {
    let mut left_cycles = 0;
    let mut right_cycles = 40_000;
    let mut cycles = (left_cycles + right_cycles) / 2;

    loop {
        let blocks_needed = calculate_blocks_needed(
            1 + cycles * cycle.len(),
            cycle,
            blocks_for_first_cycle,
            additional_blocks_per_cycle_per_cycle,
        );
        if blocks_needed > INITIAL_BLOCKS {
            right_cycles = cycles;
        } else {
            left_cycles = cycles;
        }
        if right_cycles - left_cycles <= 1 {
            break;
        }
        cycles = (left_cycles + right_cycles) / 2;
    }
    1 + cycles * cycle.len()
}

fn calculate_blocks_needed(
    layers: usize,
    cycle: &Vec<usize>,
    blocks_for_first_cycle: usize,
    additional_blocks_per_cycle_per_cycle: usize,
) -> usize {
    let cycles_elapsed = layers / cycle.len();
    let blocks_needed = blocks_for_first_cycle * (cycles_elapsed + 1)
        + additional_blocks_per_cycle_per_cycle * (1..=cycles_elapsed).sum::<usize>();
    blocks_needed
}

fn calculate_total_blocks(
    width: usize,
    mut heights: impl Iterator<Item = usize>,
    priests: usize,
    acolytes: usize,
) -> usize {
    let first = heights.next().unwrap();
    let mut total_blocks = first;
    total_blocks -= (width * priests * first) % acolytes;
    heights.enumerate().for_each(|(i, height)| {
        let removed = if i == (width + 1) / 2 - 2 {
            0
        } else {
            (width * priests * height) % acolytes
        };
        total_blocks += 2 * (height - removed);
    });
    total_blocks
}
