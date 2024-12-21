//! This is my solution for [Advent of Code - Day 21: _Keypad Conundrum_](https://adventofcode.com/2024/day/21)
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-21-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 21.
pub fn run() {
    let _contents = fs::read_to_string("res/day-21-input.txt").expect("Failed to read file");
}

#[derive(Eq, PartialEq, Debug)]
struct Code {
    chars: Vec<char>,
    value: usize,
}

fn parse_code(code: &str) -> Code {
    let chars: Vec<char> = code.chars().collect();
    let value = chars
        .iter()
        .flat_map(|c| c.to_digit(10))
        .fold(0, |acc, digit| acc * 10 + digit) as usize;

    Code { chars, value }
}

fn parse_input(input: &String) -> Vec<Code> {
    input.lines().map(parse_code).collect()
}

#[cfg(test)]
mod tests {
    use crate::day_21::*;

    fn example_codes() -> Vec<Code> {
        vec![
            Code {
                chars: vec!['0', '2', '9', 'A'],
                value: 29,
            },
            Code {
                chars: vec!['9', '8', '0', 'A'],
                value: 980,
            },
            Code {
                chars: vec!['1', '7', '9', 'A'],
                value: 179,
            },
            Code {
                chars: vec!['4', '5', '6', 'A'],
                value: 456,
            },
            Code {
                chars: vec!['3', '7', '9', 'A'],
                value: 379,
            },
        ]
    }

    #[test]
    fn can_parse_input() {
        let input = "029A
980A
179A
456A
379A"
            .to_string();

        assert_eq!(parse_input(&input), example_codes());
    }
}
