//! This is my solution for [Advent of Code - Day 20: _Race Condition_](https://adventofcode.com/2024/day/20)
//!
//!

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-20-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 20.
pub fn run() {
    let contents = fs::read_to_string("res/day-20-input.txt").expect("Failed to read file");
    let track = parse_input(&contents);

    println!(
        "There are {} cheats of length 2 that save at least 100 picoseconds",
        track.count_cheats_from(100, 2)
    );

    println!(
        "There are {} cheats of length up to 20 that save at least 100 picoseconds",
        track.count_cheats_from(100, 20)
    );
}

type Coordinates = (usize, usize);

trait CoordinateExtensions: Sized {
    fn apply(&self, delta: &(isize, isize)) -> Option<Self>;
    fn manhattan_distance(&self, other: &Self) -> usize;
}

impl CoordinateExtensions for Coordinates {
    fn apply(&self, delta: &(isize, isize)) -> Option<Self> {
        let (r, c) = self;
        let (dr, dc) = delta;

        let r1 = r.checked_add_signed(*dr);
        let c1 = c.checked_add_signed(*dc);

        r1.zip(c1)
    }

    /// [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points
    fn manhattan_distance(&self, other: &Self) -> usize {
        let (r0, c0) = self;
        let (r1, c1) = other;

        r0.abs_diff(*r1) + c0.abs_diff(*c1)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct RaceTrack {
    course: HashSet<Coordinates>,
    start: Coordinates,
    end: Coordinates,
}

impl RaceTrack {
    fn get_track_positions(&self) -> Vec<(usize, Coordinates)> {
        let mut visited = Vec::new();
        let mut position = self.start;
        let mut prev = self.start;

        for index in 0.. {
            visited.push((index, position));
            if position == self.end {
                break;
            }

            for delta in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                if let Some(next) = position
                    .apply(&delta)
                    .filter(|coords| self.course.contains(coords))
                    .filter(|coords| coords != &prev)
                {
                    prev = position;
                    position = next;
                    break;
                }
            }
        }

        visited
    }

    fn cheats(&self, max_cheat: usize) -> HashMap<(Coordinates, Coordinates), usize> {
        let track = self.get_track_positions();

        track
            .iter()
            .tuple_combinations()
            .flat_map(|(&(start_idx, start_coord), &(end_idx, end_coord))| {
                let manhattan_distance = start_coord.manhattan_distance(&end_coord);
                if manhattan_distance > max_cheat {
                    None
                } else {
                    Some((start_coord, end_coord))
                        .zip((end_idx - start_idx).checked_sub(manhattan_distance))
                        .filter(|&(_, distance)| distance > 0)
                }
            })
            .collect()
    }

    fn count_cheats_from(&self, threshold: usize, max_cheat: usize) -> usize {
        self.cheats(max_cheat)
            .iter()
            .map(|(_, saving)| saving)
            .filter(|&&saving| saving >= threshold)
            .count()
    }
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

    #[test]
    fn can_list_cheats() {
        let cheats = example_track().cheats(2);

        assert_eq!(cheats.get(&((1, 7), (1, 9))), Some(&12));
        assert_eq!(cheats.get(&((7, 9), (7, 11))), Some(&20));
        assert_eq!(cheats.get(&((7, 8), (9, 8))), Some(&38));
        assert_eq!(cheats.get(&((7, 7), (7, 5))), Some(&64));

        assert_eq!(cheats.len(), 44);
    }

    #[test]
    fn can_count_significant_cheats() {
        let track = example_track();

        assert_eq!(track.count_cheats_from(4, 2), 30);
        assert_eq!(track.count_cheats_from(15, 2), 5);

        assert_eq!(track.count_cheats_from(50, 20), 285);
        assert_eq!(track.count_cheats_from(72, 20), 29);
    }

    #[test]
    fn can_find_track() {
        let track = example_track();
        let positions = track.get_track_positions();

        assert_eq!(positions.len(), track.course.len());
        assert_eq!(positions[0], (0, (3, 1)));
        assert_eq!(positions[84], (84, (7, 5)));
    }
}
