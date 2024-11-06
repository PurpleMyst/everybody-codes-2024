#[derive(Clone, Copy, PartialEq, Eq)]
enum Creature {
    Ant,
    Beetle,
    Cockroach,
    Dragonfly,

    Empty,
}

impl Creature {
    fn potions_needed(&self) -> usize {
        match self {
            Creature::Ant => 0,
            Creature::Beetle => 1,
            Creature::Cockroach => 3,
            Creature::Dragonfly => 5,
            Creature::Empty => 0,
        }
    }
}

impl From<u8> for Creature {
    fn from(b: u8) -> Self {
        match b {
            b'A' => Creature::Ant,
            b'B' => Creature::Beetle,
            b'C' => Creature::Cockroach,
            b'D' => Creature::Dragonfly,
            b'x' => Creature::Empty,
            _ => panic!("Invalid creature"),
        }
    }
}

fn solve_part<const N: usize>(input: &str) -> usize {
    input
        .trim()
        .as_bytes()
        .chunks_exact(N)
        .map(|chunk| <[u8; N]>::try_from(chunk).unwrap().map(Creature::from))
        .map(|group| {
            let effective_group_size = group.into_iter().filter(|&c| c != Creature::Empty).count();
            let base_potions = group.into_iter().map(|c| c.potions_needed()).sum::<usize>();
            base_potions + effective_group_size * effective_group_size.saturating_sub(1)
        })
        .sum()
}

fn main() {
    let part1 = solve_part::<1>(include_str!("part1.txt"));
    let part2 = solve_part::<2>(include_str!("part2.txt"));
    let part3 = solve_part::<3>(include_str!("part3.txt"));
    println!("{}", part1);
    println!("{}", part2);
    println!("{}", part3);
}
