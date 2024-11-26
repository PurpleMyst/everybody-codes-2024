use rustc_hash::FxHashSet as HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Wheels<const WHEELS: usize> {
    wheels: [Vec<[u8; 3]>; WHEELS],
    positions: [usize; WHEELS],
    advancements: [usize; WHEELS],
}

impl<const WHEELS: usize> Wheels<WHEELS> {
    fn new(input: &str) -> Self {
        let (instructions, wheel_markings) = input.split_once("\n\n").unwrap();

        let advancements = instructions
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();

        let mut wheels = std::array::from_fn(|_| Vec::new());
        let positions = [0usize; WHEELS];

        wheel_markings.lines().for_each(|line| {
            let mut bs = line.bytes();

            for wheel in wheels.iter_mut() {
                if let Ok(face) = bs.by_ref().take(3).collect::<Vec<u8>>().try_into() {
                    if &face != b"   " {
                        wheel.push(face);
                    }
                }

                let _ = bs.next();
            }
        });

        Self {
            wheels,
            positions,
            advancements,
        }
    }

    fn advance(&mut self) {
        self.wheels
            .iter()
            .zip(self.positions.iter_mut())
            .zip(self.advancements.iter())
            .for_each(|((wheel, position), advancement)| {
                *position = (*position + advancement) % wheel.len();
            });
    }

    fn move_once_upwards(&mut self) {
        self.wheels
            .iter()
            .zip(self.positions.iter_mut())
            .for_each(|(wheel, position)| {
                *position = (*position + 1) % wheel.len();
            });
    }

    fn move_once_downwards(&mut self) {
        self.wheels
            .iter()
            .zip(self.positions.iter_mut())
            .for_each(|(wheel, position)| {
                *position = (*position + wheel.len() - 1) % wheel.len();
            });
    }

    fn faces(&self) -> [[u8; 3]; WHEELS] {
        std::array::from_fn(|i| self.wheels[i][self.positions[i]])
    }

    fn coins_awarded_part2(&self) -> usize {
        let mut counts = [0; 256];

        self.faces()
            .iter()
            .flat_map(|face| [face[0], face[2]])
            .for_each(|face| counts[face as usize] += 1);

        counts.iter().filter(|&&count| count >= 3).map(|&count| count - 2).sum()
    }
}

pub fn solve_part1(input: &str) -> impl std::fmt::Display {
    let mut wheels = Wheels::<4>::new(input);

    for _ in 0..100 {
        wheels.advance();
    }

    let faces = wheels.faces();

    faces
        .iter()
        .map(|face| std::str::from_utf8(face).unwrap())
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn solve_part2(input: &str) -> impl std::fmt::Display {
    const ITERATIONS: usize = 202420242024;
    let mut wheels = Wheels::<10>::new(input);
    let mut total = 0;

    let mut cycle_len = 0;

    loop {
        wheels.advance();

        total += wheels.coins_awarded_part2();
        cycle_len += 1;

        if wheels.positions.iter().all(|&pos| pos == 0) {
            break;
        }
    }

    total = ITERATIONS / cycle_len * total;

    for _ in 0..ITERATIONS % cycle_len {
        wheels.advance();
        total += wheels.coins_awarded_part2();
    }

    total
}

pub fn solve_part3(input: &str) -> impl std::fmt::Display {
    const ITERATIONS: usize = 256;
    let mut initial_wheels = Wheels::<5>::new(input);

    let mut states = HashSet::default();
    states.insert((initial_wheels.positions, 0usize));

    let mut new_states = HashSet::default();

    for _ in 0..ITERATIONS {
        states.drain().for_each(|(positions, coins)| {
            let mut add_state =
                |state: &Wheels<5>| new_states.insert((state.positions, coins + state.coins_awarded_part2()));

            initial_wheels.positions = positions;
            initial_wheels.advance();
            add_state(&initial_wheels);

            initial_wheels.positions = positions;
            initial_wheels.move_once_upwards();
            initial_wheels.advance();
            add_state(&initial_wheels);

            initial_wheels.positions = positions;
            initial_wheels.move_once_downwards();
            initial_wheels.advance();
            add_state(&initial_wheels);
        });
        std::mem::swap(&mut states, &mut new_states);
    }

    let mut min_coins = usize::MAX;
    let mut max_coins = 0;
    for (_positions, coins) in states {
        min_coins = min_coins.min(coins);
        max_coins = max_coins.max(coins);
    }

    format!("{} {}", max_coins, min_coins)
}
