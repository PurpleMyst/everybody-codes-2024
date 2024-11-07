fn solve_part1(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let words: Vec<&[u8]> = lines
        .next()
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .map(|word| word.as_bytes())
        .collect();
    lines
        .flat_map(|line| line.split_ascii_whitespace())
        .map(|haystack| {
            let hay_bytes = haystack.as_bytes();
            words
                .iter()
                .map(|&needle| {
                    word_mask(needle, hay_bytes).count_ones() as u64 / needle.len() as u64
                })
                .sum::<u64>()
        })
        .sum()
}

fn word_mask(needle: &[u8], haystack: &[u8]) -> u64 {
    debug_assert!(haystack.len() <= 64);
    let mut mask = 0u64;

    let hay_len = haystack.len();
    let needle_len = needle.len();

    if needle_len == 0 || needle_len > hay_len {
        return 0;
    }

    for i in 0..=hay_len - needle_len {
        if &haystack[i..i + needle_len] == needle {
            for j in i..i + needle_len {
                mask |= 1 << j;
            }
        }
    }

    mask
}

fn solve_part2(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let words: Vec<&[u8]> = lines
        .next()
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .map(|word| word.as_bytes())
        .collect();
    lines
        .flat_map(|line| line.split_ascii_whitespace())
        .map(|haystack| {
            let hay_bytes = haystack.as_bytes();
            let mut mask = 0u64;

            words
                .iter()
                .map(|&word| word_mask(word, hay_bytes))
                .for_each(|m| mask |= m);

            let reversed_hay_bytes: Vec<u8> = hay_bytes.iter().rev().cloned().collect();
            words
                .iter()
                .map(|&word| word_mask(word, &reversed_hay_bytes))
                .map(|m| m.reverse_bits() >> (64 - hay_bytes.len()))
                .for_each(|m| mask |= m);

            mask.count_ones() as u64
        })
        .sum()
}

fn transpose<T: Copy>(grid: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }
    let mut transposed = vec![vec![grid[0][0]; grid.len()]; grid[0].len()];
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            transposed[x][y] = cell;
        }
    }
    transposed
}

fn solve_part3(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let mut words: Vec<Vec<u8>> = lines
        .next()
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .map(|word| word.as_bytes().to_vec())
        .collect();
    words.extend(
        words
            .iter()
            .map(|word| word.iter().rev().cloned().collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>(),
    );
    words.sort();
    words.dedup();
    words.sort_by_key(|w| w.len());

    // skip empty line
    lines.next();

    let grid: Vec<Vec<u8>> = lines.map(|row| row.as_bytes().to_vec()).collect();

    let mut mask: Vec<Vec<bool>> = grid.iter().map(|row| vec![false; row.len()]).collect();

    part3_step(&words, &grid, &mut mask, true);

    // transpose the grid for the vertical search
    let grid = transpose(grid);
    let mut mask = transpose(mask);

    part3_step(&words, &grid, &mut mask, false);

    // Transpose back
    let mask = transpose(mask);

    mask.iter()
        .map(|row| row.iter().filter(|&&cell| cell).count() as u64)
        .sum()
}

fn part3_step(words: &Vec<Vec<u8>>, grid: &Vec<Vec<u8>>, mask: &mut Vec<Vec<bool>>, pacman: bool) {
    let row_len = grid[0].len();
    for word in words {
        let word_len = word.len();
        for (y, row) in grid.iter().enumerate() {
            for start in 0..row_len {
                let mut matched = true;
                for i in 0..word_len {
                    let x = if pacman {
                        (start + i) % row_len
                    } else {
                        start + i
                    };
                    if x >= row_len {
                        matched = false;
                        break;
                    }
                    if row[x] != word[i] {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    for i in 0..word_len {
                        let x = if pacman {
                            (start + i) % row_len
                        } else {
                            start + i
                        };
                        if x >= row_len {
                            break;
                        }
                        mask[y][x] = true;
                    }
                }
            }
        }
    }
}

fn main() {
    let part1 = solve_part1(include_str!("part1.txt"));
    let part2 = solve_part2(include_str!("part2.txt"));
    let part3 = solve_part3(include_str!("part3.txt"));
    println!("{}", part1);
    println!("{}", part2);
    println!("{}", part3);
    debug_assert_eq!(part3.to_string().len(), 5, "Part 3 is not 5 digits long");
    debug_assert!(
        part3.to_string().starts_with('1'),
        "Part 3 does not start with 1"
    );
    debug_assert_ne!(part3, 11_545, "Part 3 is 11_545 (known to be wrong)");
    debug_assert_ne!(part3, 11_566, "Part 3 is 11_566 (known to be wrong)");
    debug_assert_ne!(part3, 11_541, "Part 3 is 11_541 (known to be wrong)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let words = "WORDS:THE,OWE,MES,ROD,HER";
        let inputs = [
            "AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE",
            "THE FLAME SHIELDED THE HEART OF THE KINGS",
            "POWE PO WER P OWE R",
            "THERE IS THE END",
        ];
        let expected = [4, 3, 2, 3];
        for (input, expected) in inputs.iter().zip(expected.iter()) {
            assert_eq!(solve_part1(&format!("{}\n{}", words, input)), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let input = "WORDS:THE,OWE,MES,ROD,HER\n\nAWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE\nTHE FLAME SHIELDED THE HEART OF THE KINGS\nPOWE PO WER P OWE R\nTHERE IS THE END";
        assert_eq!(solve_part2(input), 37);
    }

    #[test]
    fn test_part3() {
        let input = "WORDS:THE,OWE,MES,ROD,RODEO\n\nHELWORLT\nENIGWDXL\nTRODEOAL";
        assert_eq!(solve_part3(input), 10);
    }

    #[test]
    fn test_part3_wrapping_success() {
        let input = "WORDS:ABC\n\nCAB";
        assert_eq!(solve_part3(input), 3);
    }

    #[test]
    fn test_part3_wrapping_failure() {
        let input = "WORDS:ABABAB\n\nAB";
        assert_eq!(solve_part3(input), 2);
    }
}
