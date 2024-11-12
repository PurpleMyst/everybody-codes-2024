pub fn solve_part12(input: &str) -> u32 {
    let nails = input.lines().map(|line| line.parse().unwrap()).collect::<Vec<u32>>();

    let min_nail = nails.iter().min().unwrap();

    nails.iter().map(|nail| nail - min_nail).sum()
}

pub fn solve_part3(input: &str) -> u32 {
    let mut nails = input.lines().map(|line| line.parse().unwrap()).collect::<Vec<u32>>();

    // The median is the element that minimizes the sum of absolute differences, from statistics.
    nails.sort_unstable();
    let median = nails[nails.len() / 2];

    nails.iter().map(|nail| nail.abs_diff(median)).sum()
}
