use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

type Columns = [Vec<usize>; 4];

fn concat(a: usize, b: usize) -> usize {
    let mut ten = 10;
    while ten <= b {
        ten *= 10;
    }
    a * ten + b
}

fn shout(columns: &Columns) -> usize {
    columns
        .iter()
        .map(|column| column[0])
        .reduce(|acc, n| concat(acc, n))
        .unwrap()
}

fn parse_input(input: &str) -> Columns {
    let mut columns = Columns::default();
    input
        .trim()
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse::<usize>().unwrap()))
        .for_each(|row| {
            row.zip(columns.iter_mut())
                .for_each(|(cell, column)| column.push(cell))
        });
    columns
}

fn step(columns: &mut Columns, i: usize) {
    let clapper = columns[i % 4].remove(0);
    let target_column = &mut columns[(i + 1) % 4];

    let height = target_column.len();

    let insertion_point = (0..height)
        .chain((1..=height).rev())
        .cycle()
        .nth(clapper - 1)
        .unwrap();

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
        *counters.entry(shouted).or_insert(0) += 1;
        if counters[&shouted] == 2024 {
            return (i + 1) * shouted;
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
