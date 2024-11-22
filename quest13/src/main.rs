use std::fmt::Display;

use grid::Grid;
use priority_queue::PriorityQueue;

fn main() {
    let part1 = solve_part12(include_str!("part1.txt"));
    let part2 = solve_part12(include_str!("part2.txt"));
    let part3 = solve_part3(include_str!("part3.txt"));
    println!("{part1}");
    println!("{part2}");
    println!("{part3}");
}

fn level(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'S' | b'E' => Some(0),
        b'#' => None,
        _ => unreachable!(),
    }
}

fn solve_part12(input: &str) -> impl Display {
    let map = Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );
    let (start, _) = map.indexed_iter().find(|(_, &c)| c == b'S').unwrap();

    {
        let map = &map;
        let mut q = PriorityQueue::new();
        q.push(start, std::cmp::Reverse(0));

        // dijkstra
        let mut dist = Grid::new(map.rows(), map.cols());
        dist.fill(u64::MAX);
        dist[start] = 0;

        let mut prev = Grid::new(map.rows(), map.cols());
        prev.fill((0, 0));

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
            let cur_level = level(map[cur_pos]).unwrap();
            let d = dist[cur_pos];

            if map[cur_pos] == b'E' {
                return d;
            }

            for (next_pos, next_cell) in neighbors(cur_pos) {
                if let Some(next_level) = level(next_cell) {
                    let mut height_difference = cur_level.abs_diff(next_level.into());
                    height_difference = height_difference.min(10 - height_difference);
                    let new_d = d + u64::from(height_difference) + 1;
                    if new_d < dist[next_pos] {
                        dist[next_pos] = new_d;
                        prev[next_pos] = cur_pos;
                        q.push(next_pos, std::cmp::Reverse(new_d));
                    }
                }
            }
        }

        unreachable!()
    }
}

fn solve_part3(input: &str) -> impl Display {
    let map = Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );
    let (start, _) = map.indexed_iter().find(|(_, &c)| c == b'E').unwrap();

    {
        let map = &map;
        let mut q = PriorityQueue::new();
        q.push(start, std::cmp::Reverse(0));

        // dijkstra
        let mut dist = Grid::new(map.rows(), map.cols());
        dist.fill(u64::MAX);
        dist[start] = 0;

        let mut prev = Grid::new(map.rows(), map.cols());
        prev.fill((0, 0));

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

        let mut distances = Vec::new();

        while let Some((cur_pos, _)) = q.pop() {
            let cur_level = level(map[cur_pos]).unwrap();
            let d = dist[cur_pos];

            if map[cur_pos] == b'S' {
                distances.push(d);
            }

            for (next_pos, next_cell) in neighbors(cur_pos) {
                if let Some(next_level) = level(next_cell) {
                    let mut height_difference = cur_level.abs_diff(next_level.into());
                    height_difference = height_difference.min(10 - height_difference);
                    let new_d = d + u64::from(height_difference) + 1;
                    if new_d < dist[next_pos] {
                        dist[next_pos] = new_d;
                        prev[next_pos] = cur_pos;
                        q.push(next_pos, std::cmp::Reverse(new_d));
                    }
                }
            }
        }

        distances
    }
    .into_iter()
    .min()
    .unwrap()
}
