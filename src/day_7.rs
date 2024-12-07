//! This is my solution for [Advent of Code - Day 7: _Bridge Repair_](https://adventofcode.com/2024/day/7)
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-7-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 7.
pub fn run() {
    let _contents = fs::read_to_string("res/day-7-input.txt").expect("Failed to read file");
}

#[derive(Eq, PartialEq, Debug)]
struct Calculation {
    target: i32,
    current: i32,
    remaining_numbers: Vec<i32>,
}

impl Calculation {
    fn new(target: i32, numbers: Vec<i32>) -> Calculation {
        Calculation {
            target,
            current: 0,
            remaining_numbers: numbers,
        }
    }
}

fn parse_calibration(line: &str) -> Calculation {
    let (target, numbers) = line.split_once(": ").unwrap();

    Calculation::new(
        target.parse().unwrap(),
        numbers.split(" ").flat_map(|num| num.parse()).collect(),
    )
}

fn parse_input(input: &String) -> Vec<Calculation> {
    input.lines().map(parse_calibration).collect()
}

#[cfg(test)]
mod tests {
    use crate::day_7::*;
    
    #[test]
    fn can_parse_input() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            .to_string();

        assert_eq!(
            parse_input(&input),
            vec![
                Calculation::new(190, vec![10, 19,]),
                Calculation::new(3267, vec![81, 40, 27]),
                Calculation::new(83, vec![17, 5,]),
                Calculation::new(156, vec![15, 6,]),
                Calculation::new(7290, vec![6, 8, 6, 15]),
                Calculation::new(161011, vec![16, 10, 13]),
                Calculation::new(192, vec![17, 8, 14]),
                Calculation::new(21037, vec![9, 7, 18, 13]),
                Calculation::new(292, vec![11, 6, 16, 20]),
            ]
        );
    }
}
