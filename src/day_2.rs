//! This is my solution for [Advent of Code - Day 2: _Red-Nosed Reports_](https://adventofcode.com/2024/day/2)
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-2-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 2.
pub fn run() {
    let _contents = fs::read_to_string("res/day-2-input.txt").expect("Failed to read file");
}

type Report = Vec<u32>;

fn parse_line(line: &str) -> Report {
    line.split(" ").flat_map(|num| num.parse()).collect()
}

fn parse_input(input: &String) -> Vec<Report> {
    input.lines().map(parse_line).collect()
}

#[cfg(test)]
mod tests {
    use crate::day_2::*;
    
    fn sample_input() -> String {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string()
    }

    #[test]
    fn can_parse_input() {
        assert_eq!(
            parse_input(&sample_input()),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]
        )
    }
}
