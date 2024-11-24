use std::{cmp::Reverse, fmt::Display};

use grid::Grid;
use priority_queue::PriorityQueue;
use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;

pub fn solve_part1(input: &str) -> impl Display {
    let map = Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    let start_x = map.iter_row(0).position(|&c| c == b'.').unwrap();
    let start = (0, start_x);

    let mut q = PriorityQueue::new();
    q.push(start, Reverse(0));

    let mut dist = Grid::new(map.rows(), map.cols());
    dist.fill(u32::MAX);
    dist[start] = 0;

    let neighbors = |(y, x): (usize, usize)| {
        [
            (y.checked_sub(1), Some(x)),
            (Some(y), x.checked_sub(1)),
            (Some(y), x.checked_add(1)),
            (y.checked_add(1), Some(x)),
        ]
        .into_iter()
        .filter_map(|(y, x)| y.zip(x))
        .filter_map(|(y, x)| Some(((y, x), map.get(y, x).copied()?)))
    };

    while let Some((cur_pos, _)) = q.pop() {
        let d = dist[cur_pos];

        if map[cur_pos] == b'H' {
            return 2 * d;
        }

        for (next_pos, next_cell) in neighbors(cur_pos) {
            if next_cell == b'#' {
                continue;
            }

            let new_d = d + 1;
            if new_d < dist[next_pos] {
                dist[next_pos] = new_d;
                q.push(next_pos, Reverse(new_d));
            }
        }
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
    let plant_types = reachable_fruits(map, start);

    let mut q = PriorityQueue::new();
    q.push((start, 0u32), Reverse(0));
    let mut dist = HashMap::default();
    dist.insert((start, 0), 0);
    let neighbors = |(y, x): (usize, usize)| {
        [
            (y.checked_sub(1), Some(x)),
            (Some(y), x.checked_sub(1)),
            (Some(y), x.checked_add(1)),
            (y.checked_add(1), Some(x)),
        ]
        .into_iter()
        .filter_map(|(y, x)| y.zip(x))
        .filter_map(|(y, x)| Some(((y, x), map.get(y, x).copied()?)))
    };

    while let Some(((cur_pos, cur_collected), _)) = q.pop() {
        let d = dist[&(cur_pos, cur_collected)];

        if cur_pos == start && cur_collected.count_ones() == plant_types as u32 {
            return Some(d);
        }

        for (next_pos, next_cell) in neighbors(cur_pos) {
            if matches!(next_cell, b'#' | b'~') {
                continue;
            }

            let next_collected = if next_cell.is_ascii_uppercase() {
                cur_collected | (1 << (next_cell - b'A'))
            } else {
                cur_collected
            };

            let new_d = d + 1;
            if new_d < *dist.get(&(next_pos, next_collected)).unwrap_or(&u32::MAX) {
                dist.insert((next_pos, next_collected), new_d);
                q.push((next_pos, next_collected), Reverse(new_d));
            }
        }
    }
    None
}

fn reachable_fruits(map: &Grid<u8>, start: (usize, usize)) -> u32 {
    let mut visited = Grid::new(map.rows(), map.cols());
    visited.fill(false);

    let mut q = vec![start];

    let neighbors = |(y, x): (usize, usize)| {
        [
            (y.checked_sub(1), Some(x)),
            (Some(y), x.checked_sub(1)),
            (Some(y), x.checked_add(1)),
            (y.checked_add(1), Some(x)),
        ]
        .into_iter()
        .filter_map(|(y, x)| y.zip(x))
        .filter_map(|(y, x)| Some(((y, x), map.get(y, x).copied()?)))
    };

    let mut plant_types = 0u32;
    while let Some(cur_pos) = q.pop() {
        if visited[cur_pos] {
            continue;
        }
        visited[cur_pos] = true;

        if map[cur_pos].is_ascii_uppercase() {
            plant_types |= 1 << (map[cur_pos] - b'A');
        }

        for (next_pos, next_cell) in neighbors(cur_pos) {
            if matches!(next_cell, b'#' | b'~') {
                continue;
            }

            q.push(next_pos);
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
