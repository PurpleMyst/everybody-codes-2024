use rayon::prelude::*;
use std::collections::VecDeque;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn solve_part1(input: &str) -> impl std::fmt::Display {
    let map = grid::Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    let mut palms = grid::Grid::new(map.rows(), map.cols());
    palms.fill(false);
    map.indexed_iter().filter(|&(_, &c)| c == b'P').for_each(|(pos, _)| {
        palms[pos] = true;
    });
    let mut palms = palms.iter().filter(|&p| *p).count();

    let (channel_pos, _) = map
        .indexed_iter()
        .find(|&(pos, &c)| (pos.0 == 0 || pos.1 == 0) && c == b'.')
        .unwrap();

    let mut water_queue = vec![(channel_pos, 0)];
    let mut watered = grid::Grid::new(map.rows(), map.cols());

    let mut t_max = 0;

    while let Some((new_water, t)) = water_queue.pop() {
        if watered[new_water] {
            continue;
        }
        watered[new_water] = true;
        if map[new_water] == b'P' {
            palms -= 1;
            t_max = t.max(t_max);

            if palms == 0 {
                return t_max;
            }
        }

        for dir in DIRECTIONS {
            if let Some(neighbor) = new_water
                .0
                .checked_add_signed(dir.0)
                .zip(new_water.1.checked_add_signed(dir.1))
            {
                if matches!(map.get(neighbor.0, neighbor.1), Some(b'#') | None) {
                    continue;
                }

                water_queue.push((neighbor, t + 1));
            }
        }
    }

    unreachable!();
}

pub fn solve_part2(input: &str) -> impl std::fmt::Display {
    let map = grid::Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    let mut palms = grid::Grid::new(map.rows(), map.cols());
    palms.fill(false);
    map.indexed_iter().filter(|&(_, &c)| c == b'P').for_each(|(pos, _)| {
        palms[pos] = true;
    });
    let mut palms = palms.iter().filter(|&p| *p).count();

    let mut water_queue = map
        .indexed_iter()
        .filter(|&(pos, &c)| {
            (pos.0 == 0 || pos.1 == 0 || pos.0 == map.rows() - 1 || pos.1 == map.cols() - 1) && c == b'.'
        })
        .map(|(pos, _)| (pos, 0))
        .collect::<VecDeque<_>>();
    debug_assert_eq!(water_queue.len(), 2);

    let mut watered = grid::Grid::new(map.rows(), map.cols());

    while let Some((new_water, t)) = water_queue.pop_front() {
        if watered[new_water] {
            continue;
        }
        watered[new_water] = true;
        if map[new_water] == b'P' {
            palms -= 1;

            if palms == 0 {
                return t;
            }
        }

        for dir in DIRECTIONS {
            if let Some(neighbor) = new_water
                .0
                .checked_add_signed(dir.0)
                .zip(new_water.1.checked_add_signed(dir.1))
            {
                if matches!(map.get(neighbor.0, neighbor.1), Some(b'#') | None) {
                    continue;
                }

                water_queue.push_back((neighbor, t + 1));
            }
        }
    }

    unreachable!();
}

pub fn solve_part3(input: &str) -> impl std::fmt::Display {
    let map = grid::Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    let mut palms = grid::Grid::new(map.rows(), map.cols());
    palms.fill(false);
    map.indexed_iter().filter(|&(_, &c)| c == b'P').for_each(|(pos, _)| {
        palms[pos] = true;
    });
    let palms = palms.iter().filter(|&p| *p).count();

    let do_solve = |mut water_queue: VecDeque<((usize, usize), usize)>| {
        let mut palms = palms;
        let mut watered = grid::Grid::new(map.rows(), map.cols());
        let mut total = 0;

        while let Some((new_water, t)) = water_queue.pop_front() {
            if watered[new_water] {
                continue;
            }
            watered[new_water] = true;
            if map[new_water] == b'P' {
                palms -= 1;
                total += t;

                if palms == 0 {
                    return total;
                }
            }

            for dir in DIRECTIONS {
                if let Some(neighbor) = new_water
                    .0
                    .checked_add_signed(dir.0)
                    .zip(new_water.1.checked_add_signed(dir.1))
                {
                    if matches!(map.get(neighbor.0, neighbor.1), Some(b'#') | None) {
                        continue;
                    }

                    water_queue.push_back((neighbor, t + 1));
                }
            }
        }

        unreachable!();
    };

    map.indexed_iter()
        .par_bridge()
        .filter(|&(_, &c)| c == b'.')
        .map(|(pos, _)| {
            let mut water_queue = VecDeque::new();
            water_queue.push_back((pos, 0));
            do_solve(water_queue)
        })
        .min()
        .unwrap()
}
