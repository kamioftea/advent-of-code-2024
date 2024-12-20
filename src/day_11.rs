//! This is my solution for [Advent of Code - Day 11: _Plutonian Pebbles_](https://adventofcode.com/2024/day/11)
//!
//! [`parse_input`] turns the text into numbers.
//!
//! [`count_after_blinks`] solves both parts, calling [`count_for_stone`] recursively. This is cached as there are a
//! lot of repeat small numbers at each depth. [`blink`] handles a single blink.

use cached::proc_macro::cached;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-11-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 11.
pub fn run() {
    let contents = fs::read_to_string("res/day-11-input.txt").expect("Failed to read file");
    let stones = parse_input(&contents);

    println!(
        "After 25 blinks there are {} stones",
        count_after_blinks(&stones, 25)
    );

    println!(
        "After 75 blinks there are {} stones",
        count_after_blinks(&stones, 75)
    );
}

/// Turn the space separated number strings into `u64`s
fn parse_input(input: &String) -> Vec<u64> {
    input
        .trim()
        .split(" ")
        .map(|num| num.parse().unwrap())
        .collect()
}

/// This was originally written to work on the whole array of stones, but that wasn't convenient for caching. It is
/// left as is for convenience of using existing tests.
fn blink(stones: &Vec<u64>) -> Vec<u64> {
    stones
        .into_iter()
        .flat_map(|&stone| {
            if stone == 0 {
                return vec![1];
            }

            // Even number of digits, split in two
            let digits = stone.ilog(10) + 1;
            if digits % 2 == 0 {
                let midpoint = 10u64.pow(digits / 2);
                return vec![stone / midpoint, stone % midpoint];
            }

            vec![stone * 2024]
        })
        .collect()
}

/// Recursive function to calculate the expansion of a single stone. There are many repeats, especially for small
/// numbers, so this is cached to allow the program to run quickly.
#[cached]
fn count_for_stone(stone: u64, iterations: u8) -> usize {
    if iterations == 0 {
        return 1;
    }

    let result = blink(&vec![stone])
        .iter()
        .map(|&next_stone| count_for_stone(next_stone, iterations - 1))
        .sum();

    result
}

/// Solves both parts. Delegates to [`count_for_stone`] for each stone in the list.
fn count_after_blinks(stones: &Vec<u64>, number_of_blinks: u8) -> usize {
    stones
        .iter()
        .map(|&stone| count_for_stone(stone, number_of_blinks))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_11::*;
    
    #[test]
    fn can_parse_input() {
        assert_eq!(
            parse_input(&"0 1 10 99 999\n".to_string()),
            vec![0, 1, 10, 99, 999]
        )
    }

    #[test]
    fn can_step_stones() {
        assert_eq!(
            blink(&vec![0, 1, 10, 99, 999]),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );
        assert_eq!(blink(&vec![125, 17]), vec![253000, 1, 7]);
        assert_eq!(blink(&vec![253000, 1, 7]), vec![253, 0, 2024, 14168]);
        assert_eq!(
            blink(&vec![253, 0, 2024, 14168]),
            vec![512072, 1, 20, 24, 28676032]
        );
        assert_eq!(
            blink(&vec![512072, 1, 20, 24, 28676032]),
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]
        );
        assert_eq!(
            blink(&vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]),
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        assert_eq!(
            blink(&vec![
                1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32
            ]),
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }

    #[test]
    fn can_count_stones_after_n_blinks() {
        assert_eq!(count_after_blinks(&vec![125, 17], 6), 22);

        assert_eq!(count_after_blinks(&vec![125, 17], 25), 55312);
    }
}
