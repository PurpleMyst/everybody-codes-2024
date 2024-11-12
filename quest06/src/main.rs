use quest06::{solve, Path};

fn main() {
    let part1 = solve::<false>(include_str!("part1.txt")).join("");
    let part2 = first_letters(solve::<false>(include_str!("part2.txt")));
    let part3 = first_letters(solve::<true>(include_str!("part3.txt")));

    println!("{}", part1);
    println!("{}", part2);
    println!("{}", part3);
}

fn first_letters(path: Path) -> String {
    path.iter()
        .map(|&node| node.chars().next().unwrap())
        .collect()
}
