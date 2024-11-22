use std::fmt::Display;

use priority_queue::PriorityQueue;
use rayon::prelude::*;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

fn direction(b: u8) -> (i64, i64, i64) {
    match b {
        b'U' => (0, 1, 0),
        b'D' => (0, -1, 0),
        b'L' => (-1, 0, 0),
        b'R' => (1, 0, 0),
        b'F' => (0, 0, 1),
        b'B' => (0, 0, -1),
        _ => panic!("Invalid direction"),
    }
}

pub fn solve_part1(input: &str) -> impl Display {
    let steps = input.trim().split(',').map(|step| {
        let (dir, n) = step.split_at(1);
        let dir = dir.bytes().next().unwrap();
        let n = n.parse::<u8>().unwrap();
        (dir, n)
    });

    let mut position = (0, 0, 0);
    let mut highest = 0;

    for step in steps {
        let (dx, dy, dz) = direction(step.0);
        let n = step.1 as i64;
        position = (position.0 + n * dx, position.1 + n * dy, position.2 + n * dz);
        highest = highest
            .max(position.0.abs())
            .max(position.1.abs())
            .max(position.2.abs());
    }

    highest
}

pub fn solve_part2(input: &str) -> impl Display {
    input
        .lines()
        .map(|line| {
            let steps = line.split(',').map(|step| {
                let (dir, n) = step.split_at(1);
                let dir = dir.bytes().next().unwrap();
                let n = n.parse::<u8>().unwrap();
                (dir, n)
            });

            let mut position = (0, 0, 0);
            let mut visited = HashSet::default();
            visited.insert(position);

            for step in steps {
                let (dx, dy, dz) = direction(step.0);
                let n = step.1 as i64;
                for _ in 0..n {
                    position = (position.0 + dx, position.1 + dy, position.2 + dz);
                    visited.insert(position);
                }
            }

            visited
        })
        .fold(HashSet::default(), |mut acc, x| {
            acc.extend(x);
            acc
        })
        .len()
        - 1
}

pub fn solve_part3(input: &str) -> impl Display {
    let (visited, leaves) = input
        .lines()
        .map(|line| {
            let steps = line.split(',').map(|step| {
                let (dir, n) = step.split_at(1);
                let dir = dir.bytes().next().unwrap();
                let n = n.parse::<u8>().unwrap();
                (dir, n)
            });

            let mut position = (0, 0, 0);
            let mut visited = HashSet::default();

            for step in steps {
                let (dx, dy, dz) = direction(step.0);
                let n = step.1 as i64;
                for _ in 0..n {
                    position = (position.0 + dx, position.1 + dy, position.2 + dz);
                    visited.insert(position);
                }
            }

            (visited, position)
        })
        .fold(
            (HashSet::default(), Vec::new()),
            |(mut visited, mut leaves), (new_visited, leaf)| {
                visited.extend(new_visited);
                leaves.push(leaf);
                (visited, leaves)
            },
        );

    let mut candidates = visited.clone();
    candidates.retain(|&(x, _y, z)| x == 0 && z == 0);

    candidates
        .into_par_iter()
        .map(|(x0, y0, z0)| {
            let mut distances = HashMap::default();

            let mut queue = PriorityQueue::new();
            queue.push((x0, y0, z0), std::cmp::Reverse(0));
            while let Some(((x, y, z), d)) = queue.pop() {
                if distances.contains_key(&(x, y, z)) {
                    continue;
                }
                distances.insert((x, y, z), d);

                [
                    (x - 1, y, z),
                    (x + 1, y, z),
                    (x, y - 1, z),
                    (x, y + 1, z),
                    (x, y, z - 1),
                    (x, y, z + 1),
                ]
                .into_iter()
                .filter(|p| visited.contains(&p))
                .for_each(|p| {
                    let new_d = d.0 + 1;
                    if !distances.contains_key(&p) || new_d < distances.get(&p).unwrap().0 {
                        queue.push(p, std::cmp::Reverse(new_d));
                    }
                });
            }

            let murkiness = leaves.iter().map(|&(x, y, z)| distances[&(x, y, z)].0).sum::<u64>();
            murkiness
        })
        .min()
        .unwrap()
}
