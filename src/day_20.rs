//! This is my solution for [Advent of Code - Day 20: _Race Condition_](https://adventofcode.com/2024/day/20)
//!
//!

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
        "There are {} cheats of at least 100 picoseconds",
        track.count_cheats_from(100)
    );
}

type Coordinates = (usize, usize);

trait CoordinateExtensions: Sized {
    fn apply(&self, delta: &(isize, isize)) -> Option<Self>;
}

impl CoordinateExtensions for Coordinates {
    fn apply(&self, delta: &(isize, isize)) -> Option<Self> {
        let (r, c) = self;
        let (dr, dc) = delta;

        let r1 = r.checked_add_signed(*dr);
        let c1 = c.checked_add_signed(*dc);

        r1.zip(c1)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct RaceTrack {
    course: HashSet<Coordinates>,
    start: Coordinates,
    end: Coordinates,
}

impl RaceTrack {
    fn cheats(&self) -> HashMap<(Coordinates, Coordinates), usize> {
        let mut visited = HashMap::new();
        let mut cheats = HashMap::new();
        let mut position = self.start;

        for index in 0.. {
            visited.insert(position, index);
            let mut next = position;
            for delta in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let maybe_adjacent = position.apply(&delta);
                let maybe_next = maybe_adjacent.filter(|coords| self.course.contains(coords));

                if maybe_adjacent.is_some() && maybe_next.is_none() {
                    let adjacent = maybe_adjacent.unwrap();
                    let maybe_cheat = adjacent
                        .apply(&delta)
                        .and_then(|coords| Some(coords).zip(visited.get(&coords)));
                    if let Some((start_pos, &start_index)) = maybe_cheat {
                        cheats.insert((start_pos, position), index - start_index - 2);
                    }
                }

                let without_visited = maybe_next.filter(|coords| !visited.contains_key(coords));
                if without_visited.is_some() {
                    next = without_visited.unwrap();
                }
            }

            if position == self.end {
                break;
            }

            if next == position {
                unreachable!("{position:?} failed to find next position")
            }

            position = next;
        }

        cheats
    }

    fn count_cheats_from(&self, threshold: usize) -> usize {
        self.cheats()
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
        let cheats = example_track().cheats();

        assert_eq!(cheats.get(&((1, 7), (1, 9))), Some(&12));
        assert_eq!(cheats.get(&((7, 9), (7, 11))), Some(&20));
        assert_eq!(cheats.get(&((7, 8), (9, 8))), Some(&38));
        assert_eq!(cheats.get(&((7, 7), (7, 5))), Some(&64));

        assert_eq!(cheats.len(), 44);
    }

    #[test]
    fn can_count_significant_cheats() {
        let track = example_track();

        assert_eq!(track.count_cheats_from(4), 30);
        assert_eq!(track.count_cheats_from(15), 5);
    }
}
