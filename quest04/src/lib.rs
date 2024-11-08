pub fn solve_part12(input: &str) -> u64 {
    let nails = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u64>>();
    let min_nail = nails.iter().min().unwrap();
    nails.iter().map(|nail| nail - min_nail).sum()
}

pub fn solve_part3(input: &str) -> u64 {
    let nails = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u64>>();

    nails
        .iter()
        .map(|&target| nails.iter().map(|nail| nail.abs_diff(target)).sum())
        .min()
        .unwrap()
}
