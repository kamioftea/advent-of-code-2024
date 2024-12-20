//! This is my solution for [Advent of Code - Day 20: _Race Condition_](https://adventofcode.com/2024/day/20)
//!
//!

use std::collections::HashSet;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-20-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 20.
pub fn run() {
    let _contents = fs::read_to_string("res/day-20-input.txt").expect("Failed to read file");
}

type Coordinates = (usize, usize);

#[derive(Eq, PartialEq, Debug)]
struct RaceTrack {
    course: HashSet<Coordinates>,
    start: Coordinates,
    end: Coordinates,
}

fn parse_input(input: &String) -> RaceTrack {
    let mut course = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (r, row) in input.lines().enumerate() {
        for (c, char) in row.chars().enumerate() {
            match char {
                '.' => {
                    course.insert((r, c));
                }
                'S' => {
                    course.insert((r, c));
                    start = (r, c);
                }
                'E' => {
                    course.insert((r, c));
                    end = (r, c);
                }
                _ => {}
            }
        }
    }

    RaceTrack { course, start, end }
}

#[cfg(test)]
mod tests {
    use crate::day_20::*;

    fn example_track() -> RaceTrack {
        #[rustfmt::skip]
        let course = vec![
        ( 1, 1),( 1, 2),( 1, 3),        ( 1, 5),( 1, 6),( 1, 7),        ( 1, 9),( 1,10),( 1,11),( 1,12),( 1,13),
        ( 2, 1),        ( 2, 3),        ( 2, 5),        ( 2, 7),        ( 2, 9),                        ( 2,13),
        ( 3, 1),        ( 3, 3),( 3, 4),( 3, 5),        ( 3, 7),        ( 3, 9),        ( 3,11),( 3,12),( 3,13),
                                                        ( 4, 7),        ( 4, 9),        ( 4,11),
                                                        ( 5, 7),        ( 5, 9),        ( 5,11),( 5,12),( 5,13),
                                                        ( 6, 7),        ( 6, 9),                        ( 6,13),
                        ( 7, 3),( 7, 4),( 7, 5),        ( 7, 7),( 7, 8),( 7, 9),        ( 7,11),( 7,12),( 7,13),
                        ( 8, 3),                                                        ( 8,11),
        ( 9, 1),( 9, 2),( 9, 3),                        ( 9, 7),( 9, 8),( 9, 9),        ( 9,11),( 9,12),( 9,13),
        (10, 1),                                        (10, 7),        (10, 9),                        (10,13),
        (11, 1),        (11, 3),(11, 4),(11, 5),        (11, 7),        (11, 9),        (11,11),(11,12),(11,13),
        (12, 1),        (12, 3),        (12, 5),        (12, 7),        (12, 9),        (12,11),
        (13, 1),(13, 2),(13, 3),        (13, 5),(13, 6),(13, 7),        (13, 9),(13,10),(13,11),
        ].into_iter().collect();

        RaceTrack {
            course,
            start: (3, 1),
            end: (7, 5),
        }
    }

    #[test]
    fn can_parse_input() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"
        .to_string();

        assert_eq!(parse_input(&input), example_track());
    }
}
