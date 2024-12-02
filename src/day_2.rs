//! This is my solution for [Advent of Code - Day 2: _Red-Nosed Reports_](https://adventofcode.com/2024/day/2)
//!
//!

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-2-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 2.
pub fn run() {
    let contents = fs::read_to_string("res/day-2-input.txt").expect("Failed to read file");
    let reports = parse_input(&contents);

    println!("There are {} safe reports", analyse_reports(&reports));
    println!(
        "There are {} safe reports with the dampener",
        analyse_reports_with_dampener(&reports)
    );
}

type Report = Vec<u32>;

fn parse_line(line: &str) -> Report {
    line.split(" ").flat_map(|num| num.parse()).collect()
}

fn parse_input(input: &String) -> Vec<Report> {
    input.lines().map(parse_line).collect()
}

fn first_bad_level_pair(report: &Report) -> Option<usize> {
    let mut maybe_direction = None;
    for (idx, (&l, &r)) in report.iter().tuple_windows().enumerate() {
        if l == r || l.abs_diff(r) > 3 {
            return Some(idx);
        }

        maybe_direction = maybe_direction.or_else(|| Some(l > r));
        let direction = maybe_direction.unwrap();

        if direction ^ (l > r) {
            return Some(idx);
        }
    }

    None
}

fn without_index(report: &Report, idx: usize) -> Report {
    let mut new = report.clone();
    new.remove(idx);

    new
}

fn report_check_with_dampener(report: &Report) -> bool {
    if let Some(pair_idx) = first_bad_level_pair(report) {
        let lower_bound = pair_idx.checked_sub(1).unwrap_or(0);
        (lower_bound..=(pair_idx + 1))
            .into_iter()
            .any(|level_idx| first_bad_level_pair(&without_index(report, level_idx)).is_none())
    } else {
        true
    }
}

fn analyse_reports(reports: &Vec<Report>) -> usize {
    reports
        .into_iter()
        .filter(|&report| first_bad_level_pair(report).is_none())
        .count()
}

fn analyse_reports_with_dampener(reports: &Vec<Report>) -> usize {
    reports
        .into_iter()
        .filter(|&report| report_check_with_dampener(report))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day_2::*;
    
    fn sample_input() -> String {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string()
    }

    fn sample_reports() -> Vec<Report> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    #[test]
    fn can_parse_input() {
        assert_eq!(parse_input(&sample_input()), sample_reports())
    }

    #[test]
    fn can_check_if_a_report_is_safe() {
        assert_eq!(first_bad_level_pair(&vec![7, 6, 4, 2, 1]), None);
        assert_eq!(first_bad_level_pair(&vec![1, 2, 7, 8, 9]), Some(1));
        assert_eq!(first_bad_level_pair(&vec![9, 7, 6, 2, 1]), Some(2));
        assert_eq!(first_bad_level_pair(&vec![1, 3, 2, 4, 5]), Some(1));
        assert_eq!(first_bad_level_pair(&vec![8, 6, 4, 4, 1]), Some(2));
        assert_eq!(first_bad_level_pair(&vec![1, 3, 6, 7, 9]), None);
        assert_eq!(first_bad_level_pair(&vec![4, 3, 6, 7, 9]), Some(1));
    }

    #[test]
    fn can_check_if_a_report_is_safe_with_dampener() {
        assert_eq!(report_check_with_dampener(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(report_check_with_dampener(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(report_check_with_dampener(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(report_check_with_dampener(&vec![1, 3, 2, 4, 5]), true);
        assert_eq!(report_check_with_dampener(&vec![8, 6, 4, 4, 1]), true);
        assert_eq!(report_check_with_dampener(&vec![1, 3, 6, 7, 9]), true);
        assert_eq!(report_check_with_dampener(&vec![5, 3, 4, 7, 9]), true);
    }

    #[test]
    fn can_analyse_reports() {
        assert_eq!(analyse_reports(&sample_reports()), 2)
    }

    #[test]
    fn can_analyse_reports_with_dampener() {
        assert_eq!(analyse_reports_with_dampener(&sample_reports()), 4)
    }
}
