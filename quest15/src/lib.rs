use std::{collections::HashMap, fmt::Display, cmp::Reverse};

use grid::Grid;
use priority_queue::PriorityQueue;

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

pub fn solve_part23(input: &str) -> impl Display {
    let map = Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    let start_x = map.iter_row(0).position(|&c| c == b'.').unwrap();
    let start = (0, start_x);

    let plant_types = map
        .iter()
        .filter(|&&c| matches!(c, b'A'..=b'Z'))
        .fold(0u32, |acc, c| acc | (1 << (c - b'A')))
        .count_ones() as usize;

    let mut q = PriorityQueue::new();
    q.push((start, 0u32), Reverse(0));

    let mut dist = HashMap::new();
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
            return d;
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

    unreachable!();
}
