//! This is my solution for [Advent of Code - Day 11: _Plutonian Pebbles_](https://adventofcode.com/2024/day/11)
//!
//!

use std::collections::HashMap;
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

fn parse_input(input: &String) -> Vec<u64> {
    input
        .trim()
        .split(" ")
        .map(|num| num.parse().unwrap())
        .collect()
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    stones
        .into_iter()
        .flat_map(|&stone| {
            if stone == 0 {
                return vec![1];
            }

            let as_str = format!("{stone}");
            if as_str.len() % 2 == 0 {
                let (left, right) = as_str.split_at(as_str.len() / 2);
                return vec![left.parse().unwrap(), right.parse().unwrap()];
            }

            return vec![stone * 2024];
        })
        .collect()
}

fn count_for_stone(stone: u64, iterations: u8, cache: &mut HashMap<(u64, u8), usize>) -> usize {
    if iterations == 0 {
        return 1;
    }

    if let Some(&count) = cache.get(&(stone, iterations)) {
        return count;
    }

    let result = blink(&vec![stone])
        .iter()
        .map(|&next_stone| count_for_stone(next_stone, iterations - 1, cache))
        .sum();

    cache.insert((stone, iterations), result);

    result
}

fn count_after_blinks(stones: &Vec<u64>, number_of_blinks: u8) -> usize {
    let mut cache = HashMap::new();

    stones
        .iter()
        .map(|&stone| count_for_stone(stone, number_of_blinks, &mut cache))
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
