use grid::Grid;
use std::fmt::Display;

const GRID_SIDE: usize = 8;
const GRIDS_PER_CHUNK: usize = 15;

pub fn solve_part1(input: &str) -> impl Display {
    let grid = grid::Grid::from_vec(
        input.bytes().filter(|b| !b.is_ascii_whitespace()).collect::<Vec<_>>(),
        GRID_SIDE,
    );

    solve_grid(&grid)
}

fn solve_grid(grid: &grid::Grid<u8>) -> String {
    let mut result = String::new();

    for y in 2..6 {
        for x in 2..6 {
            let row_choices = [grid[(y, 0)], grid[(y, 1)], grid[(y, 6)], grid[(y, 7)]];
            let col_choices = [grid[(0, x)], grid[(1, x)], grid[(6, x)], grid[(7, x)]];

            let choice = row_choices.into_iter().find(|&b| col_choices.contains(&b)).unwrap();
            result.push(choice as char);
        }
    }

    result
}

fn power(s: &str) -> usize {
    s.bytes()
        .enumerate()
        .map(|(i, c)| (1 + i) * usize::from(c - b'A' + 1))
        .sum()
}

pub fn solve_part2(input: &str) -> impl Display {
    let mut lines = input.lines().map(|line| line.split(' '));
    let mut result = 0;

    let mut grids = vec![grid::Grid::new(GRID_SIDE, GRID_SIDE); GRIDS_PER_CHUNK];

    loop {
        grids.iter_mut().for_each(|grid| grid.clear());

        lines.by_ref().take(GRID_SIDE).for_each(|line| {
            grids
                .iter_mut()
                .zip(line)
                .for_each(|(grid, s)| grid.push_row(s.as_bytes().to_vec()))
        });

        grids.iter().for_each(|grid| result += power(&solve_grid(grid)));

        if lines.next().is_none() {
            break;
        }
    }

    result
}

pub fn solve_part3(input: &str) -> impl Display {
    let mut grid = Grid::new(0, 0);
    input.lines().for_each(|line| {
        let bytes = line.bytes().collect::<Vec<_>>();
        grid.push_row(bytes);
    });
    let w = grid.cols();
    let h = grid.rows();

    for _ in 0..10 {
        for base_y in (0..h).step_by(6).take(h / 6) {
            for base_x in (0..w).step_by(6).take(w / 6) {
                for y in 2..6 {
                    for x in 2..6 {
                        let row_choices = [
                            grid[(base_y + y, base_x + 0)],
                            grid[(base_y + y, base_x + 1)],
                            grid[(base_y + y, base_x + 6)],
                            grid[(base_y + y, base_x + 7)],
                        ];
                        let col_choices = [
                            grid[(base_y + 0, base_x + x)],
                            grid[(base_y + 1, base_x + x)],
                            grid[(base_y + 6, base_x + x)],
                            grid[(base_y + 7, base_x + x)],
                        ];

                        let Some(choice) = row_choices
                            .into_iter()
                            .filter(|&b| b != b'?')
                            .find(|&b| col_choices.contains(&b))
                        else {
                            continue;
                        };

                        grid[(base_y + y, base_x + x)] = choice;
                    }
                }

                for _ in 0..10 {
                    for x in 2..6 {
                        let mut empties = Vec::new();
                        let mut used = Vec::new();
                        let col_choices = [
                            grid[(base_y + 0, base_x + x)],
                            grid[(base_y + 1, base_x + x)],
                            grid[(base_y + 6, base_x + x)],
                            grid[(base_y + 7, base_x + x)],
                        ];

                        for y in 2..6 {
                            match grid[(base_y + y, base_x + x)] {
                                b'.' => empties.push(y),
                                b => used.push(b),
                            }
                        }

                        let leftovers = col_choices
                            .into_iter()
                            .filter(|&b| b != b'?')
                            .filter(|b| !used.contains(b))
                            .collect::<Vec<_>>();

                        if empties.len() == 1 && leftovers.len() == 1 {
                            let y = empties[0];
                            for x in [0, 1, 6, 7] {
                                if grid[(base_y + y, base_x + x)] == b'?' {
                                    grid[(base_y + y, base_x + x)] = leftovers[0];
                                    break;
                                }
                            }

                            grid[(base_y + y, base_x + x)] = leftovers[0];
                        }
                    }

                    for y in 2..6 {
                        let mut empties = Vec::new();
                        let mut used = Vec::new();
                        let row_choices = [
                            grid[(base_y + y, base_x + 0)],
                            grid[(base_y + y, base_x + 1)],
                            grid[(base_y + y, base_x + 6)],
                            grid[(base_y + y, base_x + 7)],
                        ];

                        for x in 2..6 {
                            match grid[(base_y + y, base_x + x)] {
                                b'.' => empties.push(x),
                                b => used.push(b),
                            }
                        }

                        let leftovers = row_choices
                            .into_iter()
                            .filter(|&b| b != b'?')
                            .filter(|b| !used.contains(b))
                            .collect::<Vec<_>>();

                        if empties.len() == 1 && leftovers.len() == 1 {
                            let x = empties[0];
                            for y in [0, 1, 6, 7] {
                                if grid[(base_y + y, base_x + x)] == b'?' {
                                    grid[(base_y + y, base_x + x)] = leftovers[0];
                                    break;
                                }
                            }

                            grid[(base_y + y, base_x + x)] = leftovers[0];
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;
    for base_y in (0..h).step_by(6).take(h / 6) {
        'foo: for base_x in (0..w).step_by(6).take(w / 6) {
            let mut word = String::new();

            for y in 2..6 {
                for x in 2..6 {
                    match grid[(base_y + y, base_x + x)] {
                        b'.' => continue 'foo,
                        b => word.push(b as char),
                    }
                }
            }

            result += power(&word);
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
