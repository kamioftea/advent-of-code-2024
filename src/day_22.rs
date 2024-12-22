//! This is my solution for [Advent of Code - Day 22: _Monkey Market_](https://adventofcode.com/2024/day/22)
//!
//! [`parse_input`] parses the inout file as a list of ints.
//!
//! [`iterate_and_sum`] solves part 1, using [`pseudorandom_sequence`] to provide an iterator of the sequence from
//! the seed by repeatedly calling [`NumberExtensions::next_secret`].
//!
//! [`bananas_from_best_diff_sequence`] solves part 2 by keeping track of the bananas earned by each leading sequence
//! of four diffs, and then picking the maximum. For performance this packs the sequence into a 20-bit int using
//! [`shift_diff_into_sequence_id`] to manage that.

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
    /// Mixing is xor with the previous values
    fn mix(&mut self, prev: &Self) -> () {
        *self ^= prev
    }

    /// Pruning is taking the remainder mod 16777216
    fn prune(&mut self) -> () {
        *self %= 16777216
    }

    /// Given a number, generate the next pseudo random number
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

/// The input file is one integer seed per line
fn parse_input(input: &String) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// Generate a pseudorandom iterator from a seed by repeatedly calling [`NumberExtensions::next_secret`]
fn pseudorandom_sequence(seed: u64) -> impl Iterator<Item = u64> {
    iterate(seed, |prev| prev.next_secret())
}

/// Solves part 1 - take the 2000th value from each seed's pseudorandom sequence and sum
fn iterate_and_sum(seeds: &Vec<u64>) -> u64 {
    seeds
        .iter()
        .map(|seed| pseudorandom_sequence(*seed).nth(2000).unwrap())
        .sum()
}

/// For performance reasons the sequence of four diffs before the current price is packed into a 20-bit integer
///
/// Each diff is max 19, so fits in 5 bits. Given the first four diffs packed like so:
///
/// ```text
///     11111222223333344444
/// &   00000222223333344444 : The first diff is removed by applying a bit mask
/// <<5 22222333334444400000 : The previous numbers are left shifted
/// +   22222333334444455555 : The new diff is added, taking the least-significant 5 bits
///```
///
fn shift_diff_into_sequence_id(state: &mut usize, prev: usize, current: usize) {
    *state &= (1 << 15) - 1;
    *state <<= 5;
    *state += 10 + current - prev;
}

/// This finds the price of banana each sequence of four diffs that are present in the sequence will fetch. These
/// are added to a mutable `Vec` indexed by the sequence_id, which is the previous four diffs packed into an int by
/// [`shift_diff_into_sequence_id`]. Once a diff has been seen for each seed, future instances of that sequence are
/// ignored. Again this is managed using a `Vec` that keeps track of the last id that wrote to a specific
/// sequence_id, which is allocated once and passed in for performance.
fn populate_sequence_scores(
    sequence_scores: &mut Vec<usize>,
    seen: &mut Vec<usize>,
    seed: u64,
    id: usize,
) {
    pseudorandom_sequence(seed)
        .take(2000)
        .map(|secret| (secret % 10) as usize)
        .tuple_windows()
        .scan(0, |state, (prev, current)| {
            shift_diff_into_sequence_id(state, prev, current);
            Some((*state, current))
        })
        // The sequence id needs to be populated with four values before starting to store sequence prices
        .dropping(3)
        .for_each(|(sequence, price)| {
            if seen[sequence] != id {
                seen[sequence] = id;
                sequence_scores[sequence] += price
            }
        })
}

/// Solves part 2, Build a map from price difference sequences to bananas bought, and pick the best.
fn bananas_from_best_diff_sequence(seeds: &Vec<u64>) -> usize {
    let mut sequence_scores = vec![0; 0xFFFFF];
    let mut seen = vec![0; 0xFFFFF];
    for (idx, &seed) in seeds.iter().enumerate() {
        populate_sequence_scores(&mut sequence_scores, &mut seen, seed, idx + 1);
    }

    sequence_scores.iter().max().unwrap_or(&0).clone()
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
    fn can_generate_next_secret() {
        assert_eq!(123.next_secret(), 15887950)
    }

    #[test]
    fn can_iterate_secret_number() {
        assert_eq!(
            pseudorandom_sequence(123)
                .dropping(1)
                .take(10)
                .collect::<Vec<u64>>(),
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254
            ]
        );

        assert_eq!(pseudorandom_sequence(1).nth(2000), Some(8685429));
        assert_eq!(pseudorandom_sequence(10).nth(2000), Some(4700978));
        assert_eq!(pseudorandom_sequence(100).nth(2000), Some(15273692));
        assert_eq!(pseudorandom_sequence(2024).nth(2000), Some(8667524));
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
