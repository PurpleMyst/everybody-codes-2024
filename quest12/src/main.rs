use std::fmt::Display;

use itertools::iproduct;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Catapult {
    x: usize,
}

#[derive(Debug, Clone, Copy)]
enum State {
    Up(usize),
    Right(usize),
    Down,
}

#[derive(Debug, Clone, Copy)]
struct Projectile {
    x: usize,
    y: usize,
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Segment {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy)]
struct Shot {
    segment: Segment,
    power: usize,
}

impl Catapult {
    fn shoot(
        &self,
        Shot {
            segment,
            power: shooting_power,
        }: Shot,
    ) -> impl Iterator<Item = Projectile> {
        std::iter::successors(
            Some(Projectile {
                x: self.x,
                y: match segment {
                    Segment::A => 0,
                    Segment::B => 1,
                    Segment::C => 2,
                },
                state: State::Up(shooting_power),
            }),
            move |projectile| match projectile.state {
                State::Up(n) => match n.checked_sub(1) {
                    Some(n) => Some(Projectile {
                        x: projectile.x + 1,
                        y: projectile.y + 1,
                        state: State::Up(n),
                    }),
                    None => Some(Projectile {
                        x: projectile.x + 1,
                        y: projectile.y,
                        state: State::Right(shooting_power - 1),
                    }),
                },
                State::Right(n) => match n.checked_sub(1) {
                    Some(n) => Some(Projectile {
                        x: projectile.x + 1,
                        y: projectile.y,
                        state: State::Right(n),
                    }),
                    None => Some(Projectile {
                        x: projectile.x + 1,
                        y: projectile.y - 1,
                        state: State::Down,
                    }),
                },
                State::Down => match projectile.y {
                    0 => None,
                    _ => Some(Projectile {
                        x: projectile.x + 1,
                        y: projectile.y - 1,
                        state: State::Down,
                    }),
                },
            },
        )
    }

    fn shot_segments(&self, Shot { segment, power }: Shot) -> [Line; 3] {
        let initial_y = match segment {
            Segment::A => 0,
            Segment::B => 1,
            Segment::C => 2,
        };
        // let first = Line::fortyfive(self.x as isize, match segment {
        //     Segment::A => 0,
        //     Segment::B => 1,
        //     Segment::C => 2,
        // });
        // let second = Line::horizontal(power + match segment {
        //     Segment::A => 0,
        //     Segment::B => 1,
        //     Segment::C => 2,
        // });
        // let third = Line::minus_fortyfive(
        //     , match segment {
        //     Segment::A => 0,
        //     Segment::B => 1,
        //     Segment::C => 2,
        // });
        let first = Line::fortyfive(self.x as isize, initial_y as isize);
        let second = Line::horizontal(power as isize + initial_y as isize);
        let third = Line::minus_fortyfive(self.x as isize + 2 * power as isize, power as isize + initial_y as isize);
        [first, second, third]
    }
}

impl Shot {
    fn score(&self) -> usize {
        self.power
            * match self.segment {
                Segment::A => 1,
                Segment::B => 2,
                Segment::C => 3,
            }
    }
}

fn main() {
    let part1 = solve_part12(include_str!("part1.txt"));
    let part2 = solve_part12(include_str!("part2.txt"));
    let part3 = solve_part3(include_str!("part3.txt"));
    println!("{part1}");
    println!("{part2}");
    println!("{part3}");
}

struct Line {
    m: isize,
    q: isize,
}

impl Line {
    fn horizontal(y: isize) -> Self {
        Self { m: 0, q: y }
    }

    fn fortyfive(x: isize, y: isize) -> Self {
        Self { m: 1, q: y - x }
    }

    fn minus_fortyfive(x: isize, y: isize) -> Self {
        Self { m: -1, q: y + x }
    }

    fn intersection_time(&self, other: &Self) -> Option<isize> {
        if self.m == other.m {
            if self.q == other.q {
                Some(0)
            } else {
                None
            }
        } else {
            Some((other.q - self.q) / (self.m - other.m))
        }
    }
}

fn solve_part12(input: &str) -> impl Display {
    let mut targets = Vec::new();

    let height = input.lines().count();
    let mut catapult = None;

    input
        .lines()
        .enumerate()
        .take(height - 1)
        .map(|(y, line)| (height - y - 2, line))
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| match c {
                'C' => catapult = Some(Catapult { x }),
                'T' => targets.push((x, y, 1)),
                'H' => {
                    targets.push((x, y, 2));
                }
                _ => (),
            });
        });
    let catapult = catapult.unwrap();

    let mut result = 0;
    for (target_x, target_y, multiplier) in targets {
        let delta_x = target_x as isize - catapult.x as isize;

        let shot = iproduct!([Segment::A, Segment::B, Segment::C], 1..=delta_x as usize)
            .map(|(segment, power)| Shot { segment, power })
            .find(|shot| {
                catapult
                    .shoot(*shot)
                    .find(|projectile| projectile.x == target_x && projectile.y == target_y)
                    .is_some()
            })
            .unwrap();

        result += multiplier * shot.score();
    }

    result
}

fn meteorite_hit(x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
) -> Option<usize> {
    let q = y0 as isize - x0 as isize;

    if q > y1 as isize {
        return None;
    }

    let meteor_x = y1 as isize - q;
    if meteor_x >= 0 && meteor_x <= x0 as isize && meteor_x == x1 as isize {
        Some(x0 - meteor_x as usize)
    } else {
        None
    }
}

fn solve_part3(input: &str) -> impl Display {
    let catapult = Catapult { x: 0 };

    let targets = input.lines().map(|line| {
        let (x, y) = line.split_once(' ').unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    });

    targets
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(target_x, target_y)| {
            let delta_x = target_x as isize - catapult.x as isize;

            let (shot, _hit) = iproduct!([Segment::A, Segment::B, Segment::C], 1..=delta_x as usize)
                .map(|(segment, power)| Shot { segment, power })
                .flat_map(|shot| {
                    catapult.shoot(shot).enumerate().filter(|&(t1, projectile)| {
                        if let Some(t2) = meteorite_hit(
                            target_x,
                            target_y,
                            projectile.x,
                            projectile.y
                        ) {
                            t1 <= t2
                        } else {
                            false
                        }
                    })
                    .map(move |(_, hit)| (shot, hit))
                })
                .min_by_key(|(shot, hit)| (std::cmp::Reverse(hit.y), shot.score()))
                .unwrap();

            shot.score()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part12(include_str!("part1.txt")).to_string(), "227");
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part12(include_str!("part2.txt")).to_string(), "20566");
    }

    #[test]
    fn test_part3() {
        assert_eq!(solve_part3(include_str!("part3.txt")).to_string(), "721561");
    }
}
