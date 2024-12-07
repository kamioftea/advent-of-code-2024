//! This is my solution for [Advent of Code - Day 6: _Guard Gallivant_](https://adventofcode.com/2024/day/6)
//!
//! [`parse_input`] captures a representation of the [`Lab`] and [`Guard`]. [`Guard::take_step`] is the key function
//! for moving a guard, delegating to a bunch of helper functions in the same `impl`.
//!
//![`count_guard_positions`] is the solution to part one, using [`route_iter`] to generate the sequence of positions
//! visited
//!
//! [`count_obstructions_causing_loops`] is the solution to part 2, using [`is_loop`] along with reusing some of part 1.

use crate::day_6::Direction::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::iter::successors;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-6-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 6.
pub fn run() {
    let contents = fs::read_to_string("res/day-6-input.txt").expect("Failed to read file");
    let (lab, guard) = parse_input(&contents);

    println!(
        "The guard visits {} positions",
        count_guard_positions(&guard, &lab)
    );

    println!(
        "There are {} positions where obstructions will cause a loop",
        count_obstructions_causing_loops(&guard, &lab)
    )
}

/// The direction the guard is facing
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    /// new Direction after a 90-degree turn
    fn turn(&self) -> Direction {
        match self {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
        }
    }
}

type Position = (usize, usize);

/// Represent a lab by its dimensions and a set of Positions with obstructions
#[derive(Eq, PartialEq, Debug, Clone)]
struct Lab {
    width: usize,
    height: usize,
    obstructions: HashSet<Position>,
}

impl Lab {
    /// Add an obstruction to the lab, returns false if there was already an obstruction in that Position
    fn with_obstruction(&self, position: Position) -> Lab {
        let mut obstructions = self.obstructions.clone();
        obstructions.insert(position);

        Lab {
            obstructions,
            ..self.clone()
        }
    }
}

/// A Guard represented by their position and facing
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    /// Basic constructor
    fn new(position: Position, direction: Direction) -> Guard {
        Guard {
            position,
            direction,
        }
    }

    /// Step up or down row(s) - None if the new row is outside the lab
    fn step_row(&self, delta: isize, &Lab { height, .. }: &Lab) -> Option<Position> {
        let (row, column) = self.position;
        let new_row = row.checked_add_signed(delta).filter(|&c| c < height);
        new_row.zip(Some(column.clone()))
    }

    /// Step across column(s) - None if the new column is outside the lab
    fn step_column(&self, delta: isize, &Lab { width, .. }: &Lab) -> Option<Position> {
        let (row, column) = self.position;
        let new_column = column.checked_add_signed(delta).filter(|&c| c < width);
        Some(row.clone()).zip(new_column)
    }

    /// Given the current facing, get the next position in that direction
    fn next_position(&self, lab: &Lab) -> Option<(usize, usize)> {
        match self.direction {
            UP => self.step_row(-1, lab),
            RIGHT => self.step_column(1, lab),
            DOWN => self.step_row(1, lab),
            LEFT => self.step_column(-1, lab),
        }
    }

    /// A copy of the guard in a new position
    fn with_position(&self, position: Position) -> Guard {
        Guard {
            position,
            direction: self.direction,
        }
    }

    /// A copy of the guard with a new facing
    fn with_direction(&self, direction: Direction) -> Guard {
        Guard { direction, ..*self }
    }

    /// Attempt to take a step forward, turning if obstructed. Returns None if the step takes the guard out of the lab
    fn take_step(&self, lab: &Lab) -> Option<Guard> {
        match self.next_position(lab) {
            Some(position) if lab.obstructions.contains(&position) => {
                Some(self.with_direction(self.direction.turn()))
            }
            Some(position) => Some(self.with_position(position)),
            None => None,
        }
    }
}

/// Walk the input, building a set of obstructions and identifying the guard's position
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
                    guard = Some(Guard::new((row, column), UP));
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

/// Return the list of positions and facings a guard follows until they leave the lab
fn route_iter<'a>(guard: &'a Guard, lab: &'a Lab) -> impl Iterator<Item = Guard> + 'a {
    successors(Some(guard.clone()), |g| g.take_step(lab))
}

/// Count the unique positions visited by the guard before she leaves the lab
fn count_guard_positions(guard: &Guard, lab: &Lab) -> usize {
    route_iter(guard, &lab).map(|g| g.position).unique().count()
}

/// Will the guard end up in an infinite loop for the provided lab and starting position
fn is_loop(guard: &Guard, lab: &Lab) -> bool {
    route_iter(&guard, &lab).duplicates().next().is_some()
}

/// Try adding obstacles to all locations on the guard's route, and see which ones cause the guard to end up in an
/// infinite loop
fn count_obstructions_causing_loops(guard: &Guard, lab: &Lab) -> usize {
    route_iter(guard, lab)
        .flat_map(|g| Some(g).zip(g.next_position(lab)))
        .filter(|(_, pos)| *pos != guard.position)
        .unique_by(|(_, pos)| *pos)
        .par_bridge()
        .filter(|(g, pos)| is_loop(g, &lab.with_obstruction(*pos)))
        .count()
}

#[cfg(test)]
mod tests {
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
        assert_eq!(guard, Guard::new((6, 4), UP));
    }

    #[test]
    fn can_take_step() {
        let lab = example_lab();

        let examples = vec![
            (Guard::new((6, 4), UP), Some(Guard::new((5, 4), UP))),
            (Guard::new((1, 4), UP), Some(Guard::new((1, 4), RIGHT))),
            (Guard::new((1, 4), RIGHT), Some(Guard::new((1, 5), RIGHT))),
            (Guard::new((1, 8), RIGHT), Some(Guard::new((1, 8), DOWN))),
            (Guard::new((1, 8), DOWN), Some(Guard::new((2, 8), DOWN))),
            (Guard::new((6, 8), DOWN), Some(Guard::new((6, 8), LEFT))),
            (Guard::new((6, 8), LEFT), Some(Guard::new((6, 7), LEFT))),
            (Guard::new((6, 2), LEFT), Some(Guard::new((6, 2), UP))),
            (Guard::new((0, 0), UP), None),
            (Guard::new((9, 9), RIGHT), None),
            (Guard::new((9, 9), DOWN), None),
            (Guard::new((0, 0), LEFT), None),
        ];

        for (guard, expected) in examples {
            assert_eq!(guard.take_step(&lab), expected)
        }
    }

    #[test]
    fn can_count_guard_positions() {
        assert_eq!(
            count_guard_positions(&Guard::new((6, 4), UP), &example_lab()),
            41
        );
    }

    #[test]
    fn can_check_if_route_loops() {
        let lab = example_lab();
        let guard = Guard::new((6, 4), UP);

        assert_eq!(is_loop(&guard, &lab), false);

        let looping_positions = vec![(6, 3), (7, 6), (7, 7), (8, 1), (8, 3), (9, 7)];

        for position in looping_positions {
            assert!(
                is_loop(&guard, &example_lab().with_obstruction(position)),
                "Should loop with an obstruction at {position:?}"
            )
        }
    }

    #[test]
    fn can_count_obstructions() {
        assert_eq!(
            count_obstructions_causing_loops(&Guard::new((6, 4), UP), &example_lab()),
            6
        )
    }
}
