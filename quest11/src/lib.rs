use std::fmt::Display;

use nalgebra::{DMatrix, SMatrix};
use rayon::prelude::*;

fn load_simple_matrix(input: &str) -> SMatrix<u64, 26, 26> {
    let mut m = SMatrix::from_element(0);
    input.lines().for_each(|line| {
        let (lhs, rhs) = line.split_once(':').unwrap();
        let converts_to = rhs.trim().split(',').map(|s| s.trim().bytes().next().unwrap() - b'A');
        let lhs = lhs.bytes().next().unwrap() - b'A';
        for n in converts_to {
            m[(n as usize, lhs as usize)] += 1;
        }
    });
    m
}

pub fn solve_part1(input: &str) -> impl Display {
    let mut m = load_simple_matrix(input);
    let mut initial_state = SMatrix::<u64, 26, 1>::from_element(0);
    initial_state[(0, 0)] = 1;
    m.pow_mut(4);
    let final_state = m * initial_state;
    final_state.sum()
}

pub fn solve_part2(input: &str) -> impl Display {
    let mut m = load_simple_matrix(input);
    let mut initial_state = SMatrix::<u64, 26, 1>::from_element(0u64);
    initial_state[(25, 0)] = 1;
    m.pow_mut(10);
    let final_state = m * initial_state;
    final_state.sum()
}

pub fn solve_part3(input: &str) -> impl Display {
    let side = input.lines().count();
    let indices = input
        .lines()
        .map(|line| line.split_once(':').unwrap().0)
        .collect::<Vec<_>>();

    let mut m = DMatrix::from_element(side, side, 0);
    input.lines().for_each(|line| {
        let (lhs, rhs) = line.split_once(':').unwrap();
        let converts_to = rhs
            .trim()
            .split(',')
            .map(|s| indices.iter().position(|&i| i == s.trim()).unwrap());
        let lhs = indices.iter().position(|&i| i == lhs).unwrap();
        for n in converts_to {
            m[(n as usize, lhs as usize)] += 1;
        }
    });

    m.pow_mut(20);

    let (max, min) = (0..side)
        .into_par_iter()
        .map(|k| {
            let mut initial_state = DMatrix::from_element(side, 1, 0);
            initial_state[(k, 0)] = 1;
            let state = &m * initial_state;
            state.sum()
        })
        .fold(|| (0, u64::MAX), |(max, min), x| (max.max(x), min.min(x)))
        .reduce(
            || (0, u64::MAX),
            |(max1, min1), (max2, min2)| (max1.max(max2), min1.min(min2)),
        );

    max - min
}
