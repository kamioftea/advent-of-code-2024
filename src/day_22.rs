//! This is my solution for [Advent of Code - Day 22: _Monkey Market_](https://adventofcode.com/2024/day/22)
//!
//!

use itertools::{iterate, Itertools};
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-22-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 22.
pub fn run() {
    let contents = fs::read_to_string("res/day-22-input.txt").expect("Failed to read file");
    let seeds = parse_input(&contents);

    println!(
        "After 2000 secret numbers are generated the sum is {}",
        iterate_and_sum(seeds)
    );
}

trait NumberExtensions {
    fn mix(&self, prev: &Self) -> Self;
    fn prune(&self) -> Self;
    fn next_secret(&self) -> Self;
}

impl NumberExtensions for u64 {
    fn mix(&self, prev: &Self) -> Self {
        self ^ prev
    }

    fn prune(&self) -> Self {
        self % 16777216
    }

    fn next_secret(&self) -> Self {
        let step_1 = (self * 64).mix(self).prune();
        let step_2 = (step_1 / 32).mix(&step_1).prune();
        (step_2 * 2048).mix(&step_2).prune()
    }
}

fn pseudorandom(seed: u64) -> impl Iterator<Item = u64> {
    iterate(seed, |prev| prev.next_secret())
}

fn iterate_and_sum(seeds: Vec<u64>) -> u64 {
    seeds
        .iter()
        .map(|seed| pseudorandom(*seed).nth(2000).unwrap())
        .sum()
}

fn parse_input(input: &String) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use crate::day_22::*;

    #[test]
    fn can_parse_input() {
        let input = "1
10
100
2024
"
        .to_string();
        assert_eq!(parse_input(&input), vec![1, 10, 100, 2024])
    }

    #[test]
    fn can_mix_numbers() {
        assert_eq!(42.mix(&15), 37)
    }

    #[test]
    fn can_prune_numbers() {
        assert_eq!(100000000.prune(), 16113920)
    }

    #[test]
    fn can_iterate_secret_number() {
        assert_eq!(
            pseudorandom(123).dropping(1).take(10).collect::<Vec<u64>>(),
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254
            ]
        );

        assert_eq!(pseudorandom(1).nth(2000), Some(8685429));
        assert_eq!(pseudorandom(10).nth(2000), Some(4700978));
        assert_eq!(pseudorandom(100).nth(2000), Some(15273692));
        assert_eq!(pseudorandom(2024).nth(2000), Some(8667524));
    }

    #[test]
    fn can_iterate_and_sum_list() {
        assert_eq!(iterate_and_sum(vec![1, 10, 100, 2024]), 37327623);
    }
}
