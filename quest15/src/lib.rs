use std::fmt::Display;

use grid::Grid;
use rayon::prelude::*;

use pathfinding::prelude::dijkstra;

pub fn solve_part1(input: &str) -> impl Display {
    let map = Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    let start_x = map.iter_row(0).position(|&c| c == b'.').unwrap();
    let start = (0, start_x);

    let successors = |&(y, x): &(usize, usize)| {
        let map = &map;
        let dirs = [(-1isize, 0isize), (0isize, -1isize), (0isize, 1isize), (1isize, 0isize)];
        dirs.into_iter().filter_map(move |(dy, dx)| {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny >= 0 && ny < map.rows() as isize && nx >= 0 && nx < map.cols() as isize {
                let ny = ny as usize;
                let nx = nx as usize;
                if map[(ny, nx)] != b'#' {
                    Some(((ny, nx), 1u32))
                } else {
                    None
                }
            } else {
                None
            }
        })
    };

    let goal = |&(y, x): &(usize, usize)| map[(y, x)] == b'H';

    if let Some((_path, cost)) = dijkstra(&start, successors, goal) {
        return 2 * cost;
    }

    unreachable!();
}

pub fn solve_part2(input: &str) -> impl Display {
    let map = Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    let start_x = map.iter_row(0).position(|&c| c == b'.').unwrap();
    let start = (0, start_x);

    if let Some(value) = do_solve23(&map, start) {
        return value;
    }

    unreachable!();
}

fn do_solve23(map: &Grid<u8>, start: (usize, usize)) -> Option<u32> {
    let plant_types_count = reachable_fruits(map, start);

    let start_state = (start, 0u32);

    let successors = move |&(pos, collected): &((usize, usize), u32)| {
        let (y, x) = pos;
        let dirs = [(-1isize, 0isize), (0isize, -1isize), (0isize, 1isize), (1isize, 0isize)];
        dirs.into_iter().filter_map(move |(dy, dx)| {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny >= 0 && ny < map.rows() as isize && nx >= 0 && nx < map.cols() as isize {
                let ny = ny as usize;
                let nx = nx as usize;
                let next_cell = map[(ny, nx)];
                if matches!(next_cell, b'#' | b'~') {
                    return None;
                }
                let mut next_collected = collected;
                if next_cell.is_ascii_uppercase() {
                    next_collected |= 1 << (next_cell - b'A');
                }
                Some((((ny, nx), next_collected), 1u32))
            } else {
                None
            }
        })
    };

    let goal = |&(pos, collected): &((usize, usize), u32)| -> bool {
        pos == start && collected.count_ones() == plant_types_count
    };

    if let Some((_path, cost)) = dijkstra(&start_state, successors, goal) {
        Some(cost)
    } else {
        None
    }
}

fn reachable_fruits(map: &Grid<u8>, start: (usize, usize)) -> u32 {
    let mut visited = Grid::new(map.rows(), map.cols());
    visited.fill(false);

    let mut q = vec![start];

    let mut plant_types = 0u32;

    while let Some(cur_pos) = q.pop() {
        if visited[cur_pos] {
            continue;
        }
        visited[cur_pos] = true;

        if map[cur_pos].is_ascii_uppercase() {
            plant_types |= 1 << (map[cur_pos] - b'A');
        }

        let (y, x) = cur_pos;
        let dirs = [(-1isize, 0isize), (0isize, -1isize), (0isize, 1isize), (1isize, 0isize)];
        for &(dy, dx) in &dirs {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny >= 0
                && ny < map.rows() as isize
                && nx >= 0
                && nx < map.cols() as isize
                && !matches!(map[(ny as usize, nx as usize)], b'#' | b'~')
            {
                q.push((ny as usize, nx as usize));
            }
        }
    }

    plant_types.count_ones()
}

pub fn solve_part3(input: &str) -> impl Display {
    let mut map = Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    // Compute the actual start position
    let start_x = map.iter_row(0).position(|&c| c == b'.').unwrap();
    let start = (0, start_x);

    // Find the Ks, which from inspection of the map represent the cut points between three strongly connected parts of
    // the map.
    let mut ks = Vec::with_capacity(2);
    for ((y, x), cell) in map.indexed_iter_mut() {
        if *cell == b'K' {
            *cell = b'~';
            ks.push((y, x));
        }
    }
    debug_assert_eq!(ks.len(), 2);

    // Massage the map to separate the three parts, adding fictitious fruits to the middle column to simulate
    // the path going to the two Ks.
    let mut starts = ks.clone();
    starts.insert(0, start);
    starts[1].1 -= 1;
    starts[2].1 += 1;
    map[(ks[0].0 - 1, ks[0].1)] = b'A';
    map[(ks[1].0 - 1, ks[1].1)] = b'B';

    8 + starts
        .into_par_iter()
        .map(|start| do_solve23(&map, start).unwrap())
        .sum::<u32>()
}
