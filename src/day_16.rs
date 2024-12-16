//! This is my solution for [Advent of Code - Day 16: _Reindeer Maze_](https://adventofcode.com/2024/day/16)
//!
//!

use std::collections::HashSet;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-16-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 16.
pub fn run() {
    let _contents = fs::read_to_string("res/day-16-input.txt").expect("Failed to read file");
}

type Coordinate = (usize, usize);

#[derive(Eq, PartialEq, Debug)]
struct Maze {
    hedges: HashSet<Coordinate>,
    start: Coordinate,
    end: Coordinate,
}

fn parse_input(input: &String) -> Maze {
    let mut hedges = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (r, row) in input.lines().enumerate() {
        for (c, char) in row.chars().enumerate() {
            match char {
                '#' => {
                    hedges.insert((r, c));
                }
                'S' => start = (r, c),
                'E' => end = (r, c),
                _ => {}
            }
        }
    }

    Maze { hedges, start, end }
}

#[cfg(test)]
mod tests {
    use crate::day_16::*;
    use crate::helpers::test::assert_contains_in_any_order;
    
    fn example_maze() -> Maze {
        // @formatter:off
        #[rustfmt::skip]
        let hedges = vec![
            ( 0,  0), ( 0,  1), ( 0,  2), ( 0,  3), ( 0,  4), ( 0,  5), ( 0,  6), ( 0,  7), ( 0,  8), ( 0,  9), ( 0, 10), ( 0, 11), ( 0, 12), ( 0, 13), ( 0, 14),
            ( 1,  0),                                                                       ( 1,  8),                                                   ( 1, 14),
            ( 2,  0),           ( 2,  2),           ( 2,  4), ( 2,  5), ( 2,  6),           ( 2,  8),           ( 2, 10), ( 2, 11), ( 2, 12),           ( 2, 14),
            ( 3,  0),                                                   ( 3,  6),           ( 3,  8),                               ( 3, 12),           ( 3, 14),
            ( 4,  0),           ( 4,  2), ( 4,  3), ( 4,  4),           ( 4,  6), ( 4,  7), ( 4,  8), ( 4,  9), ( 4, 10),           ( 4, 12),           ( 4, 14),
            ( 5,  0),           ( 5,  2),           ( 5,  4),                                                                       ( 5, 12),           ( 5, 14),
            ( 6,  0),           ( 6,  2),           ( 6,  4), ( 6,  5), ( 6,  6), ( 6,  7), ( 6,  8),           ( 6, 10), ( 6, 11), ( 6, 12),           ( 6, 14),
            ( 7,  0),                                                                                                               ( 7, 12),           ( 7, 14),
            ( 8,  0), ( 8,  1), ( 8,  2),           ( 8,  4),           ( 8,  6), ( 8,  7), ( 8,  8), ( 8,  9), ( 8, 10),           ( 8, 12),           ( 8, 14),
            ( 9,  0),                               ( 9,  4),                                                   ( 9, 10),           ( 9, 12),           ( 9, 14),
            (10,  0),           (10,  2),           (10,  4),           (10,  6), (10,  7), (10,  8),           (10, 10),           (10, 12),           (10, 14),
            (11,  0),                                                   (11,  6),                               (11, 10),           (11, 12),           (11, 14),
            (12,  0),           (12,  2), (12,  3), (12,  4),           (12,  6),           (12,  8),           (12, 10),           (12, 12),           (12, 14),
            (13,  0),                               (13,  4),                                                   (13, 10),                               (13, 14),
            (14,  0), (14,  1), (14,  2), (14,  3), (14,  4), (14,  5), (14,  6), (14,  7), (14,  8), (14,  9), (14, 10), (14, 11), (14, 12), (14, 13), (14, 14),
        ].into_iter().collect();
        // @formatter:on

        Maze {
            hedges,
            start: (13, 1),
            end: (1, 13),
        }
    }

    #[test]
    fn can_parse_input() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            .to_string();

        assert_contains_in_any_order(parse_input(&input).hedges, example_maze().hedges);

        assert_eq!(parse_input(&input), example_maze())
    }
}
