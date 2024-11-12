use quest02::{solve_part1, solve_part2, solve_part3};

fn main() {
    let part1 = solve_part1(include_str!("part1.txt"));
    let part2 = solve_part2(include_str!("part2.txt"));
    let part3 = solve_part3(include_str!("part3.txt"));
    println!("{}", part1);
    println!("{}", part2);
    println!("{}", part3);
    debug_assert_eq!(part3.to_string().len(), 5, "Part 3 is not 5 digits long");
    debug_assert!(part3.to_string().starts_with('1'), "Part 3 does not start with 1");
    debug_assert_ne!(part3, 11_545, "Part 3 is 11_545 (known to be wrong)");
    debug_assert_ne!(part3, 11_566, "Part 3 is 11_566 (known to be wrong)");
    debug_assert_ne!(part3, 11_541, "Part 3 is 11_541 (known to be wrong)");
}
