fn step(key: &str, map: &mut grid::Grid<char>) {
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

fn main() {
    let (key, map) = include_str!("../q21.txt").split_once("\n\n").unwrap();
    let mut map = grid::Grid::from_vec(
        map.chars().filter(|&c| c != '\n').collect(),
        map.lines().next().unwrap().len(),
    );

    (0..1024).for_each(|_| step(key, &mut map));
    for y in 0..map.rows() {
        for x in 0..map.cols() {
            let ch = map[(y, x)] as char;
            print!("{}", match ch {
                '.' => ' ',
                _ => ch,
            });
        }
        println!();
    }
}
