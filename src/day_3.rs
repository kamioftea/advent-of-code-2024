//! This is my solution for [Advent of Code - Day 3: _???_](https://adventofcode.com/2024/day/3)
//!
//!

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

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn extract_instructions(program: &String) -> Vec<Instruction> {
    let pattern = Regex::new(r"(mul|don't|do)\(((\d{1,3}),(\d{1,3}))?\)").unwrap();

    pattern
        .captures_iter(program)
        .map(|c| {
            let instruction = c.get(1).map_or("???", |m| m.as_str());
            match instruction {
                "mul" => Mul(match_number(&c, 3), match_number(&c, 4)),
                "do" => Do,
                "don't" => Dont,
                inst => unreachable!("Unexpected instruction '{}'", inst),
            }
        })
        .collect()
}

fn match_number(c: &Captures, idx: usize) -> u32 {
    c.get(idx).unwrap().as_str().parse().unwrap()
}

fn sum_muls(instructions: &Vec<Instruction>) -> u32 {
    instructions
        .iter()
        .map(|instruction| match instruction {
            Mul(lhs, rhs) => lhs * rhs,
            _ => 0,
        })
        .sum()
}

fn sum_instructions(instructions: &Vec<Instruction>) -> u32 {
    instructions
        .iter()
        .fold((0, true), |(sum, active), instruction| {
            match (active, instruction) {
                (true, Mul(lhs, rhs)) => (sum + lhs * rhs, true),
                (false, Mul(_, _)) => (sum, false),
                (_, Do) => (sum, true),
                (_, Dont) => (sum, false),
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use crate::day_3::Instruction::*;
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
