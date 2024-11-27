use quest17::{solve_part12, solve_part3};

fn main() {
    let part1 = solve_part12(include_str!("part1.txt")).to_string();
    let part2 = solve_part12(include_str!("part2.txt")).to_string();
    let part3 = solve_part3(include_str!("part3.txt")).to_string();
    println!("{part1}");
    println!("{part2}");
    println!("{part3}");
}
