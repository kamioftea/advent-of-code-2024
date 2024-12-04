//! This is my solution for [Advent of Code - Day 3: _Mull It Over_](https://adventofcode.com/2024/day/3)
//!
//! Most of today's heavy lifting is done by the parser [`extract_instructions`]. Part one is solved by [`sum_muls`]
//! which cherry-picks all the [`Mul`] instructions. [`sum_instructions`] extends that by respecting [`Do`] and
//! [`Dont`] instructions.

use regex::{Captures, Regex};
use std::fs;
use Instruction::*;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-3-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 3.
pub fn run() {
    let contents = fs::read_to_string("res/day-3-input.txt").expect("Failed to read file");

    let instructions = extract_instructions(&contents);
    println!("Sum of mul instructions: {}", sum_muls(&instructions));
    println!(
        "Sum of all instructions: {}",
        sum_instructions(&instructions)
    );
}

/// The possible instructions that can be extracted from the input string
#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

/// Helper for parsing a specified Regex capturing group as a number
fn parse_named_group(c: &Captures, name: &str) -> u32 {
    c.name(name).unwrap().as_str().parse().unwrap()
}

/// Uses a [`Regex`] to extract specific [`Instruction`]s from the input string.
fn extract_instructions(program: &String) -> Vec<Instruction> {
    let pattern = Regex::new(
        r"(?x)        # Enable verbose mode
(?<inst>mul|don't|do) # The instructions name
\(                    # Open the arguments list
  (                   # Optionally caputure two 1-3 digit arguments
    (?<lhs>\d{1,3}),
    (?<rhs>\d{1,3})
  )?
\)                    # Finally close the arguments list",
    )
    .unwrap();

    pattern
        .captures_iter(program)
        .map(|c| {
            let instruction = c.name("inst").map(|m| m.as_str());
            match instruction {
                Some("mul") => Mul(parse_named_group(&c, "lhs"), parse_named_group(&c, "rhs")),
                Some("do") => Do,
                Some("don't") => Dont,
                inst => unreachable!("Unexpected instruction '{:?}'", inst),
            }
        })
        .collect()
}

/// Solution to part 1. Sums the results of applying all [`Mul`] instructions
fn sum_muls(instructions: &Vec<Instruction>) -> u32 {
    instructions
        .iter()
        .map(|instruction| match instruction {
            Mul(lhs, rhs) => lhs * rhs,
            _ => 0,
        })
        .sum()
}

/// Solution to part 1. Sums the results of applying only [`Mul`] instructions that are not disabled by [`Dont`]
/// instructions.
fn sum_instructions(instructions: &Vec<Instruction>) -> u32 {
    instructions
        .iter()
        .fold((0, true), |(sum, active), instruction| match instruction {
            Mul(lhs, rhs) => (sum + if active { lhs * rhs } else { 0 }, active),
            Do => (sum, true),
            Dont => (sum, false),
        })
        .0
}

#[cfg(test)]
mod tests {
    use crate::day_3::*;
    
    #[test]
    fn can_extract_muls() {
        assert_eq!(
            extract_instructions(
                &"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .to_string()
            ),
            vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)]
        )
    }

    #[test]
    fn can_extract_instructions() {
        assert_eq!(
            extract_instructions(
                &"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                    .to_string()
            ),
            vec![Mul(2, 4), Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)]
        )
    }

    #[test]
    fn can_sum_muls() {
        assert_eq!(
            sum_muls(&vec![Mul(2, 4), Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)]),
            161
        )
    }

    #[test]
    fn can_sum_instructions() {
        assert_eq!(
            sum_instructions(&vec![Mul(2, 4), Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)]),
            48
        )
    }
}
