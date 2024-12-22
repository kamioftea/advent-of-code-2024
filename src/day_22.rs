//! This is my solution for [Advent of Code - Day 22: _Monkey Market_](https://adventofcode.com/2024/day/22)
//!
//!

use itertools::{iterate, Itertools};
use std::collections::{HashMap, HashSet};
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
        iterate_and_sum(&seeds)
    );

    println!(
        "The best sequence for today's market buys {} bananas",
        bananas_from_best_diff_sequence(&seeds)
    );
}

trait NumberExtensions {
    fn mix(&mut self, prev: &Self) -> ();
    fn prune(&mut self) -> ();
    fn next_secret(&self) -> Self;
}

impl NumberExtensions for u64 {
    fn mix(&mut self, prev: &Self) -> () {
        *self ^= prev
    }

    fn prune(&mut self) -> () {
        *self %= 16777216
    }

    fn next_secret(&self) -> Self {
        let mut next = *self;

        next.mix(&(next * 64));
        next.prune();

        next.mix(&(next / 32));
        next.prune();

        next.mix(&(next * 2048));
        next.prune();

        next
    }
}

fn pseudorandom(seed: u64) -> impl Iterator<Item = u64> {
    iterate(seed, |prev| prev.next_secret())
}

fn iterate_and_sum(seeds: &Vec<u64>) -> u64 {
    seeds
        .iter()
        .map(|seed| pseudorandom(*seed).nth(2000).unwrap())
        .sum()
}

fn populate_sequence_scores(sequence_scores: &mut HashMap<(i8, i8, i8, i8), u64>, seed: u64) {
    let mut seen = HashSet::new();
    pseudorandom(seed)
        .take(2000)
        .map(|secret| (secret % 10) as i8)
        .tuple_windows()
        .for_each(|(a, b, c, d, e)| {
            let diff_sequence = (b - a, c - b, d - c, e - d);
            if seen.insert(diff_sequence) {
                *(sequence_scores.entry(diff_sequence).or_default()) += e as u64;
            }
        })
}

fn bananas_from_best_diff_sequence(seeds: &Vec<u64>) -> u64 {
    let mut sequence_scores = HashMap::new();
    for &seed in seeds {
        populate_sequence_scores(&mut sequence_scores, seed);
    }

    sequence_scores.values().max().unwrap_or(&0).clone()
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
        let mut num = 42;
        num.mix(&15);
        assert_eq!(num, 37)
    }

    #[test]
    fn can_prune_numbers() {
        let mut num = 100000000;
        num.prune();
        assert_eq!(num, 16113920);
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
        assert_eq!(iterate_and_sum(&vec![1, 10, 100, 2024]), 37327623);
    }

    #[test]
    fn can_find_best_sequence() {
        assert_eq!(bananas_from_best_diff_sequence(&vec![1, 2, 3, 2024]), 23)
    }
}
