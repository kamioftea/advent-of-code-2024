//! This is my solution for [Advent of Code - Day 6: _Guard Gallivant_](https://adventofcode.com/2024/day/6)
//!
//!

use crate::day_6::Direction::UP;
use std::collections::HashSet;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-6-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 6.
pub fn run() {
    let _contents = fs::read_to_string("res/day-6-input.txt").expect("Failed to read file");
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
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
        assert_eq!(
            guard,
            Guard {
                row: 6,
                column: 4,
                direction: UP
            }
        );
    }
}
