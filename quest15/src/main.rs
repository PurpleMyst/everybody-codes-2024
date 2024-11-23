use quest15::{solve_part1, solve_part23};

fn main() {
    let part1 = solve_part1(include_str!("part1.txt"));
    let part2 = solve_part23(include_str!("part2.txt"));
    let part3 = solve_part23(include_str!("part3.txt"));
    println!("{part1}");
    println!("{part2}");
    println!("{part3}");
}
