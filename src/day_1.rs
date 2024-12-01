//! This is my solution for [Advent of Code - Day 1: _???_](https://adventofcode.com/2023/day/1)
//!
//!

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-1-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 1.
pub fn run() {
    let contents = fs::read_to_string("res/day-1-input.txt").expect("Failed to read file");
    let (left, right) = parse_input(&contents);
    
    println!("Sum of distances: {}", sum_diffs(&to_sorted_pairs(&left, &right)));
    println!("Sum of similarity scores: {}", sum_similarity_scores(&left, &right));
}

fn parse_input(input: &String) -> (Vec<u32>, Vec<u32>) {
    let mut left = vec![];
    let mut right = vec![];
    
    input.lines()
         .flat_map(|line| line.split_once("   "))
         .for_each(|(l, r)| {
             left.push(l.parse::<u32>().unwrap());
             right.push(r.parse::<u32>().unwrap());
         });
    
    (left, right)
}

fn to_sorted_pairs(left: &Vec<u32>, right: &Vec<u32>) -> Vec<(u32, u32)> {
    left.iter().sorted().zip(right.iter().sorted())
        .map(|(&l, &r)| (l, r))
        .collect()
}

fn sum_diffs(pairs: &Vec<(u32, u32)>) -> u32 {
    pairs.iter().map(|&(l,r)| l.abs_diff(r)).sum()
}

fn sum_similarity_scores(left: &Vec<u32>, right: &Vec<u32>) -> usize {
    let lookup = right.iter().counts();
    left.iter().map(|&id| (id as usize) * lookup.get(&id).unwrap_or(&0usize)).sum()
}

#[cfg(test)]
mod tests {
    use crate::day_1::*;
    
    fn sample_input() -> String {
        "\
3   4
4   3
2   5
1   3
3   9
3   3".to_string()
    }
    #[test]
    fn can_parse_input() {
        assert_eq!(
            parse_input(&sample_input()),
            (
                vec![3, 4, 2, 1, 3, 3],
                vec![4, 3, 5, 3, 9, 3]
            )
        );
    }
    
    #[test]
    fn can_generate_pairs() {
        assert_eq!(
            to_sorted_pairs(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]),
            vec!(
                (1, 3),
                (2, 3),
                (3, 3),
                (3, 4),
                (3, 5),
                (4, 9)
            )
        );
    }
    
    #[test]
    fn can_sum_diff() {
        assert_eq!(
            sum_diffs(&vec!(
                (1, 3),
                (2, 3),
                (3, 3),
                (3, 4),
                (3, 5),
                (4, 9)
            )),
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
