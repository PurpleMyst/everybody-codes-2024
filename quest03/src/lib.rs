use std::mem::swap;

#[inline]
pub fn solve(input: &str, include_diagonals: bool) -> u16 {
    let mut depthmap: Vec<Vec<u16>> = Vec::new();
    let mut diggable: Vec<Vec<bool>> = Vec::new();

    for row in input.lines() {
        diggable.push(row.bytes().map(|b| b == b'#').collect());
        depthmap.push(row.bytes().map(|_| 0).collect());
    }

    let height = depthmap.len();
    let width = depthmap[0].len();
    let mut next_depthmap = Vec::new();

    loop {
        let mut dug = false;

        next_depthmap.clone_from(&depthmap);
        for (y, row) in depthmap.iter().enumerate() {
            for (x, &current_depth) in row.iter().enumerate() {
                if !diggable[y][x] {
                    continue;
                }

                let can_dig = if include_diagonals {
                    [
                        (x != 0).then(|| depthmap[y][x - 1]),
                        (x != width - 1).then(|| depthmap[y][x + 1]),
                        (y != 0).then(|| depthmap[y - 1][x]),
                        (y != height - 1).then(|| depthmap[y + 1][x]),
                        (x != 0 && y != 0).then(|| depthmap[y - 1][x - 1]),
                        (x != width - 1 && y != 0).then(|| depthmap[y - 1][x + 1]),
                        (x != 0 && y != height - 1).then(|| depthmap[y + 1][x - 1]),
                        (x != width - 1 && y != height - 1).then(|| depthmap[y + 1][x + 1]),
                    ]
                    .map(|n| n.unwrap_or(0))
                    .into_iter()
                    .all(|h| h == current_depth)
                } else {
                    [
                        (x != 0).then(|| depthmap[y][x - 1]),
                        (x != width - 1).then(|| depthmap[y][x + 1]),
                        (y != 0).then(|| depthmap[y - 1][x]),
                        (y != height - 1).then(|| depthmap[y + 1][x]),
                    ]
                    .into_iter()
                    .flatten()
                    .all(|h| h == current_depth)
                };

                if can_dig {
                    next_depthmap[y][x] = current_depth + 1;
                    dug = true;
                }

                diggable[y][x] = can_dig;
            }
        }
        swap(&mut depthmap, &mut next_depthmap);

        if !dug {
            break;
        }
    }

    depthmap.into_iter().flatten().sum()
}
