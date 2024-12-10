//! This is my solution for [Advent of Code - Day 10: _Hoof It_](https://adventofcode.com/2024/day/10)
//!
//! [`parse_input`] turns the input file into a [`TopographicalMap`], where most of the logic is.
//!
//! [`TopographicalMap::total_score`] solves part 1, and [`TopographicalMap::total_rating`] solves part 2. Both use
//! [`TopographicalMap::trailheads`] to get a list of starting points, which are passed to
//! [`TopographicalMap::score_trailhead`] and [`TopographicalMap::rate_trailhead`] respectively. These both use
//! [`TopographicalMap::get_peaks`] to recursively walk the trail permutations and get a list of peaks that terminate
//! them. The score (part 1) gets the unique peaks before counting them, the rating counts the duplicates.

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-10-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 10.
pub fn run() {
    let contents = fs::read_to_string("res/day-10-input.txt").expect("Failed to read file");
    let topographical_map = parse_input(&contents);

    println!("The trailhead score is {}", topographical_map.total_score());
    println!(
        "The trailhead rating is {}",
        topographical_map.total_rating()
    );
}

type Coordinate = (usize, usize);

/// Represent the map as a list of lists of cells. Most of the business logic for today's puzzles are functions
/// implemented on this struct.
#[derive(Eq, PartialEq, Debug)]
struct TopographicalMap {
    cells: Vec<Vec<u8>>,
}

impl TopographicalMap {
    /// Find all the lowest points (height `0`)
    fn trailheads(&self) -> Vec<Coordinate> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| cell == &&0)
                    .map(move |(c, _)| (r, c))
            })
            .collect()
    }

    /// Get the value at particular coordinates. Returns `None` if the coordinates are outside the bounds of the grid.
    fn get(&self, (r, c): Coordinate) -> Option<u8> {
        self.cells.get(r).and_then(|row| row.get(c).copied())
    }

    /// Return a list of coordinates and heights of orthogonally adjacent cells. Typically, there are four, but cells
    /// on the edge of the [`TopographicalMap`] will return fewer.
    fn adjacent(&self, (r, c): Coordinate) -> Vec<(Coordinate, u8)> {
        [
            r.checked_sub(1).zip(Some(c)),
            Some(r).zip(c.checked_add(1)),
            r.checked_add(1).zip(Some(c)),
            Some(r).zip(c.checked_sub(1)),
        ]
        .into_iter()
        .flatten()
        .flat_map(|coord| Some(coord).zip(self.get(coord)))
        .collect()
    }

    /// Find all valid routes to any peak (height `9`) from a given trailhead, returning the coordinates of those peaks.
    /// Where there are multiple routes up a peak they will be duplicates. A route is valid if each step increases by
    /// 1 unit.
    fn get_peaks(&self, cell: Coordinate) -> Vec<Coordinate> {
        match self.get(cell) {
            Some(9) => vec![cell],
            Some(n) => self
                .adjacent(cell)
                .iter()
                .filter(|(_, val)| *val == n + 1)
                .map(|(coords, _)| self.get_peaks(*coords))
                .reduce(|acc, val| [acc, val].concat())
                .unwrap_or(Vec::new()),
            None => Vec::new(),
        }
    }

    /// Get the count of unique peaks reachable from a given trailhead
    fn score_trailhead(&self, trailhead: Coordinate) -> usize {
        self.get_peaks(trailhead).iter().unique().count()
    }

    /// Solves part 1 - the sum of [`self.score_trailhead`] over all trailheads.
    fn total_score(&self) -> usize {
        self.trailheads()
            .iter()
            .map(|&trailhead| self.score_trailhead(trailhead))
            .sum()
    }

    /// Get the count of valid trails to peaks from a given trailhead
    fn rate_trailhead(&self, cell: Coordinate) -> usize {
        self.get_peaks(cell).iter().count()
    }

    /// Solves part 2 - the sum of [`self.rate_trailhead`] over all trailheads.
    fn total_rating(&self) -> usize {
        self.trailheads()
            .iter()
            .map(|&trailhead| self.rate_trailhead(trailhead))
            .sum()
    }
}

/// Parse the puzzle input into the internal representation
fn parse_input(input: &String) -> TopographicalMap {
    TopographicalMap {
        cells: input
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|c| c.to_digit(10))
                    .map(|num| num as u8)
                    .collect()
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use crate::day_10::*;
    
    fn small_example() -> TopographicalMap {
        TopographicalMap {
            cells: vec![
                vec![0, 1, 2, 3],
                vec![1, 2, 3, 4],
                vec![8, 7, 6, 5],
                vec![9, 8, 7, 6],
            ],
        }
    }

    #[test]
    fn can_parse_input() {
        let input = "0123
1234
8765
9876"
            .to_string();

        assert_eq!(
            parse_input(&input),
            TopographicalMap {
                cells: vec![
                    vec![0, 1, 2, 3],
                    vec![1, 2, 3, 4],
                    vec![8, 7, 6, 5],
                    vec![9, 8, 7, 6],
                ]
            }
        );
    }

    fn larger_example() -> TopographicalMap {
        parse_input(
            &"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
                .to_string(),
        )
    }

    #[test]
    fn can_find_trailheads() {
        assert_eq!(small_example().trailheads(), vec![(0, 0)]);

        assert_eq!(
            larger_example().trailheads(),
            vec![
                (0, 2),
                (0, 4),
                (2, 4),
                (4, 6),
                (5, 2),
                (5, 5),
                (6, 0),
                (6, 6),
                (7, 1)
            ]
        )
    }

    #[test]
    fn can_find_adjacent_cells() {
        let topographical_map = small_example();

        assert_eq!(
            topographical_map.adjacent((1, 1)),
            vec![((0, 1), 1), ((1, 2), 3), ((2, 1), 7), ((1, 0), 1),]
        );

        assert_eq!(
            topographical_map.adjacent((0, 0)),
            vec![((0, 1), 1), ((1, 0), 1),]
        );

        assert_eq!(
            topographical_map.adjacent((3, 2)),
            vec![((2, 2), 6), ((3, 3), 6), ((3, 1), 8),]
        )
    }

    #[test]
    fn can_score_trailhead() {
        assert_eq!(larger_example().score_trailhead((0, 2)), 5);
    }

    #[test]
    fn can_score_topographical_map() {
        assert_eq!(larger_example().total_score(), 36);
    }

    #[test]
    fn can_rate_trailhead() {
        assert_eq!(larger_example().rate_trailhead((0, 2)), 20);
    }

    #[test]
    fn can_rate_topographical_map() {
        assert_eq!(larger_example().total_rating(), 81);
    }
}
