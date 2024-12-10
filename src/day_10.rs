//! This is my solution for [Advent of Code - Day 10: _Hoof It_](https://adventofcode.com/2024/day/10)
//!
//!

use std::collections::HashSet;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-10-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 10.
pub fn run() {
    let contents = fs::read_to_string("res/day-10-input.txt").expect("Failed to read file");
    let grid = parse_input(&contents);

    println!("The trailhead score is {}", grid.total_score());
    println!("The trailhead rating is {}", grid.total_rating());
}

type Coordinate = (usize, usize);

#[derive(Eq, PartialEq, Debug)]
struct Grid {
    cells: Vec<Vec<u8>>,
}

impl Grid {
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

    fn score_points(&self, cell: Coordinate) -> usize {
        self.get_peaks(cell).len()
    }

    fn get_peaks(&self, cell: Coordinate) -> HashSet<Coordinate> {
        match self.get(cell) {
            Some(9) => vec![cell].into_iter().collect(),
            Some(n) => self
                .adjacent(cell)
                .iter()
                .filter(|(_, val)| *val == n + 1)
                .map(|(coords, _)| self.get_peaks(*coords))
                .reduce(|acc, val| acc.union(&val).cloned().collect())
                .unwrap_or(HashSet::new()),
            None => HashSet::new(),
        }
    }

    fn total_score(&self) -> usize {
        self.trailheads()
            .iter()
            .map(|&trailhead| self.score_points(trailhead))
            .sum()
    }

    fn rate_points(&self, cell: Coordinate) -> usize {
        match self.get(cell) {
            Some(9) => 1,
            Some(n) => self
                .adjacent(cell)
                .iter()
                .filter(|(_, val)| *val == n + 1)
                .map(|(coords, _)| self.rate_points(*coords))
                .sum(),
            None => 0,
        }
    }

    fn total_rating(&self) -> usize {
        self.trailheads()
            .iter()
            .map(|&trailhead| self.rate_points(trailhead))
            .sum()
    }

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

    fn get(&self, (r, c): Coordinate) -> Option<u8> {
        self.cells.get(r).and_then(|row| row.get(c).copied())
    }
}

fn parse_input(input: &String) -> Grid {
    Grid {
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
    
    fn small_example() -> Grid {
        Grid {
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
            Grid {
                cells: vec![
                    vec![0, 1, 2, 3],
                    vec![1, 2, 3, 4],
                    vec![8, 7, 6, 5],
                    vec![9, 8, 7, 6],
                ]
            }
        );
    }

    fn larger_example() -> Grid {
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
        let grid = small_example();

        assert_eq!(
            grid.adjacent((1, 1)),
            vec![((0, 1), 1), ((1, 2), 3), ((2, 1), 7), ((1, 0), 1),]
        );

        assert_eq!(grid.adjacent((0, 0)), vec![((0, 1), 1), ((1, 0), 1),]);

        assert_eq!(
            grid.adjacent((3, 2)),
            vec![((2, 2), 6), ((3, 3), 6), ((3, 1), 8),]
        )
    }

    #[test]
    fn can_score_point() {
        let grid = larger_example();

        assert_eq!(grid.score_points((0, 2)), 5);
    }

    #[test]
    fn can_score_grid() {
        let grid = larger_example();

        assert_eq!(grid.total_score(), 36);
    }

    #[test]
    fn can_rate_cell() {
        let grid = larger_example();

        assert_eq!(grid.rate_points((0, 2)), 20);
    }

    #[test]
    fn can_rate_grid() {
        let grid = larger_example();

        assert_eq!(grid.total_rating(), 81);
    }
}
