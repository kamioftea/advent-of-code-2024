//! This is my solution for [Advent of Code - Day 6: _Guard Gallivant_](https://adventofcode.com/2024/day/6)
//!
//!

use crate::day_6::Direction::{DOWN, LEFT, RIGHT, UP};
use std::collections::HashSet;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-6-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 6.
pub fn run() {
    let _contents = fs::read_to_string("res/day-6-input.txt").expect("Failed to read file");
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Lab {
    width: usize,
    height: usize,
    obstructions: HashSet<(usize, usize)>,
}

#[derive(Eq, PartialEq, Debug)]
struct Guard {
    row: usize,
    column: usize,
    direction: Direction,
}

impl Guard {
    fn new(row: usize, column: usize, direction: Direction) -> Guard {
        Guard {
            row,
            column,
            direction,
        }
    }

    fn with_position(&self, row: usize, column: usize) -> Guard {
        Guard {
            row,
            column,
            direction: self.direction,
        }
    }

    fn with_direction(&self, direction: Direction) -> Guard {
        Guard { direction, ..*self }
    }

    fn take_step(&self, lab: &Lab) -> Option<Guard> {
        match self.next_position(lab) {
            Some((row, column)) if lab.obstructions.contains(&(row, column)) => {
                Some(self.with_direction(self.direction.turn()))
            }
            Some((row, column)) => Some(self.with_position(row, column)),
            None => None,
        }
    }

    fn next_position(&self, lab: &Lab) -> Option<(usize, usize)> {
        match self.direction {
            UP => self.row.checked_add_signed(-1).zip(Some(self.column)),
            RIGHT => {
                Some(self.column).zip(self.column.checked_add_signed(1).filter(|&c| c < lab.width))
            }
            DOWN => self
                .row
                .checked_add_signed(1)
                .filter(|&r| r < lab.height)
                .zip(Some(self.column)),
            LEFT => Some(self.column).zip(self.column.checked_add_signed(-1)),
        }
    }
}

fn parse_input(input: &String) -> (Lab, Guard) {
    let mut lines = input.lines();
    let width = lines.next().unwrap().len();
    let height = lines.count() + 1;
    let mut guard = None;
    let mut obstructions = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        for (column, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    obstructions.insert((row, column));
                }
                '^' => {
                    guard = Some(Guard {
                        row,
                        column,
                        direction: UP,
                    })
                }
                _ => (),
            }
        }
    }

    (
        Lab {
            width,
            height,
            obstructions,
        },
        guard.unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use crate::day_6::Direction::UP;
    use crate::day_6::*;
    
    fn example_lab() -> Lab {
        Lab {
            width: 10,
            height: 10,
            obstructions: vec![
                (0, 4),
                (1, 9),
                (3, 2),
                (4, 7),
                (6, 1),
                (7, 8),
                (8, 0),
                (9, 6),
            ]
            .into_iter()
            .collect(),
        }
    }

    #[test]
    fn can_parse_input() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();

        let (lab, guard) = parse_input(&input);

        assert_eq!(lab, example_lab());
        assert_eq!(guard, Guard::new(6, 4, UP));
    }

    #[test]
    fn can_take_step() {
        let lab = example_lab();

        assert_eq!(
            Guard::new(6, 4, UP).take_step(&lab),
            Some(Guard::new(5, 4, UP))
        );

        assert_eq!(
            Guard::new(1, 4, UP).take_step(&lab),
            Some(Guard::new(1, 4, RIGHT))
        );

        assert_eq!(Guard::new(0, 0, UP).take_step(&lab), None);

        assert_eq!(Guard::new(0, 0, LEFT).take_step(&lab), None);

        assert_eq!(Guard::new(9, 9, RIGHT).take_step(&lab), None);

        assert_eq!(Guard::new(9, 9, DOWN).take_step(&lab), None);
    }
}
