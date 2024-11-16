use std::fmt::Display;

use grid::Grid;
use itertools::iproduct;

const GRID_SIDE: usize = 8;

const PATTERN_COORDS: [usize; 4] = [0, 1, 6, 7];
const UNKNOWN: u8 = b'?';
const EMPTY: u8 = b'.';

pub fn solve_part1(input: &str) -> impl Display {
    let grid = Grid::from_vec(
        input.bytes().filter(|b| !b.is_ascii_whitespace()).collect::<Vec<_>>(),
        GRID_SIDE,
    );
    let mut result = String::with_capacity(6 * 6);
    iproduct!(2..6, 2..6).for_each(|(y, x)| {
        let row_choices = PATTERN_COORDS.map(|x| grid[(y, x)]);
        let col_choices = PATTERN_COORDS.map(|y| grid[(y, x)]);
        let choice = row_choices.into_iter().find(|&b| col_choices.contains(&b)).unwrap();
        result.push(choice as char);
    });
    result
}

fn power(s: impl IntoIterator<Item = u8>) -> usize {
    s.into_iter()
        .enumerate()
        .map(|(i, c)| (1 + i) * usize::from(c - b'A' + 1))
        .sum()
}

pub fn solve_part2(input: &str) -> impl Display {
    let grid: Grid<u8> = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into();
    let w = grid.cols();
    let h = grid.rows();

    let mut result = 0;

    for (base_y, base_x) in iproduct!((0..h).step_by(GRID_SIDE), (0..w).step_by(GRID_SIDE + 1)) {
        result += power(iproduct!(2..6, 2..6).map(|(y, x)| {
            let row_choices = PATTERN_COORDS.map(|x| grid[(base_y + y, base_x + x)]);
            let col_choices = PATTERN_COORDS.map(|y| grid[(base_y + y, base_x + x)]);
            let choice = row_choices.into_iter().find(|&b| col_choices.contains(&b)).unwrap();
            choice
        }));
    }

    result
}

fn only_one<T>(mut iter: impl Iterator<Item = T>) -> Option<T> {
    if let Some(first) = iter.next() {
        if iter.next().is_none() {
            return Some(first);
        }
    }
    None
}

pub fn solve_part3(input: &str) -> impl Display {
    let mut grid: Grid<u8> = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into();
    let w = grid.cols();
    let h = grid.rows();

    let mut empties = Vec::with_capacity(4);
    let mut used = Vec::with_capacity(4);

    // Run twice to ensure all patterns are deduced.
    for (_, base_y, base_x) in iproduct!(0..2, (0..h).step_by(6).take(h / 6), (0..w).step_by(6).take(w / 6)) {
        // Solve grid using the known patterns.
        for y in 2..6 {
            for x in 2..6 {
                let row_choices = PATTERN_COORDS.map(|x| grid[(base_y + y, base_x + x)]);
                let col_choices = PATTERN_COORDS.map(|y| grid[(base_y + y, base_x + x)]);

                if let Some(choice) = row_choices
                    .into_iter()
                    .filter(|&b| b != UNKNOWN)
                    .find(|&b| col_choices.contains(&b))
                {
                    grid[(base_y + y, base_x + x)] = choice;
                }
            }
        }

        // Check for deducible patterns in columns.
        for x in 2..6 {
            // Find empty spots and used patterns in the column.
            empties.clear();
            used.clear();
            for y in 2..6 {
                match grid[(base_y + y, base_x + x)] {
                    EMPTY => empties.push(y),
                    b => used.push(b),
                }
            }
            let [y] = empties[..] else {
                continue;
            };

            // Check for unused known patterns.
            let col_choices = PATTERN_COORDS.map(|y| grid[(base_y + y, base_x + x)]);
            let leftovers = col_choices
                .into_iter()
                .filter(|&b| b != UNKNOWN)
                .filter(|b| !used.contains(b));

            // When there's only one pattern left, fill it in.
            if let Some(leftover) = only_one(leftovers) {
                // We can also deduce that an unknown in the corresponding row must match the pattern we're
                // filling in.
                if let Some(&x) = PATTERN_COORDS
                    .iter()
                    .find(|&x| grid[(base_y + y, base_x + *x)] == UNKNOWN)
                {
                    grid[(base_y + y, base_x + x)] = leftover;
                }

                grid[(base_y + y, base_x + x)] = leftover;
            }
        }

        // Check for deducible patterns in rows, similar to the column check.
        for y in 2..6 {
            empties.clear();
            used.clear();

            for x in 2..6 {
                match grid[(base_y + y, base_x + x)] {
                    EMPTY => empties.push(x),
                    b => used.push(b),
                }
            }
            let [x] = empties[..] else {
                continue;
            };

            let row_choices = PATTERN_COORDS.map(|x| grid[(base_y + y, base_x + x)]);
            let leftovers = row_choices
                .into_iter()
                .filter(|&b| b != UNKNOWN)
                .filter(|b| !used.contains(b));

            if let Some(leftover) = only_one(leftovers) {
                if let Some(&y) = PATTERN_COORDS
                    .iter()
                    .find(|&y| grid[(base_y + *y, base_x + x)] == UNKNOWN)
                {
                    grid[(base_y + y, base_x + x)] = leftover;
                }

                grid[(base_y + y, base_x + x)] = leftover;
            }
        }
    }

    // Compute total power!
    let mut result = 0;
    let mut word = Vec::with_capacity(6 * 6);
    for base_y in (0..h).step_by(6).take(h / 6) {
        'grid_loop: for base_x in (0..w).step_by(6).take(w / 6) {
            word.clear();

            for (y, x) in iproduct!(2..6, 2..6) {
                match grid[(base_y + y, base_x + x)] {
                    EMPTY => continue 'grid_loop,
                    b => word.push(b),
                }
            }

            result += power(word.drain(..));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(include_str!("part1.txt")).to_string(), "FMRPJNBTDWKQCZLH");
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(include_str!("part2.txt")).to_string(), "194554");
    }

    #[test]
    fn test_part3() {
        assert_eq!(solve_part3(include_str!("part3.txt")).to_string(), "212032");
    }
}
