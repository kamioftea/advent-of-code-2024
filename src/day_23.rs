//! This is my solution for [Advent of Code - Day 23: _LAN Party_](https://adventofcode.com/2024/day/23)
//!
//! [`parse_input`] turns the input into a [`Network`] where each node can be mapped to the set of computer ids it's
//! connected to.
//!
//! Part 1 is solved by [`Network::clusters_containing`] using [`Network::trios`]
//!
//! Part 2 is solved by [`Network::find_lan_password`] using [`Network::find_lan_password`]

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-23-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 23.
pub fn run() {
    let contents = fs::read_to_string("res/day-23-input.txt").expect("Failed to read file");
    let network = parse_input(&contents);

    println!(
        "There are {} trios containing ids starting with 't'",
        network.clusters_containing("t").len()
    );

    println!("The lan password is {}", network.find_lan_password());
}

/// Represents a network of computers as a map from any computer to the ids of its direct connections
#[derive(Eq, PartialEq, Debug)]
struct Network<'a> {
    links: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Network<'a> {
    /// Find all the sets of three mutually interconnected computers
    fn trios(&self) -> HashSet<Vec<&str>> {
        let mut clusters = HashSet::new();

        for (start, connected) in self.links.clone() {
            for (a, b) in connected.iter().tuple_combinations() {
                if self.links.get(a).unwrap().contains(b) {
                    clusters.insert(vec![start, a, b].into_iter().sorted().collect());
                }
            }
        }

        clusters
    }

    /// Uses [`Network::trios`] to find all clusters of three, and filters to only those where at least one computer
    /// starts with the provided character
    fn clusters_containing(&self, char: &str) -> Vec<Vec<&str>> {
        self.trios()
            .iter()
            .filter(|cluster| cluster.iter().any(|node| node.starts_with(char)))
            .cloned()
            .collect()
    }

    /// Given a starting computer id and the list of it's direct connections, find all that are also mutually
    /// interconnected
    fn find_fully_connected_cluster(
        &self,
        start: &'a str,
        connected: &HashSet<&'a str>,
    ) -> Vec<&str> {
        let mut cluster = vec![start];
        for computer in connected {
            if cluster.iter().all(|b| self.links[computer].contains(b)) {
                cluster.push(*computer);
            }
        }
        cluster
    }

    /// For each node find a cluster that is fully interconnected, and then take the biggest and turn it into a
    /// password.
    fn find_lan_password(&self) -> String {
        self.links
            .iter()
            .map(|(&start, connected)| self.find_fully_connected_cluster(start, connected))
            .max_by_key(|c| c.len())
            .unwrap()
            .iter()
            .sorted()
            .join(",")
    }
}

/// Build a network from lines like `ab-cd` denoting that `ab` is directly connected to `cd`.
fn parse_input(input: &String) -> Network {
    let mut links: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (a, b) in input.lines().map(|line| line.split_once("-").unwrap()) {
        links.entry(a).or_default().insert(b);
        links.entry(b).or_default().insert(a);
    }

    Network { links }
}

#[cfg(test)]
mod tests {
    use crate::day_23::*;
    use crate::helpers::test::assert_contains_in_any_order;

    fn example_network() -> Network<'static> {
        let links = vec![
            ("kh", vec!["tc", "qp", "ub", "ta"].into_iter().collect()),
            ("tc", vec!["kh", "wh", "td", "co"].into_iter().collect()),
            ("qp", vec!["kh", "ub", "td", "wh"].into_iter().collect()),
            ("de", vec!["cg", "co", "ta", "ka"].into_iter().collect()),
            ("cg", vec!["de", "tb", "yn", "aq"].into_iter().collect()),
            ("ka", vec!["co", "tb", "ta", "de"].into_iter().collect()),
            ("co", vec!["ka", "ta", "de", "tc"].into_iter().collect()),
            ("yn", vec!["aq", "cg", "wh", "td"].into_iter().collect()),
            ("aq", vec!["yn", "vc", "cg", "wq"].into_iter().collect()),
            ("ub", vec!["qp", "kh", "wq", "vc"].into_iter().collect()),
            ("tb", vec!["cg", "ka", "wq", "vc"].into_iter().collect()),
            ("vc", vec!["aq", "ub", "wq", "tb"].into_iter().collect()),
            ("wh", vec!["tc", "td", "yn", "qp"].into_iter().collect()),
            ("ta", vec!["co", "ka", "de", "kh"].into_iter().collect()),
            ("td", vec!["tc", "wh", "qp", "yn"].into_iter().collect()),
            ("wq", vec!["tb", "ub", "aq", "vc"].into_iter().collect()),
        ]
        .into_iter()
        .collect();

        Network { links }
    }

    #[test]
    fn can_parse_input() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"
        .to_string();

        let actual = parse_input(&input);
        let expected = example_network();

        assert_contains_in_any_order(actual.links.keys(), expected.links.keys());
        for (key, value) in actual.links {
            assert_contains_in_any_order(&value, expected.links.get(key).unwrap())
        }
    }

    #[test]
    fn can_find_clusters() {
        assert_eq!(
            example_network().trios(),
            vec![
                vec!["aq", "cg", "yn"],
                vec!["aq", "vc", "wq"],
                vec!["co", "de", "ka"],
                vec!["co", "de", "ta"],
                vec!["co", "ka", "ta"],
                vec!["de", "ka", "ta"],
                vec!["kh", "qp", "ub"],
                vec!["qp", "td", "wh"],
                vec!["tb", "vc", "wq"],
                vec!["tc", "td", "wh"],
                vec!["td", "wh", "yn"],
                vec!["ub", "vc", "wq"],
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn can_find_clusters_starting_with_t() {
        assert_contains_in_any_order(
            example_network().clusters_containing("t"),
            vec![
                vec!["co", "de", "ta"],
                vec!["co", "ka", "ta"],
                vec!["de", "ka", "ta"],
                vec!["qp", "td", "wh"],
                vec!["tb", "vc", "wq"],
                vec!["tc", "td", "wh"],
                vec!["td", "wh", "yn"],
            ]
            .into_iter()
            .collect::<Vec<Vec<&str>>>(),
        );
    }

    #[test]
    fn can_find_lan_password() {
        assert_eq!(example_network().find_lan_password(), "co,de,ka,ta");
    }
}
