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

fn word_mask_reverse(needle: &[u8], haystack: &[u8]) -> u64 {
    debug_assert!(haystack.len() <= 64);
    let mut mask = 0u64;

    let hay_len = haystack.len();
    let needle_len = needle.len();

    if needle_len == 0 || needle_len > hay_len {
        return 0;
    }

    for i in 0..=hay_len - needle_len {
        if haystack[i..i + needle_len].iter().rev().eq(needle.iter()) {
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

            for &word in &words {
                mask |= word_mask(word, hay_bytes);
                mask |= word_mask_reverse(word, hay_bytes);
            }

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
    let words_line = lines.next().unwrap();
    let words_iter = words_line.strip_prefix("WORDS:").unwrap().split(',');

    let mut words = Vec::new();
    for word in words_iter {
        let word_bytes = word.as_bytes().to_vec();
        words.push(word_bytes.clone());
        let reversed_word: Vec<u8> = word_bytes.iter().rev().cloned().collect();
        words.push(reversed_word);
    }

    // Skip empty line
    lines.next();

    let grid: Vec<Vec<u8>> = lines.map(|row| row.as_bytes().to_vec()).collect();
    let mut mask: Vec<Vec<bool>> = grid.iter().map(|row| vec![false; row.len()]).collect();

    part3_step(&words, &grid, &mut mask, true);

    // Transpose the grid and mask for vertical search
    let grid_t = transpose(grid);
    let mut mask_t = transpose(mask);

    part3_step(&words, &grid_t, &mut mask_t, false);

    // Transpose back the mask
    let mask = transpose(mask_t);

    mask.iter()
        .map(|row| row.iter().filter(|&&cell| cell).count() as u64)
        .sum()
}

fn part3_step(words: &[Vec<u8>], grid: &[Vec<u8>], mask: &mut [Vec<bool>], pacman: bool) {
    for (y, row) in grid.iter().enumerate() {
        let row_len = row.len();
        for i in 0..row_len {
            let c = row[i];
            for word in words.iter().filter(|w| w[0] == c) {
                let word_len = word.len();
                let mut matched = true;
                for j in 0..word_len {
                    let x = if pacman { (i + j) % row_len } else { i + j };
                    if x >= row_len || row[x] != word[j] {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    for j in 0..word_len {
                        let x = if pacman { (i + j) % row_len } else { i + j };
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
