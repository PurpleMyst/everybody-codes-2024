const PART1_AVAILABLE_STAMPS: [usize; 4] = [1, 3, 5, 10];
const PART2_AVAILABLE_STAMPS: [usize; 10] = [1, 3, 5, 10, 15, 16, 20, 24, 25, 30];
const PART3_AVAILABLE_STAMPS: [usize; 18] = [1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101];

pub fn solve_part1(input: &str) -> usize {
    let targets = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Box<[usize]>>();

    let table = solve_for_all_targets(&PART1_AVAILABLE_STAMPS, &targets);

    targets.iter().copied().map(|target| table[target]).sum()
}

pub fn solve_part2(input: &str) -> usize {
    let targets = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Box<[usize]>>();

    let table = solve_for_all_targets(&PART2_AVAILABLE_STAMPS, &targets);

    targets.iter().copied().map(|target| table[target]).sum()
}

pub fn solve_part3(input: &str) -> usize {
    let targets = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Box<[usize]>>();

    let table = solve_for_all_targets(&PART3_AVAILABLE_STAMPS, &targets);

    targets.iter().copied().map(|target| 
        (target/2..=target/2+50)
            .map(|a| table[a] + table[target - a])
            .min().unwrap()
    ).sum()
}

// adapted from 
// https://github.com/maneatingape/everybody-codes-rust/blob/090c126651fc5c4dfde4af07e4dd304ff4928b5b/src/event2024/quest09.rs#L21
fn solve_for_all_targets(
    stamps: &[usize],
    targets: &[usize],
) -> Box<[usize]> {
    // Compute the maximum target value.
    let max_target = targets.iter().copied().max().unwrap();

    // Create a dynamic programming table for each target value.
    let mut min_stamps = vec![usize::MAX; 1+max_target].into_boxed_slice();

    // Each stamp can be used to make a target value of itself.
    for &stamp in stamps {
        min_stamps[stamp] = 1;
    }

    // Let's consider every target value from 2 to the maximum.
    for target in 2..=max_target {
        for &stamp in stamps {
            // We'll update the table for each target value, comparing the current best way to get our desired target
            // with the new way to get it using the current stamp.
            if target > stamp {
                min_stamps[target] = min_stamps[target].min(min_stamps[target - stamp].saturating_add(1));
            }
        }
    }

    min_stamps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("33\n41\n55\n99"), 10);
    }

    #[test]
    fn test_part3() {
        assert_eq!(solve_part3("156488\n352486\n546212"), 10_449)
    }
}
