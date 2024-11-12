use rayon::prelude::*;

pub fn solve_part1(input: &str) -> u64 {
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
        .map(|haystack| haystack.as_bytes())
        .map(|haystack| {
            words
                .iter()
                .map(|&needle| word_mask::<false>(needle, haystack).count_ones() as u64 / needle.len() as u64)
                .sum::<u64>()
        })
        .sum()
}

fn word_mask<const REVERSE: bool>(needle: &[u8], haystack: &[u8]) -> u64 {
    debug_assert!(haystack.len() <= 64);
    let mut mask = 0u64;

    haystack
        .windows(needle.len())
        .enumerate()
        .filter(|&(_, window)| {
            if REVERSE {
                window.iter().rev().eq(needle.iter())
            } else {
                window == needle
            }
        })
        .for_each(|(i, _)| {
            for j in i..i + needle.len() {
                mask |= 1 << j;
            }
        });

    mask
}

pub fn solve_part2(input: &str) -> u64 {
    let (first, rest) = input.trim().split_once('\n').unwrap();

    let words: Vec<&[u8]> = first
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .map(|word| word.as_bytes())
        .collect();

    rest.par_split_ascii_whitespace()
        .map(|haystack| {
            let hay_bytes = haystack.as_bytes();

            let mask = words
                .iter()
                .map(|&word| word_mask::<true>(word, hay_bytes) | word_mask::<false>(word, hay_bytes))
                .fold(0u64, |a, b| a | b);

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

pub fn solve_part3(input: &str) -> u64 {
    let mut lines = input.trim().lines();

    // Load in words, handling reversal in the word list.
    let words_line = lines.next().unwrap();
    let words_iter = words_line.strip_prefix("WORDS:").unwrap().split(',');
    let mut words = Vec::new();
    for word in words_iter {
        let mut word_bytes = word.as_bytes().to_vec();
        words.push(word_bytes.clone());
        word_bytes.reverse();
        words.push(word_bytes);
    }

    // Skip empty line.
    lines.next();

    // Load grid & initialize empty mask.
    let grid: Vec<Vec<u8>> = lines.map(|row| row.as_bytes().to_vec()).collect();
    let mut mask: Vec<Vec<bool>> = grid.iter().map(|row| vec![false; row.len()]).collect();

    // Horizontal search.
    part3_step::<true>(&words, &grid, &mut mask);

    // Transpose the grid and mask for vertical search.
    let grid = transpose(grid);
    let mut mask = transpose(mask);

    // Vertical search.
    part3_step::<false>(&words, &grid, &mut mask);

    // Counting time!
    mask.iter()
        .map(|row| row.iter().filter(|&&cell| cell).count() as u64)
        .sum()
}

fn part3_step<const PACMAN: bool>(words: &[Vec<u8>], grid: &[Vec<u8>], mask: &mut [Vec<bool>]) {
    let grid_width = grid[0].len();

    let xes = |word_len, i| (0..word_len).map(move |j| (j, if PACMAN { (i + j) % grid_width } else { i + j }));

    grid.par_iter().zip(mask.par_iter_mut()).for_each(|(row, row_mask)| {
        for word_start in 0..grid_width {
            let first_char = row[word_start];
            for word in words.iter().filter(|w| w[0] == first_char) {
                let matches = xes(word.len(), word_start).all(|(j, x)| row.get(x) == Some(&word[j]));
                if matches {
                    for (_, x) in xes(word.len(), word_start) {
                        row_mask[x] = true;
                    }
                }
            }
        }
    });
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
