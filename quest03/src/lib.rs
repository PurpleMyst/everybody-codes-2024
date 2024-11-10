use grid::Grid;

#[inline]
pub fn solve(input: &str, include_diagonals: bool) -> u16 {
    let mut depthmap: Grid<u16> = Grid::new(0, 0);
    let mut digmap: Grid<bool> = Grid::new(0, 0);

    for row in input.lines() {
        digmap.push_row(row.bytes().map(|b| b == b'#').collect());
        depthmap.push_row(row.bytes().map(|_| 0).collect());
    }

    let height = depthmap.rows();
    let width = depthmap.cols();
    let mut next_depthmap = depthmap.clone();

    loop {
        let mut dug = false;

        for (((y, x), &depth), diggable) in depthmap.indexed_iter().zip(digmap.iter_mut()) {
            if !*diggable {
                continue;
            }

            let can_dig = if include_diagonals {
                [
                    (x != 0).then(|| depthmap[(y, x - 1)]),
                    (x != width - 1).then(|| depthmap[(y, x + 1)]),
                    (y != 0).then(|| depthmap[(y - 1, x)]),
                    (y != height - 1).then(|| depthmap[(y + 1, x)]),
                    (x != 0 && y != 0).then(|| depthmap[(y - 1, x - 1)]),
                    (x != width - 1 && y != 0).then(|| depthmap[(y - 1, x + 1)]),
                    (x != 0 && y != height - 1).then(|| depthmap[(y + 1, x - 1)]),
                    (x != width - 1 && y != height - 1).then(|| depthmap[(y + 1, x + 1)]),
                ]
                .map(|n| n.unwrap_or(0))
                .into_iter()
                .all(|h| h == depth)
            } else {
                [
                    (x != 0).then(|| depthmap[(y, x - 1)]),
                    (x != width - 1).then(|| depthmap[(y, x + 1)]),
                    (y != 0).then(|| depthmap[(y - 1, x)]),
                    (y != height - 1).then(|| depthmap[(y + 1, x)]),
                ]
                .into_iter()
                .flatten()
                .all(|h| h == depth)
            };

            if can_dig {
                next_depthmap[(y, x)] = depth + 1;
                dug = true;
            }

            *diggable = can_dig;
        }
        depthmap.clone_from(&next_depthmap);

        if !dug {
            break;
        }
    }

    depthmap.into_vec().into_iter().sum()
}
