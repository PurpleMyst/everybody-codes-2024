use quest03::solve;

fn main() {
    let part1 = solve(include_str!("part1.txt"), false);
    let part2 = solve(include_str!("part2.txt"), false);
    let part3 = solve(include_str!("part3.txt"), true);
    println!("{}", part1);
    println!("{}", part2);
    println!("{}", part3);
}
