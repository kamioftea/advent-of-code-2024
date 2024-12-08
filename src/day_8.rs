//! This is my solution for [Advent of Code - Day 8: _Resonant Collinearity_](https://adventofcode.com/2024/day/8)
//!
//!

use itertools::{iterate, Itertools};
use std::collections::HashMap;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-8-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 8.
pub fn run() {
    let contents = fs::read_to_string("res/day-8-input.txt").expect("Failed to read file");
    let antenna_map = parse_input(&contents);

    println!(
        "There are {} unique antinodes",
        count_antinodes_for_map(&antenna_map, antinode_pair_sequence_modifier)
    );

    println!(
        "There are {} unique antinodes",
        count_antinodes_for_map(&antenna_map, resonant_harmonies_sequence_modifier)
    );
}

type Coordinate = (usize, usize);

#[derive(Eq, PartialEq, Debug)]
struct AntennaMap {
    height: usize,
    width: usize,
    antenna: HashMap<char, Vec<Coordinate>>,
}

fn parse_input(input: &String) -> AntennaMap {
    let mut lines = input.lines();
    let width = lines.next().unwrap().len();
    let height = lines.count() + 1;
    let mut antenna: HashMap<char, Vec<Coordinate>> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                antenna.entry(char).or_default().push((row, col))
            }
        }
    }

    AntennaMap {
        width,
        height,
        antenna,
    }
}

type SequenceModifier = fn(Vec<Coordinate>) -> Vec<Coordinate>;

fn sequence_from_antenna(
    (r, c): Coordinate,
    (dr, dc): (isize, isize),
    (height, width): &(usize, usize),
) -> Vec<Coordinate> {
    iterate(0, |i| i + 1)
        .map(move |i| {
            r.checked_add_signed(i * dr)
                .zip(c.checked_add_signed(i * dc))
                .filter(|(r, c)| r < height && c < width)
        })
        .while_some()
        .collect()
}

fn find_antinodes_for_pair(
    (r1, c1): Coordinate,
    (r2, c2): Coordinate,
    bounds: &(usize, usize),
    sequence_modifier: SequenceModifier,
) -> Vec<Coordinate> {
    let dr = r1 as isize - r2 as isize;
    let dc = c1 as isize - c2 as isize;

    let increasing = sequence_from_antenna((r1, c1).clone(), (dr, dc).clone(), bounds);
    let decreasing = sequence_from_antenna((r2, c2), (-dr, -dc), bounds);

    [sequence_modifier(increasing), sequence_modifier(decreasing)].concat()
}

fn antinode_pair_sequence_modifier(coordinate_sequence: Vec<Coordinate>) -> Vec<Coordinate> {
    coordinate_sequence
        .into_iter()
        .dropping(1)
        .take(1)
        .collect()
}

fn resonant_harmonies_sequence_modifier(coordinate_sequence: Vec<Coordinate>) -> Vec<Coordinate> {
    coordinate_sequence
}

fn find_antinodes_for_frequency(
    antenna: &Vec<Coordinate>,
    bounds: &(usize, usize),
    sequence_modifier: SequenceModifier,
) -> Vec<Coordinate> {
    antenna
        .iter()
        .tuple_combinations()
        .flat_map(|(a1, a2)| find_antinodes_for_pair(*a1, *a2, bounds, sequence_modifier))
        .unique()
        .collect()
}

fn count_antinodes_for_map(antenna_map: &AntennaMap, sequence_modifier: SequenceModifier) -> usize {
    let bounds = (antenna_map.height, antenna_map.width);
    antenna_map
        .antenna
        .values()
        .flat_map(|antenna| find_antinodes_for_frequency(antenna, &bounds, sequence_modifier))
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day_8::*;
    use crate::helpers::test::assert_contains_in_any_order;
    
    #[test]
    fn can_parse_input() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .to_string();

        assert_eq!(parse_input(&input), example_map());
    }

    fn example_map() -> AntennaMap {
        AntennaMap {
            height: 12,
            width: 12,
            antenna: vec![
                ('0', vec![(1, 8), (2, 5), (3, 7), (4, 4)]),
                ('A', vec![(5, 6), (8, 8), (9, 9)]),
            ]
            .into_iter()
            .collect(),
        }
    }

    #[test]
    fn can_find_antinodes_for_pair() {
        assert_contains_in_any_order(
            find_antinodes_for_pair((3, 4), (5, 5), &(12, 12), antinode_pair_sequence_modifier),
            vec![(1, 3), (7, 6)],
        );
        assert_contains_in_any_order(
            find_antinodes_for_pair((4, 8), (5, 5), &(12, 12), antinode_pair_sequence_modifier),
            vec![(6, 2), (3, 11)],
        );
        assert_contains_in_any_order(
            find_antinodes_for_pair((4, 8), (5, 5), &(10, 10), antinode_pair_sequence_modifier),
            vec![(6, 2)],
        );
        assert_contains_in_any_order(
            find_antinodes_for_pair((1, 1), (3, 3), &(10, 10), antinode_pair_sequence_modifier),
            vec![(5, 5)],
        );
    }

    #[test]
    fn can_find_antinodes_for_pair_with_resonant_harmonics() {
        assert_contains_in_any_order(
            find_antinodes_for_pair(
                (2, 3),
                (3, 5),
                &(10, 10),
                resonant_harmonies_sequence_modifier,
            ),
            vec![(1, 1), (2, 3), (3, 5), (4, 7), (5, 9)],
        );
        assert_contains_in_any_order(
            find_antinodes_for_pair(
                (4, 3),
                (3, 5),
                &(10, 10),
                resonant_harmonies_sequence_modifier,
            ),
            vec![(5, 1), (4, 3), (3, 5), (2, 7), (1, 9)],
        );
    }

    #[test]
    fn can_find_antinodes_for_frequency() {
        assert_contains_in_any_order(
            find_antinodes_for_frequency(
                &vec![(3, 4), (4, 8), (5, 5)],
                &(10, 10),
                antinode_pair_sequence_modifier,
            ),
            vec![(1, 3), (7, 6), (6, 2), (2, 0)],
        );
    }

    #[test]
    fn can_find_antinodes_for_frequency_with_resonant_harmonics() {
        assert_contains_in_any_order(
            find_antinodes_for_frequency(
                &vec![(0, 0), (1, 3), (2, 1)],
                &(10, 10),
                resonant_harmonies_sequence_modifier,
            ),
            vec![
                (0, 0),
                (0, 5),
                (1, 3),
                (2, 1),
                (2, 6),
                (3, 9),
                (4, 2),
                (6, 3),
                (8, 4),
            ],
        );
    }

    #[test]
    fn can_count_antinodes_for_map() {
        assert_eq!(
            count_antinodes_for_map(&example_map(), antinode_pair_sequence_modifier),
            14
        );
        assert_eq!(
            count_antinodes_for_map(&example_map(), resonant_harmonies_sequence_modifier),
            34
        );
    }
}
