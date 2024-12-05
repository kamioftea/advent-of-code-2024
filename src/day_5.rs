//! This is my solution for [Advent of Code - Day 5: _Print Queue_](https://adventofcode.com/2024/day/5)
//!
//! [`parse_input`] splits the input into the two sections, delegating these to [`parse_rules`] and [`parse_updates`]
//! respectively.
//!
//! Part 1 is solved by [`sum_valid_middle_pages`], with the validation being done by [`validate_update`], and
//! [`get_middle`] split out for ease of reuse.
//!
//! Part 2 is solved by [`sort_and_sum_invalid_middle_pages`], with [`sort_pages`] doing the extra work, everything
//! else is reused from part 1.

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-5-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 5.
pub fn run() {
    let contents = fs::read_to_string("res/day-5-input.txt").expect("Failed to read file");

    let (rules, updates) = parse_input(&contents);

    println!(
        "The sum of valid middle page numbers is {}",
        sum_valid_middle_pages(&updates, &rules)
    );

    println!(
        "The sum of sorted invalid middle page numbers is {}",
        sort_and_sum_invalid_middle_pages(&updates, &rules)
    );
}

/// Rules represented as a lookup from page number to the set of pages that must come after
type Rules = HashMap<u32, HashSet<u32>>;
/// Updates as a list of page numbers
type Update = Vec<u32>;

/// Build the Rules lookup by adding each rule in turn.
fn parse_rules(input: &str) -> Rules {
    let mut rules: Rules = HashMap::new();
    input
        .lines()
        .flat_map(|line| line.split_once("|"))
        .for_each(|(before, after)| {
            let key = before.parse().unwrap();
            let value = after.parse().unwrap();

            rules.entry(key).or_default().insert(value);
        });

    rules
}

/// Parse the lines of input into lists of page numbers
fn parse_updates(input: &str) -> Vec<Update> {
    input
        .lines()
        .map(|line| line.split(",").flat_map(|page| page.parse()).collect())
        .collect()
}

/// Split the list into two sections and parse each
fn parse_input(input: &String) -> (Rules, Vec<Update>) {
    let (rule_input, updates_input) = input.split_once("\n\n").unwrap();

    (parse_rules(rule_input), parse_updates(updates_input))
}

/// For a given [`Update`], check for each page that a page already in the update list does not need to come after
/// the current page.
fn validate_update(update: &Update, rules: &Rules) -> bool {
    let mut seen = HashSet::new();
    let empty = HashSet::new();

    for page in update {
        let rule = rules.get(page).unwrap_or(&empty);
        if seen.intersection(rule).next().is_some() {
            return false;
        }
        seen.insert(*page);
    }

    true
}

/// Utility to get the middle page of an update.
fn get_middle(update: &Update) -> u32 {
    let middle = (update.len() - 1) / 2;
    update.get(middle).unwrap().clone()
}

/// Solution to part 1, find the valid update lists and sum their middle page.
fn sum_valid_middle_pages(updates: &Vec<Update>, rules: &Rules) -> u32 {
    updates
        .iter()
        .filter(|update| validate_update(update, rules))
        .map(|update| get_middle(update))
        .sum()
}

/// Assuming all pairs of pages have rules that specify an ordering for that pair, use that to provide a sorting
/// function for [`Itertools::sorted_by`].
fn sort_pages(update: &Update, rules: &Rules) -> Update {
    let empty = HashSet::new();

    update
        .iter()
        .sorted_by(|page_a, page_b| {
            let rule_a = rules.get(page_a).unwrap_or(&empty);
            let rule_b = rules.get(page_b).unwrap_or(&empty);

            if rule_a.contains(page_b) {
                Ordering::Less
            } else if rule_b.contains(page_a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
        .cloned()
        .collect()
}

/// Solution to part 2 - similar to [`sum_valid_middle_pages`], but finds invalid pages and sorts them before
/// extracting thr middle and summing.
fn sort_and_sum_invalid_middle_pages(updates: &Vec<Update>, rules: &Rules) -> u32 {
    updates
        .iter()
        .filter(|update| !validate_update(update, rules))
        .map(|update| sort_pages(update, &rules))
        .map(|update| get_middle(&update))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_5::*;
    use crate::helpers::test::assert_contains_in_any_order;
    
    fn example_rules() -> Rules {
        vec![
            (97, vec![13, 61, 47, 29, 53, 75].into_iter().collect()),
            (75, vec![29, 53, 47, 61, 13].into_iter().collect()),
            (61, vec![13, 53, 29].into_iter().collect()),
            (29, vec![13].into_iter().collect()),
            (53, vec![29, 13].into_iter().collect()),
            (47, vec![53, 13, 61, 29].into_iter().collect()),
        ]
        .into_iter()
        .collect()
    }

    fn example_updates() -> Vec<Update> {
        vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ]
    }

    #[test]
    fn can_parse_input() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string();

        let (rules, updates) = parse_input(&input);

        assert_contains_in_any_order(rules, example_rules());
        assert_contains_in_any_order(updates, example_updates());
    }

    #[test]
    fn can_validate_updates() {
        let rules = example_rules();

        assert!(validate_update(&vec![75, 47, 61, 53, 29], &rules));
        assert!(validate_update(&vec![97, 61, 53, 29, 13], &rules));
        assert!(validate_update(&vec![75, 29, 13], &rules));
        assert!(!validate_update(&vec![75, 97, 47, 61, 53], &rules));
        assert!(!validate_update(&vec![61, 13, 29], &rules));
        assert!(!validate_update(&vec![97, 13, 75, 29, 47], &rules));
    }

    #[test]
    fn can_get_middle_page() {
        assert_eq!(get_middle(&vec![75, 47, 61, 53, 29]), 61);
        assert_eq!(get_middle(&vec![97, 61, 53, 29, 13]), 53);
        assert_eq!(get_middle(&vec![75, 29, 13]), 29);
        assert_eq!(get_middle(&vec![75, 97, 47, 61, 53]), 47);
        assert_eq!(get_middle(&vec![61, 13, 29]), 13);
        assert_eq!(get_middle(&vec![97, 13, 75, 29, 47]), 75);
    }

    #[test]
    fn can_sum_valid_middle_pages() {
        assert_eq!(
            sum_valid_middle_pages(&example_updates(), &example_rules()),
            143
        )
    }

    #[test]
    fn can_sort_pages() {
        let rules = example_rules();
        assert_eq!(
            sort_pages(&vec![75, 97, 47, 61, 53], &rules),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(sort_pages(&vec![61, 13, 29], &rules), vec![61, 29, 13]);
        assert_eq!(
            sort_pages(&vec![97, 13, 75, 29, 47], &rules),
            vec![97, 75, 47, 29, 13]
        );
    }

    #[test]
    fn can_sort_and_sum_invalid_middle_pages() {
        assert_eq!(
            sort_and_sum_invalid_middle_pages(&example_updates(), &example_rules()),
            123
        )
    }
}
