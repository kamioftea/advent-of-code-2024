//! This is my solution for [Advent of Code - Day 4: _Ceres Search_](https://adventofcode.com/2024/day/4)
//!
//!

use std::fs;
use std::str::FromStr;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-4-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 4.
pub fn run() {
    let _contents = fs::read_to_string("res/day-4-input.txt").expect("Failed to read file");
}

#[derive(Eq, PartialEq, Debug)]
struct Wordsearch {
    cells: Vec<Vec<char>>,
}

type CellCoords = (usize, usize);

impl Wordsearch {}

impl FromStr for Wordsearch {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s.lines().map(|l| l.chars().collect()).collect();

        Ok(Wordsearch { cells })
    }
}

#[cfg(test)]
mod tests {
    use crate::day_4::*;
    
    #[test]
    fn can_parse_input() {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

        let cells = vec![
            vec!['.', '.', 'X', '.', '.', '.'],
            vec!['.', 'S', 'A', 'M', 'X', '.'],
            vec!['.', 'A', '.', '.', 'A', '.'],
            vec!['X', 'M', 'A', 'S', '.', 'S'],
            vec!['.', 'X', '.', '.', '.', '.'],
        ];

        assert_eq!(Wordsearch::from_str(input), Ok(Wordsearch { cells }));
    }
}
