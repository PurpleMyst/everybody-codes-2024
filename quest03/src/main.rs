use std::mem::swap;

fn do_solve(input: &str, include_diagonals: bool) -> u64 {
    let mut heightmap: Vec<Vec<usize>> = Vec::new();
    let mut diggable: Vec<Vec<bool>> = Vec::new();

    for row in input.lines() {
        diggable.push(row.bytes().map(|b| b == b'#').collect());
        heightmap.push(row.bytes().map(|_| 0).collect());
    }

    let map_height = heightmap.len();
    let map_width = heightmap[0].len();
    let mut next_heightmap = Vec::new();

    loop {
        let mut dug = false;

        next_heightmap.clone_from(&heightmap);
        for y in 0..heightmap.len() {
            for x in 0..heightmap[y].len() {
                if !diggable[y][x] {
                    continue;
                }

                let cell_height = heightmap[y][x];

                let can_dig = if !include_diagonals {
                    [
                        (x != 0).then(|| heightmap[y][x - 1]),
                        (x != map_width - 1).then(|| heightmap[y][x + 1]),
                        (y != 0).then(|| heightmap[y - 1][x]),
                        (y != map_height - 1).then(|| heightmap[y + 1][x]),
                    ]
                    .into_iter()
                    .flatten()
                    .all(|h| h == cell_height)
                } else {
                    [
                        (x != 0).then(|| heightmap[y][x - 1]).unwrap_or(0),
                        (x != map_width - 1)
                            .then(|| heightmap[y][x + 1])
                            .unwrap_or(0),
                        (y != 0).then(|| heightmap[y - 1][x]).unwrap_or(0),
                        (y != map_height - 1)
                            .then(|| heightmap[y + 1][x])
                            .unwrap_or(0),
                        (x != 0 && y != 0)
                            .then(|| heightmap[y - 1][x - 1])
                            .unwrap_or(0),
                        (x != map_width - 1 && y != 0)
                            .then(|| heightmap[y - 1][x + 1])
                            .unwrap_or(0),
                        (x != 0 && y != map_height - 1)
                            .then(|| heightmap[y + 1][x - 1])
                            .unwrap_or(0),
                        (x != map_width - 1 && y != map_height - 1)
                            .then(|| heightmap[y + 1][x + 1])
                            .unwrap_or(0),
                    ]
                    .into_iter()
                    .all(|h| h == cell_height)
                };

                if can_dig {
                    next_heightmap[y][x] = cell_height + 1;
                    dug = true;
                }
            }
        }
        swap(&mut heightmap, &mut next_heightmap);

        if !dug {
            break;
        }
    }

    heightmap.iter().flatten().map(|&h| h as u64).sum()
}

fn main() {
    let part1 = do_solve(include_str!("part1.txt"), false);
    let part2 = do_solve(include_str!("part2.txt"), false);
    let part3 = do_solve(include_str!("part3.txt"), true);
    println!("{}", part1);
    println!("{}", part2);
    println!("{}", part3);
}
