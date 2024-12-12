//! This is my solution for [Advent of Code - Day 12: _Garden Groups_](https://adventofcode.com/2024/day/12)
//!
//!

use std::collections::HashSet;
use std::fs;

type Coordinate = (usize, usize);

struct Region {
    plots: HashSet<Coordinate>,
    perimeter: usize,
}

#[derive(Eq, PartialEq, Debug)]
struct Garden {
    plots: Vec<Vec<char>>,
}

impl Garden {
    fn get(&self, (r, c): Coordinate) -> Option<char> {
        self.plots.get(r).and_then(|row| row.get(c).copied())
    }

    fn adjacent(&self, (r, c): Coordinate) -> Vec<(Coordinate, char)> {
        [
            r.checked_sub(1).zip(Some(c)),
            Some(r).zip(c.checked_add(1)),
            r.checked_add(1).zip(Some(c)),
            Some(r).zip(c.checked_sub(1)),
        ]
        .into_iter()
        .flatten()
        .flat_map(|coord| Some(coord).zip(self.get(coord)))
        .collect()
    }
}

fn parse_input(input: &String) -> Garden {
    Garden {
        plots: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-12-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 12.
pub fn run() {
    let _contents = fs::read_to_string("res/day-12-input.txt").expect("Failed to read file");
}

#[cfg(test)]
mod tests {
    use crate::day_12::*;
    
    #[test]
    fn can_parse_input() {
        let input = "AAAA
BBCD
BBCC
EEEC"
            .to_string();

        assert_eq!(
            parse_input(&input),
            Garden {
                plots: vec![
                    vec!['A', 'A', 'A', 'A'],
                    vec!['B', 'B', 'C', 'D'],
                    vec!['B', 'B', 'C', 'C'],
                    vec!['E', 'E', 'E', 'C'],
                ]
            }
        )
    }
}
