//! This is my solution for [Advent of Code - Day 7: _Bridge Repair_](https://adventofcode.com/2024/day/7)
//!
//!

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-7-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 7.
pub fn run() {
    let contents = fs::read_to_string("res/day-7-input.txt").expect("Failed to read file");
    let equations = parse_input(&contents);

    println!(
        "The calibration total is {}",
        calculate_calibration_total(&equations)
    )
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Equation {
    target: i64,
    total: i64,
    remaining_numbers: Vec<i64>,
}

impl Equation {
    fn new(target: i64, current: i64, remaining_numbers: Vec<i64>) -> Equation {
        Equation {
            target,
            total: current,
            remaining_numbers,
        }
    }

    fn apply<Op>(&self, operation: Op) -> Option<Equation>
    where
        Op: FnOnce(i64, i64) -> Option<i64>,
    {
        let mut remaining = self.remaining_numbers.iter();
        remaining
            .next()
            .and_then(|&next| operation(self.total, next))
            .filter(|&sum| sum <= self.target)
            .map(|sum| Equation::new(self.target, sum, remaining.cloned().collect()))
    }

    fn apply_add(&self) -> Option<Equation> {
        self.apply(|acc, next| acc.checked_add(next))
    }

    fn apply_mul(&self) -> Option<Equation> {
        self.apply(|acc, next| acc.checked_mul(next))
    }
}

impl Ord for Equation {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_remaining = self.remaining_numbers.len();
        let other_remaining = other.remaining_numbers.len();

        self_remaining
            .cmp(&other_remaining)
            .then_with(|| (self.target - self.total).cmp(&(other.target - other.total)))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Equation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_calibration(line: &str) -> Equation {
    let (target, number_list) = line.split_once(": ").unwrap();
    let mut numbers = number_list.split(" ").flat_map(|num| num.parse());

    Equation::new(
        target.parse().unwrap(),
        numbers.next().unwrap(),
        numbers.collect(),
    )
}

fn parse_input(input: &String) -> Vec<Equation> {
    input.lines().map(parse_calibration).collect()
}

fn is_solveable(equation: &Equation) -> bool {
    let mut heap: BinaryHeap<Equation> = BinaryHeap::new();
    heap.push(equation.clone());

    while let Some(curr) = heap.pop() {
        if curr.target == curr.total && curr.remaining_numbers.is_empty() {
            return true;
        }

        if let Some(added) = curr.apply_add() {
            heap.push(added);
        }

        if let Some(multiplied) = curr.apply_mul() {
            heap.push(multiplied);
        }
    }

    false
}

fn calculate_calibration_total(equations: &Vec<Equation>) -> i64 {
    equations
        .into_iter()
        .filter(|&eq| is_solveable(eq))
        .map(|eq| eq.target)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_7::*;
    use itertools::Itertools;
    
    fn example_equations() -> Vec<Equation> {
        vec![
            Equation::new(190, 10, vec![19]),
            Equation::new(3267, 81, vec![40, 27]),
            Equation::new(83, 17, vec![5]),
            Equation::new(156, 15, vec![6]),
            Equation::new(7290, 6, vec![8, 6, 15]),
            Equation::new(161011, 16, vec![10, 13]),
            Equation::new(192, 17, vec![8, 14]),
            Equation::new(21037, 9, vec![7, 18, 13]),
            Equation::new(292, 11, vec![6, 16, 20]),
        ]
    }

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

        assert_eq!(parse_input(&input), example_equations());
    }

    #[test]
    fn can_check_equation() {
        let equations = example_equations();
        let examples = equations.iter().zip(vec![
            true, true, false, false, false, false, false, false, true,
        ]);

        for (equation, expected) in examples {
            assert_eq!(is_solveable(equation), expected)
        }
    }

    #[test]
    fn can_order_equations() {
        let sorted: Vec<Equation> = example_equations().into_iter().sorted().collect();

        assert_eq!(
            sorted,
            vec![
                Equation::new(83, 17, vec![5]),
                Equation::new(156, 15, vec![6]),
                Equation::new(190, 10, vec![19]),
                Equation::new(192, 17, vec![8, 14]),
                Equation::new(3267, 81, vec![40, 27]),
                Equation::new(161011, 16, vec![10, 13]),
                Equation::new(292, 11, vec![6, 16, 20]),
                Equation::new(7290, 6, vec![8, 6, 15]),
                Equation::new(21037, 9, vec![7, 18, 13]),
            ]
        )
    }

    #[test]
    fn can_apply_ops() {
        assert_eq!(
            Equation::new(190, 10, vec![19]).apply_add(),
            Some(Equation::new(190, 29, vec![]))
        );
        assert_eq!(Equation::new(190, 190, vec![19]).apply_add(), None);
        assert_eq!(Equation::new(190, 29, vec![]).apply_add(), None);

        assert_eq!(
            Equation::new(190, 10, vec![19]).apply_mul(),
            Some(Equation::new(190, 190, vec![]))
        );
        assert_eq!(Equation::new(190, 190, vec![]).apply_mul(), None);
        assert_eq!(Equation::new(190, 10, vec![20]).apply_mul(), None);
    }

    #[test]
    fn can_calculate_calibration_total() {
        assert_eq!(calculate_calibration_total(&example_equations()), 3749)
    }
}
