use std::cmp::Reverse;

use arrayvec::ArrayVec;
use grid::Grid;
use priority_queue::PriorityQueue;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(self) -> Self {
        self.turn_left().turn_left().turn_left()
    }

    fn value(self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn add_to(self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        let (dx, dy) = self.value();
        Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct State {
    position: (usize, usize),
    direction: Direction,
    time_left: u16,
    altitude: u16,
}

fn new_altitude(altitude: u16, cell: u8) -> Option<u16> {
    match cell {
        b'#' => None,
        b'A'..=b'C' | b'S' | b'.' => Some(altitude - 1),
        b'-' => Some(altitude - 2),
        b'+' => Some(altitude + 1),
        _ => unreachable!("Invalid cell: {:?}", cell as char),
    }
}

impl State {
    fn advance(self, map: &Grid<u8>) -> ArrayVec<Self, 3> {
        let mut result = ArrayVec::new();
        if self.time_left == 0 {
            return result;
        }

        for direction in [self.direction, self.direction.turn_left(), self.direction.turn_right()] {
            if let Some(next) = self.r#move(map, direction) {
                result.push(next);
            }
        }

        result
    }

    fn r#move(self, map: &Grid<u8>, direction: Direction) -> Option<Self> {
        let position = direction.add_to(self.position)?;
        let &cell = map.get(position.0, position.1)?;
        let altitude = new_altitude(self.altitude, cell)?;
        Some(Self {
            position,
            direction,
            time_left: self.time_left - 1,
            altitude,
        })
    }

    fn key_part1(self) -> (Self, impl Ord) {
        (self, (self.altitude, Reverse(self.time_left)))
    }
}

pub fn solve_part1(input: &str) -> impl std::fmt::Display {
    let map = grid::Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    let start = map.indexed_iter().find(|(_, &cell)| cell == b'S').unwrap().0;

    let mut queue = PriorityQueue::new();
    for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
        let initial_state = State {
            position: start,
            direction,
            time_left: 100,
            altitude: 1000,
        };
        let (initial_state, initial_priority) = initial_state.key_part1();
        queue.push(initial_state, initial_priority);
    }

    let mut result = 0;

    while let Some((mut state, _)) = queue.pop() {
        // Skip if we can't reach the current best result.
        if state.altitude + (state.time_left as u16) < result {
            continue;
        }

        // Skip if we're off the beaten path.
        if state.position.0 >= 20 && map[state.position] != b'+' {
            continue;
        }

        // Short-circuit if we've reached the loop bit.
        if state.position.0 == 24 && map[state.position] == b'+' && state.direction == Direction::Down {
            state.altitude += state.time_left as u16;
            state.time_left = 0;
        }

        // Compare to current best.
        if state.altitude > result {
            result = state.altitude;
        }

        // Advance state.
        queue.extend(state.advance(&map).into_iter().map(State::key_part1));
    }

    result
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct StateWithCheckpoints {
    position: (usize, usize),
    direction: Direction,

    last_checkpoint: u8, // bitmask, ensures visitation in order
    value: i64,          // time - altitude
}

impl StateWithCheckpoints {
    fn advance(self, map: &Grid<u8>) -> ArrayVec<Self, 3> {
        let mut result = ArrayVec::new();
        if self.value < 0 {
            return result;
        }

        for direction in [self.direction, self.direction.turn_left(), self.direction.turn_right()] {
            if let Some(next) = self.r#move(map, direction) {
                result.push(next);
            }
        }

        result
    }

    fn r#move(self, map: &Grid<u8>, direction: Direction) -> Option<Self> {
        let position = direction.add_to(self.position)?;
        let &cell = map.get(position.0, position.1)?;
        if cell == b'#' {
            return None;
        }

        let got_checkpoint = match cell {
            b'A' => 1,
            b'B' => 2,
            b'C' => 3,
            _ => 0,
        };
        if got_checkpoint != 0 && self.last_checkpoint != got_checkpoint - 1 {
            return None;
        }
        let last_checkpoint = if got_checkpoint != 0 {
            got_checkpoint
        } else {
            self.last_checkpoint
        };

        let delta_altitude: i64 = match cell {
            b'A'..=b'C' | b'S' | b'.' => -1,
            b'-' => -2,
            b'+' => 1,
            _ => unreachable!("Invalid cell: {:?}", cell as char),
        };
        let delta_time = 1;

        // v0 + dv = ((t0 + dt) - (a0 + da))
        // v0 + dv = t0 - a0 + dt - da
        let value = self.value + delta_time - delta_altitude;

        Some(Self {
            position,
            direction,
            last_checkpoint,
            value,
        })
    }
}

pub fn solve_part2(input: &str) -> impl std::fmt::Display {
    let map = grid::Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    let (start, _) = map.indexed_iter().find(|(_, &cell)| cell == b'S').unwrap();

    let mut checkpoints = map
        .indexed_iter()
        .filter_map(|(position, &cell)| {
            if matches!(cell, b'A'..=b'C') {
                Some((cell, position))
            } else {
                None
            }
        })
        .collect::<ArrayVec<_, 3>>();
    checkpoints.sort_unstable_by_key(|&(cell, _)| cell);

    let initial_state = StateWithCheckpoints {
        position: start,
        direction: Direction::Down,
        last_checkpoint: 0,
        value: 10_000,
    };

    let final_value = pathfinding::directed::dijkstra::dijkstra(
        &initial_state,
        |state| {
            let prev_value = state.value;
            state
                .advance(&map)
                .into_iter()
                .map(move |state| (state, state.value - prev_value))
        },
        |state| state.position == start && state.last_checkpoint == 3,
    )
    .unwrap()
    .1;

    final_value + final_value % 2
}

pub fn solve_part3(input: &str) -> impl std::fmt::Display {
    let mut map = grid::Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    let start = map.indexed_iter().find(|(_, &cell)| cell == b'S').unwrap().0;
    map[start] = b'.';

    let mut altitude = 384400 - 2;

    let mut result = 0usize;

    loop {
        for y in 0..map.rows() {
            match map[(y, start.1 + 2)] {
                b'+' => altitude += 1,

                b'.' => altitude -= 1,

                _ => unreachable!(),
            }
            result += 1;
            if altitude == 0 {
                return result;
            }
        }
    }
}
