use itertools::iproduct;

type Permutation = Vec<usize>;

fn compose(perm1: &Permutation, perm2: &Permutation) -> Permutation {
    let n = perm1.len();
    let mut result = vec![0; n];
    for i in 0..n {
        result[i] = perm1[perm2[i]];
    }
    result
}

fn power(perm: &Permutation, k: usize) -> Permutation {
    let n = perm.len();
    let mut result = vec![0; n];
    let mut visited = vec![false; n];

    for i in 0..n {
        if !visited[i] {
            // Start of a new cycle
            let mut cycle = vec![];
            let mut j = i;
            while !visited[j] {
                visited[j] = true;
                cycle.push(j);
                j = perm[j];
            }
            // Compute the new positions after raising to power k
            let cycle_len = cycle.len();
            for (idx, &pos) in cycle.iter().enumerate() {
                let new_idx = (idx + k) % cycle_len;
                result[pos] = cycle[new_idx];
            }
        }
    }
    result
}

pub fn solve_part1(input: &str) -> impl std::fmt::Display {
    let (key, map) = input.split_once("\n\n").unwrap();
    let mut map = grid::Grid::from_vec(
        map.bytes().filter(|&b| b != b'\n').collect(),
        map.lines().next().unwrap().len(),
    );
    step(key, &mut map);
    extract(map.iter().copied())
}

pub fn solve_part2(input: &str) -> impl std::fmt::Display {
    let (key, map) = input.split_once("\n\n").unwrap();
    let mut map = grid::Grid::from_vec(
        map.bytes().filter(|&b| b != b'\n').collect(),
        map.lines().next().unwrap().len(),
    );
    (0..100).for_each(|_| step(key, &mut map));
    extract(map.iter().copied())
}

pub fn solve_part3(input: &str) -> impl std::fmt::Display {
    let (key, map) = input.split_once("\n\n").unwrap();
    let map = grid::Grid::from_vec(
        map.bytes().filter(|&b| b != b'\n').collect(),
        map.lines().next().unwrap().len(),
    );

    let n = map.rows() * map.cols();

    let mut transformation: Vec<usize> = (0..n).collect();
    let mut key_iter = key.bytes().cycle();

    for (y, x) in iproduct!(1..map.rows() - 1, 1..map.cols() - 1) {
        let neighbors = [
            (y - 1, x - 1),
            (y - 1, x),
            (y - 1, x + 1),
            (y, x + 1),
            (y + 1, x + 1),
            (y + 1, x),
            (y + 1, x - 1),
            (y, x - 1),
        ];

        let mut sources = neighbors;
        match key_iter.next().unwrap() {
            b'R' => sources.rotate_right(1),
            b'L' => sources.rotate_left(1),
            _ => unreachable!(),
        }

        let mut perm: Vec<usize> = (0..n).collect();
        for (&dst, &src) in neighbors.iter().zip(sources.iter()) {
            let dst_index = dst.0 * map.cols() + dst.1;
            let src_index = src.0 * map.cols() + src.1;
            perm[dst_index] = src_index;
        }

        transformation = compose(&transformation, &perm);
    }

    let iterations = 1_048_576_000;
    transformation = power(&transformation, iterations);

    let state = map.into_vec();
    let final_state = transformation.iter().map(|&i| state[i]).collect::<Vec<u8>>();

    extract(final_state.iter().copied())
}

fn step(key: &str, map: &mut grid::Grid<u8>) {
    let mut key = key.bytes().cycle();
    for y in 1..map.rows() - 1 {
        for x in 1..map.cols() - 1 {
            let neighbors = [
                (y - 1, x - 1),
                (y - 1, x),
                (y - 1, x + 1),
                (y, x + 1),
                (y + 1, x + 1),
                (y + 1, x),
                (y + 1, x - 1),
                (y, x - 1),
            ];

            let mut sources = neighbors;
            match key.next().unwrap() {
                b'R' => sources.rotate_right(1),

                b'L' => sources.rotate_left(1),

                _ => unreachable!(),
            }

            let values = sources.map(|(y, x)| map[(y, x)]);
            for (source, value) in neighbors.iter().zip(values) {
                map[*source] = value;
            }
        }
    }
}

fn extract(i: impl IntoIterator<Item = u8>) -> String {
    i.into_iter()
        .skip_while(|&c| c != b'>')
        .skip(1)
        .take_while(|&c| c != b'<')
        .map(|c| c as char)
        .collect()
}
