//! This is my solution for [Advent of Code - Day 14: _Restroom Redoubt_](https://adventofcode.com/2024/day/14)
//!
//!

use crate::day_14::Quadrant::*;
use itertools::Itertools;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-14-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 14.
pub fn run() {
    let contents = fs::read_to_string("res/day-14-input.txt").expect("Failed to read file");
    let robots = parse_input(&contents);

    println!(
        "The total safety factor after 100 steps is {}",
        total_safety_factor(&robots, 100, &(103, 101))
    )
}

type Position = (usize, usize);
type Velocity = (isize, isize);

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity }
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn parse_part<T>(part: &str) -> (T, T)
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            let (_, values) = part.split_once("=").unwrap();
            // Coordinates are x, y
            let (c, r) = values.split_once(",").unwrap();
            (r.parse::<T>().unwrap(), c.parse::<T>().unwrap())
        }

        let (position, velocity) = line.split_once(" ").ok_or(())?;

        Ok(Robot::new(parse_part(position), parse_part(velocity)))
    }
}

impl Robot {
    fn simulate(&self, steps: usize, &(max_r, max_c): &(usize, usize)) -> Position {
        let Robot {
            position: (r, c),
            velocity: (dr, dc),
        } = *self;

        let dr = ((dr % max_r as isize) + max_r as isize) as usize;
        let dc = ((dc % max_c as isize) + max_c as isize) as usize;

        ((r + dr * steps) % max_r, (c + dc * steps) % max_c)
    }
}

fn parse_input(input: &String) -> Vec<Robot> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn simulate_robots(robots: &Vec<Robot>, steps: usize, bounds: &(usize, usize)) -> Vec<Position> {
    robots
        .iter()
        .map(|robot| robot.simulate(steps, bounds))
        .collect()
}

fn partition_position((r, c): Position, (max_r, max_c): &(usize, usize)) -> Option<Quadrant> {
    let mid_r = max_r / 2;
    let mid_c = max_c / 2;

    if r < mid_r && c < mid_c {
        Some(TopLeft)
    } else if r < mid_r && c > mid_c {
        Some(TopRight)
    } else if r > mid_r && c < mid_c {
        Some(BottomLeft)
    } else if r > mid_r && c > mid_c {
        Some(BottomRight)
    } else {
        None
    }
}

fn total_safety_factor(robots: &Vec<Robot>, steps: usize, bounds: &(usize, usize)) -> usize {
    let positions = simulate_robots(robots, steps, bounds);
    positions
        .iter()
        .flat_map(|&pos| partition_position(pos, bounds))
        .counts()
        .values()
        .product()
}

#[cfg(test)]
mod tests {
    use crate::day_14::*;
    use crate::helpers::test::assert_contains_in_any_order;
    
    fn example_robots() -> Vec<Robot> {
        vec![
            Robot::new((4, 0), (-3, 3)),
            Robot::new((3, 6), (-3, -1)),
            Robot::new((3, 10), (2, -1)),
            Robot::new((0, 2), (-1, 2)),
            Robot::new((0, 0), (3, 1)),
            Robot::new((0, 3), (-2, -2)),
            Robot::new((6, 7), (-3, -1)),
            Robot::new((0, 3), (-2, -1)),
            Robot::new((3, 9), (3, 2)),
            Robot::new((3, 7), (2, -1)),
            Robot::new((4, 2), (-3, 2)),
            Robot::new((5, 9), (-3, -3)),
        ]
    }

    fn positions_after_100_steps() -> Vec<(usize, usize)> {
        vec![
            (0, 6),
            (0, 6),
            (0, 9),
            (2, 0),
            (3, 1),
            (3, 2),
            (4, 5),
            (5, 3),
            (5, 4),
            (5, 4),
            (6, 1),
            (6, 6),
        ]
    }

    #[test]
    fn can_parse_input() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
        .to_string();

        assert_eq!(parse_input(&input), example_robots())
    }

    #[test]
    fn can_simulate_robot() {
        let robot = Robot::new((4, 2), (-3, 2));
        assert_eq!(robot.simulate(0, &(7, 11)), (4, 2));
        assert_eq!(robot.simulate(1, &(7, 11)), (1, 4));
        assert_eq!(robot.simulate(2, &(7, 11)), (5, 6));
        assert_eq!(robot.simulate(3, &(7, 11)), (2, 8));
        assert_eq!(robot.simulate(4, &(7, 11)), (6, 10));
        assert_eq!(robot.simulate(5, &(7, 11)), (3, 1));
    }

    #[test]
    fn can_simulate_robots() {
        let positions = simulate_robots(&example_robots(), 100, &(7, 11));
        assert_eq!(
            positions.iter().filter(|&&p| p == (0, 6)).count(),
            2,
            "There should be two robots at position (0,6)"
        );
        assert_eq!(
            positions.iter().filter(|&&p| p == (5, 4)).count(),
            2,
            "There should be two robots at position (5,\
        4)"
        );

        assert_contains_in_any_order(positions, positions_after_100_steps());
    }

    #[test]
    fn can_partition_robots() {
        assert_eq!(
            positions_after_100_steps()
                .into_iter()
                .map(|pos| partition_position(pos, &(7, 11)))
                .collect::<Vec<Option<Quadrant>>>(),
            vec![
                Some(TopRight),
                Some(TopRight),
                Some(TopRight),
                Some(TopLeft),
                None,
                None,
                None,
                Some(BottomLeft),
                Some(BottomLeft),
                Some(BottomLeft),
                Some(BottomLeft),
                Some(BottomRight),
            ]
        )
    }

    #[test]
    fn can_calculate_total_safety_factor() {
        assert_eq!(total_safety_factor(&example_robots(), 100, &(7, 11)), 12)
    }
}