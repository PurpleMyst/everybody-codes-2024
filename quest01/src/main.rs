use quest01::solve_part;

fn main() {
    let part1 = solve_part::<1>(include_str!("part1.txt"));
    let part2 = solve_part::<2>(include_str!("part2.txt"));
    let part3 = solve_part::<3>(include_str!("part3.txt"));
    println!("{}", part1);
    println!("{}", part2);
    println!("{}", part3);
}
