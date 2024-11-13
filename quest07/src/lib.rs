use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Add,
    Sub,
    Stay,
}

impl From<u8> for Action {
    fn from(b: u8) -> Self {
        match b {
            b'+' => Action::Add,
            b'-' => Action::Sub,
            b'=' | b'S' => Action::Stay,
            _ => unreachable!(),
        }
    }
}

impl Action {
    fn to_index(self) -> usize {
        match self {
            Action::Add => 0,
            Action::Sub => 1,
            Action::Stay => 2,
        }
    }
}

pub fn solve_part1(input: &str) -> String {
    let mut plans = input
        .lines()
        .map(|line| {
            let (name, plan) = line.split_once(':').unwrap();
            let plan = plan.bytes().step_by(2).cycle().take(10).map(Action::from);

            let mut power = 10u64;
            let mut total = 0;

            for action in plan {
                match action {
                    Action::Add => power += 1,
                    Action::Sub => power -= 1,
                    Action::Stay => (),
                }
                total += power;
            }

            (name, total)
        })
        .collect::<Vec<_>>();

    plans.sort_by_key(|(_, power)| *power);
    plans.reverse();
    plans.into_iter().map(|(name, _)| name).collect()
}

pub fn solve_part2(input: &str) -> String {
    let part2_map = linearize_map(include_str!("part2_map.txt"));
    let mut plans = input
        .lines()
        .map(|line| {
            let (name, plan) = line.split_once(':').unwrap();
            let plan = plan.bytes().step_by(2).map(Action::from).collect::<Vec<_>>();
            (name, execute_plan(&part2_map, &plan, 10))
        })
        .collect::<Vec<_>>();
    plans.sort_by_key(|(_, power)| *power);
    plans.reverse();
    plans.into_iter().map(|(name, _)| name).collect()
}

pub fn solve_part3(input: &str) -> usize {
    let part3_map = linearize_map(include_str!("part3_map.txt"));

    let enemy_plan = input
        .split_once(':')
        .unwrap()
        .1
        .bytes()
        .step_by(2)
        .map(Action::from)
        .collect::<Vec<_>>();
    let target = execute_plan(&part3_map, &enemy_plan, 2024);

    let mut possible_plans = Vec::new();
    all_possible_plans(5, 3, 3, &mut possible_plans, &mut Vec::new());

    possible_plans
        .into_par_iter()
        .filter(|plan| execute_plan(&part3_map, plan, 2024) > target)
        .count()
}

fn all_possible_plans(
    pluses_remaining: usize,
    minuses_remaining: usize,
    equals_remaining: usize,
    output: &mut Vec<Vec<Action>>,
    current: &mut Vec<Action>,
) {
    if pluses_remaining != 0 {
        current.push(Action::Add);
        all_possible_plans(
            pluses_remaining - 1,
            minuses_remaining,
            equals_remaining,
            output,
            current,
        );
        current.pop();
    }
    if minuses_remaining != 0 {
        current.push(Action::Sub);
        all_possible_plans(
            pluses_remaining,
            minuses_remaining - 1,
            equals_remaining,
            output,
            current,
        );
        current.pop();
    }
    if equals_remaining != 0 {
        current.push(Action::Stay);
        all_possible_plans(
            pluses_remaining,
            minuses_remaining,
            equals_remaining - 1,
            output,
            current,
        );
        current.pop();
    }

    if pluses_remaining == 0 && minuses_remaining == 0 && equals_remaining == 0 {
        debug_assert_eq!(current.len(), 11);
        output.push(current.clone());
    }
}

fn execute_plan(map: &[Action], plan: &[Action], n: usize) -> u64 {
    let mut power = 10u64;
    let mut total = 0u64;

    let map_len = map.len();
    let plan_len = plan.len();
    let total_iterations = n * map_len - 1; // Adjusted for skipping the first map action

    let mut map_idx = 1 % map_len; // Start from index 1 as per original logic
    let mut plan_idx = 0;

    // Precompute delta table to eliminate match statements inside the loop
    const DELTA_TABLE: [[i8; 3]; 3] = [
        // plan_action: Add, Sub, Stay
        [1, 1, 1],    // map_action: Add
        [-1, -1, -1], // map_action: Sub
        [1, -1, 0],   // map_action: Stay
    ];

    for _ in 0..total_iterations {
        let map_action = map[map_idx];
        let plan_action = plan[plan_idx];

        // Use precomputed delta table to get the power change
        let delta = DELTA_TABLE[map_action.to_index()][plan_action.to_index()] as i64;
        power = power.wrapping_add(delta as u64);
        total = total.wrapping_add(power);

        // Increment indices with wrap-around
        map_idx = if map_idx + 1 == map_len { 0 } else { map_idx + 1 };
        plan_idx = if plan_idx + 1 == plan_len { 0 } else { plan_idx + 1 };
    }

    total
}

fn linearize_map(map: &str) -> Vec<Action> {
    let map = map
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut result = Vec::with_capacity(map.len() * map[0].len());

    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;
    debug_assert_eq!(map[y][x], b'S');

    let mut saw_start = false;

    let next_cell = |x: usize, y: usize, dx: isize, dy: isize| {
        let y = y.checked_add_signed(dy)?;
        let x = x.checked_add_signed(dx)?;
        let row = map.get(y)?;
        row.get(x).copied()
    };

    loop {
        let b = map[y][x];
        if b == b'S' {
            if saw_start {
                break;
            }
            saw_start = true;
        }
        result.push(Action::from(b));

        if matches!(next_cell(x, y, dx, dy), Some(b' ') | None) {
            let (new_dx, new_dy, _) = [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .filter(|&new_dir| new_dir != (-dx, -dy))
                .filter_map(|(new_dx, new_dy)| next_cell(x, y, new_dx, new_dy).map(|c| (new_dx, new_dy, c)))
                .find(|&(_, _, c)| c != b' ')
                .unwrap();
            dx = new_dx;
            dy = new_dy;
        }

        x = x.checked_add_signed(dx).unwrap();
        y = y.checked_add_signed(dy).unwrap();
    }

    result
}
