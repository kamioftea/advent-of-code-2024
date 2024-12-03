//! This is my solution for [Advent of Code - Day 2: _Red-Nosed Reports_](https://adventofcode.com/2024/day/2)
//!
//! [`parse_input`] uses [`parse_report`] to turn the input file into `Vec<Report>`. [`first_bad_level_pair`] is used
//! by both parts to find the first pair that causes the report to be unsafe. [`analyse_reports`] solves part 1.
//! [`report_check_with_dampener`] applies the more relaxed check for part 2, trying the permutations of dropping a
//! level that might allow the report to pass. [`analyse_reports_with_dampener`] uses that to get the part 2 solution.

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

/// Parse a line of input as a list of levels
fn parse_report(line: &str) -> Report {
    line.split(" ").flat_map(|num| num.parse()).collect()
}

/// Parse the input file into a list of reports
fn parse_input(input: &String) -> Vec<Report> {
    input.lines().map(parse_report).collect()
}

/// Find the first pair in the report that either:
/// - Is too large a jump in levels
/// - Is not in the same direction as previous pairs
///
/// Returns `None` if the report is safe, or `Some(idx)` - the first pair that makes the report unsafe.
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

/// Returns a copy of the input report without the level at the specified index
fn without_index(report: &Report, idx: usize) -> Report {
    let mut new = report.clone();
    new.remove(idx);

    new
}

/// If the report is unsafe, it can be considered safe enough if it becomes safe when removing one level.
///
/// The level to remove must be one of the pair that causes the initial check to fail, or the first level in the
/// report if that causes the first step to be in the wrong direction.
fn report_check_with_dampener(report: &Report) -> bool {
    if let Some(pair_idx) = first_bad_level_pair(report) {
        let lower_bound = if pair_idx == 1 { 0 } else { pair_idx };
        (lower_bound..=(pair_idx + 1))
            .into_iter()
            .any(|level_idx| first_bad_level_pair(&without_index(report, level_idx)).is_none())
    } else {
        true
    }
}

/// Solves part 1, counting all the reports that are safe as is
fn analyse_reports(reports: &Vec<Report>) -> usize {
    reports
        .into_iter()
        .filter(|&report| first_bad_level_pair(report).is_none())
        .count()
}

/// Solves part 1, counting all the reports that are safe after dampening
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
1 3 6 7 9
5 3 6 7 9"
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
            vec![5, 3, 6, 7, 9],
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
        assert_eq!(analyse_reports_with_dampener(&sample_reports()), 5)
    }
}
