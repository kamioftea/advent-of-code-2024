//! This is my solution for [Advent of Code - Day 6: _Guard Gallivant_](https://adventofcode.com/2024/day/6)
//!
//!

use crate::day_6::Direction::*;
use std::collections::HashSet;
use std::fs;

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
        count_loops(&guard, &lab)
    )
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
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

#[derive(Eq, PartialEq, Debug, Clone)]
struct Lab {
    width: usize,
    height: usize,
    obstructions: HashSet<(usize, usize)>,
}

impl Lab {
    fn with_obstruction(&mut self, position: (usize, usize)) -> bool {
        self.obstructions.insert(position)
    }

    fn without_obstruction(&mut self, position: (usize, usize)) -> bool {
        self.obstructions.remove(&position)
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
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
                Some(self.row).zip(self.column.checked_add_signed(1).filter(|&c| c < lab.width))
            }
            DOWN => self
                .row
                .checked_add_signed(1)
                .filter(|&r| r < lab.height)
                .zip(Some(self.column)),
            LEFT => Some(self.row).zip(self.column.checked_add_signed(-1)),
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

fn count_guard_positions(guard: &Guard, lab: &Lab) -> usize {
    let visited = &mut HashSet::new();
    let mut guard = Some(guard.clone());
    while let Some(current_guard) = guard {
        visited.insert((current_guard.row, current_guard.column));
        guard = current_guard.take_step(lab);
    }

    visited.len()
}

fn will_loop(guard: &Guard, lab: &Lab) -> bool {
    let path = &mut HashSet::new();
    let mut guard = Some(guard.clone());
    while let Some(current_guard) = guard {
        if path.contains(&current_guard) {
            return true;
        }
        path.insert(current_guard);
        guard = current_guard.take_step(lab);
    }

    false
}

fn get_path(guard: &Guard, lab: &Lab) -> Vec<Guard> {
    let mut path = Vec::new();
    let mut prev_guard = Some(guard.clone());
    while let Some(current_guard) = prev_guard {
        path.push(current_guard);
        prev_guard = current_guard.take_step(lab);
    }

    path
}

fn count_loops(guard: &Guard, lab: &Lab) -> usize {
    let mut lab = lab.clone();
    let mut counter = 0;
    let mut tried = HashSet::new();

    for guard_position in get_path(guard, &lab) {
        if let Some((row, column)) = guard_position.next_position(&lab) {
            if tried.insert((row, column)) && lab.with_obstruction((row, column)) {
                if will_loop(&guard_position, &lab) {
                    counter += 1;
                }
                lab.without_obstruction((row, column));
            }
        }
    }

    counter
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
        assert_eq!(guard, Guard::new(6, 4, UP));
    }

    #[test]
    fn can_take_step() {
        let lab = example_lab();

        let examples = vec![
            (Guard::new(6, 4, UP), Some(Guard::new(5, 4, UP))),
            (Guard::new(1, 4, UP), Some(Guard::new(1, 4, RIGHT))),
            (Guard::new(1, 4, RIGHT), Some(Guard::new(1, 5, RIGHT))),
            (Guard::new(1, 8, RIGHT), Some(Guard::new(1, 8, DOWN))),
            (Guard::new(1, 8, DOWN), Some(Guard::new(2, 8, DOWN))),
            (Guard::new(6, 8, DOWN), Some(Guard::new(6, 8, LEFT))),
            (Guard::new(6, 8, LEFT), Some(Guard::new(6, 7, LEFT))),
            (Guard::new(6, 2, LEFT), Some(Guard::new(6, 2, UP))),
            (Guard::new(0, 0, UP), None),
            (Guard::new(9, 9, RIGHT), None),
            (Guard::new(9, 9, DOWN), None),
            (Guard::new(0, 0, LEFT), None),
        ];

        for (guard, expected) in examples {
            assert_eq!(guard.take_step(&lab), expected)
        }
    }

    #[test]
    fn can_count_guard_positions() {
        assert_eq!(
            count_guard_positions(&Guard::new(6, 4, UP), &example_lab()),
            41
        );
    }

    #[test]
    fn can_check_for_loop() {
        let lab = example_lab();
        let guard = Guard::new(6, 4, UP);

        assert_eq!(will_loop(&guard, &lab), false);

        let looping_positions = vec![(6, 3), (7, 6), (7, 7), (8, 1), (8, 3), (9, 7)];

        for position in looping_positions {
            let mut lab = example_lab();
            lab.with_obstruction(position);
            assert!(
                will_loop(&guard, &lab),
                "Should loop with an obstruction at {position:?}"
            )
        }
    }

    #[test]
    fn can_count_obstructions() {
        assert_eq!(count_loops(&Guard::new(6, 4, UP), &example_lab()), 6)
    }
}
