//! This is my solution for [Advent of Code - Day 1: _Historian Hysteria_](https://adventofcode.com/2024/day/1)
//!
//! The input has been turned into a list of u32 for each column by [`parse_input`]. Part 1 is solved in two steps
//! [`to_sorted_pairs`] sorts the lists and zips them together, then [`sum_diffs`] reduces the list of pairs to the
//! puzzle solution. Part 2 is solved by [`sum_similarity_scores`].

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-1-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 1.
pub fn run() {
    let contents = fs::read_to_string("res/day-1-input.txt").expect("Failed to read file");
    let (left, right) = parse_input(&contents);

    println!(
        "Sum of distances: {}",
        sum_diffs(&to_sorted_pairs(&left, &right))
    );
    println!(
        "Sum of similarity scores: {}",
        sum_similarity_scores(&left, &right)
    );
}

/// Build up lists of ids from the puzzle input. The input is two columns of numbers separated by three spaces, e.g.
///
/// ```text
/// 3   4
/// 4   3
/// 2   5
/// 1   3
/// 3   9
/// 3   3
/// ```
fn parse_input(input: &String) -> (Vec<u32>, Vec<u32>) {
    let mut left = vec![];
    let mut right = vec![];

    input
        .lines()
        .flat_map(|line| line.split_once("   "))
        .for_each(|(l, r)| {
            left.push(l.parse::<u32>().unwrap());
            right.push(r.parse::<u32>().unwrap());
        });

    (left, right)
}

/// The first part of the solution to part 1. Pair the lowest integers in each list, then second lowest, and so on...
fn to_sorted_pairs(left: &Vec<u32>, right: &Vec<u32>) -> Vec<(u32, u32)> {
    let sorted_left = left.iter().cloned().sorted();
    let sorted_right = right.iter().cloned().sorted();
    sorted_left.zip(sorted_right).collect()
}

/// The second part of the solution to part 1, given a sorted list of pairs, sum the distance between them
fn sum_diffs(pairs: &Vec<(u32, u32)>) -> u32 {
    pairs.iter().map(|&(l, r)| l.abs_diff(r)).sum()
}

/// The solution to part 2. The similarity score for a number in the left-hand column is that number multiplied by the
/// number of times it appears in the right-hand column.
fn sum_similarity_scores(left: &Vec<u32>, right: &Vec<u32>) -> usize {
    let lookup = right.iter().counts();
    left.iter()
        .map(|&id| (id as usize) * lookup.get(&id).unwrap_or(&0usize))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_1::*;
    
    fn sample_input() -> String {
        "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string()
    }
    #[test]
    fn can_parse_input() {
        assert_eq!(
            parse_input(&sample_input()),
            (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
        );
    }

    #[test]
    fn can_generate_pairs() {
        assert_eq!(
            to_sorted_pairs(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]),
            vec!((1, 3), (2, 3), (3, 3), (3, 4), (3, 5), (4, 9))
        );
    }

    #[test]
    fn can_sum_diff() {
        assert_eq!(
            sum_diffs(&vec!((1, 3), (2, 3), (3, 3), (3, 4), (3, 5), (4, 9))),
            11
        );
    }

    #[test]
    fn can_sum_similarity_scores() {
        assert_eq!(
            sum_similarity_scores(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]),
            31
        )
    }
}
