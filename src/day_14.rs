//! This is my solution for [Advent of Code - Day 14: _Restroom Redoubt_](https://adventofcode.com/2024/day/14)
//!
//!

use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-14-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 14.
pub fn run() {
    let _contents = fs::read_to_string("res/day-14-input.txt").expect("Failed to read file");
}

type Position = (usize, usize);
type Velocity = (isize, isize);

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

fn parse_input(input: &String) -> Vec<Robot> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use crate::day_14::*;
    
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
}
