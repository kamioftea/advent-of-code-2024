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
        calculate_calibration_total_part_1(&equations)
    );

    println!(
        "The calibration total with concatenation is {}",
        calculate_calibration_total_part_2(&equations)
    );
}

type Operation = fn(i64, i64) -> Option<i64>;

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

    fn apply(&self, operation: Operation) -> Option<Equation> {
        let mut remaining = self.remaining_numbers.iter();
        remaining
            .next()
            .and_then(|&next| operation(self.total, next))
            .filter(|&total| total <= self.target)
            .map(|sum| Equation::new(self.target, sum, remaining.cloned().collect()))
    }
}

impl Ord for Equation {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_remaining = self.remaining_numbers.len();
        let other_remaining = other.remaining_numbers.len();

        other_remaining
            .cmp(&self_remaining)
            .then_with(|| (other.target - other.total).cmp(&(self.target - self.total)))
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

fn is_solvable(equation: &Equation, ops: &Vec<Operation>) -> bool {
    let mut heap: BinaryHeap<Equation> = BinaryHeap::new();
    heap.push(equation.clone());

    while let Some(curr) = heap.pop() {
        if curr.target == curr.total && curr.remaining_numbers.is_empty() {
            return true;
        }

        ops.iter()
            .flat_map(|&op| curr.apply(op))
            .for_each(|eq| heap.push(eq))
    }

    false
}

fn calculate_calibration_total_part_1(equations: &Vec<Equation>) -> i64 {
    #[rustfmt::skip]
    let ops: Vec<Operation> = vec![
        |acc, next| acc.checked_add(next),
        |acc, next| acc.checked_mul(next)
    ];

    equations
        .into_iter()
        .filter(|&eq| is_solvable(eq, &ops))
        .map(|eq| eq.target)
        .sum()
}

fn calculate_calibration_total_part_2(equations: &Vec<Equation>) -> i64 {
    #[rustfmt::skip]
    let ops: Vec<Operation> = vec![
        |acc, next| acc.checked_add(next),
        |acc, next| acc.checked_mul(next),
        |acc, next| format!("{acc}{next}").parse().ok(),
    ];

    equations
        .into_iter()
        .filter(|&eq| is_solvable(eq, &ops))
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
    fn can_check_equation_par1_1() {
        let equations = example_equations();
        let examples = equations.iter().zip(vec![
            true, true, false, false, false, false, false, false, true,
        ]);

        #[rustfmt::skip]
        let ops: Vec<Operation> = vec![
            |acc, next| acc.checked_add(next),
            |acc, next| acc.checked_mul(next)
        ];

        for (equation, expected) in examples {
            assert_eq!(is_solvable(equation, &ops), expected)
        }
    }

    #[test]
    fn can_check_equation_part_2() {
        let equations = example_equations();
        let examples = equations.iter().zip(vec![
            true, true, false, true, true, false, true, false, true,
        ]);

        #[rustfmt::skip]
        let ops: Vec<Operation> = vec![
            |acc, next| acc.checked_add(next),
            |acc, next| acc.checked_mul(next),
            |acc, next| format!("{acc}{next}").parse().ok()
        ];

        for (equation, expected) in examples {
            assert_eq!(
                is_solvable(equation, &ops),
                expected,
                "Expected {equation:?} to be {expected}"
            )
        }
    }

    #[test]
    fn can_order_equations() {
        assert_eq!(
            Equation::new(190, 29, vec![]).cmp(&Equation::new(190, 19, vec![10])),
            Ordering::Greater
        );
        assert_eq!(
            Equation::new(190, 190, vec![]).cmp(&Equation::new(190, 29, vec![])),
            Ordering::Greater
        );

        let sorted: Vec<Equation> = example_equations().into_iter().sorted().collect();

        assert_eq!(
            sorted,
            vec![
                Equation::new(21037, 9, vec![7, 18, 13]),
                Equation::new(7290, 6, vec![8, 6, 15]),
                Equation::new(292, 11, vec![6, 16, 20]),
                Equation::new(161011, 16, vec![10, 13]),
                Equation::new(3267, 81, vec![40, 27]),
                Equation::new(192, 17, vec![8, 14]),
                Equation::new(190, 10, vec![19]),
                Equation::new(156, 15, vec![6]),
                Equation::new(83, 17, vec![5]),
            ]
        )
    }

    #[test]
    fn can_apply_add() {
        let add: Operation = |acc, next| acc.checked_add(next);

        assert_eq!(
            Equation::new(190, 10, vec![19]).apply(add),
            Some(Equation::new(190, 29, vec![]))
        );
        assert_eq!(Equation::new(190, 190, vec![19]).apply(add), None);
        assert_eq!(Equation::new(190, 29, vec![]).apply(add), None);
    }

    #[test]
    fn can_apply_mul() {
        let mul: Operation = |acc, next| acc.checked_mul(next);

        assert_eq!(
            Equation::new(190, 10, vec![19]).apply(mul),
            Some(Equation::new(190, 190, vec![]))
        );
        assert_eq!(Equation::new(190, 190, vec![]).apply(mul), None);
        assert_eq!(Equation::new(190, 10, vec![20]).apply(mul), None);
    }

    #[test]
    fn can_apply_concat() {
        let concat: Operation = |acc, next| format!("{acc}{next}").parse().ok();

        assert_eq!(
            Equation::new(1090, 10, vec![19]).apply(concat),
            Some(Equation::new(1090, 1019, vec![]))
        );
        assert_eq!(Equation::new(190, 190, vec![]).apply(concat), None);
        assert_eq!(Equation::new(190, 10, vec![19]).apply(concat), None);
    }

    #[test]
    fn can_calculate_calibration_total() {
        assert_eq!(
            calculate_calibration_total_part_1(&example_equations()),
            3749
        )
    }

    #[test]
    fn can_calculate_calibration_total_part_2() {
        assert_eq!(
            calculate_calibration_total_part_2(&example_equations()),
            11387
        )
    }
}