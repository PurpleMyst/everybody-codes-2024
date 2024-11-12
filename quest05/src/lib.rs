use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

type Columns = [Vec<u16>; 4];

fn concat(a: usize, b: u16) -> usize {
    let b = usize::from(b);
    let mut ten = 10;
    while ten <= b {
        ten *= 10;
    }
    a * ten + b
}

fn shout(columns: &Columns) -> usize {
    columns.iter().map(|column| column[0]).fold(0, |acc, n| concat(acc, n))
}

fn parse_input(input: &str) -> Columns {
    let mut columns = Columns::default();
    input
        .trim()
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse::<u16>().unwrap()))
        .for_each(|row| row.zip(columns.iter_mut()).for_each(|(cell, column)| column.push(cell)));
    columns
}

fn step(columns: &mut Columns, i: usize) {
    let clapper = columns[i % 4].remove(0);
    let target_column = &mut columns[(i + 1) % 4];
    let height = target_column.len();
    let mut insertion_point = (clapper as usize % (2 * height)).abs_diff(1);
    if insertion_point > height {
        insertion_point = 2 * height - insertion_point;
    }
    target_column.insert(insertion_point, clapper);
}

pub fn solve_part1(input: &str) -> usize {
    let mut columns = parse_input(input);
    for i in 0..10 {
        step(&mut columns, i);
    }
    shout(&columns)
}

pub fn solve_part2(input: &str) -> usize {
    let mut columns = parse_input(input);
    let mut counters = HashMap::default();
    for i in 0.. {
        step(&mut columns, i);
        let shouted = shout(&columns);
        let counter = counters.entry(shouted).or_insert(0);
        *counter += 1;
        if *counter == 2024 {
            return i + 1 * shouted;
        }
    }
    unreachable!()
}

pub fn solve_part3(input: &str) -> usize {
    let mut columns = parse_input(input);
    let mut states = HashSet::default();
    let mut answer = usize::MIN;
    for i in 0.. {
        if !states.insert(columns.clone()) {
            return answer;
        }
        step(&mut columns, i);
        let shouted = shout(&columns);
        answer = answer.max(shouted);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part3() {
        assert_eq!(solve_part3(include_str!("part3.txt")), 8641100010001000);
    }
}
