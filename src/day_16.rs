//! This is my solution for [Advent of Code - Day 16: _Reindeer Maze_](https://adventofcode.com/2024/day/16)
//!
//!

use crate::day_16::Facing::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-16-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 16.
pub fn run() {
    let contents = fs::read_to_string("res/day-16-input.txt").expect("Failed to read file");
    let maze = parse_input(&contents);

    println!(
        "The lowest scoring route scores {}",
        maze.lowest_scoring_route()
    );

    println!(
        "There are {} tiles on the best routes",
        maze.count_visited_by_best_routes()
    )
}

type Coordinates = (usize, usize);

trait ManhattanDistance {
    fn manhattan_distance(&self, other: &Self) -> usize;
}

impl ManhattanDistance for Coordinates {
    fn manhattan_distance(&self, other: &Self) -> usize {
        let (r0, c0) = self;
        let (r1, c1) = other;

        r0.abs_diff(*r1) + c0.abs_diff(*c1)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Maze {
    hedges: HashSet<Coordinates>,
    start: Coordinates,
    end: Coordinates,
    bounds: (usize, usize),
}

impl Maze {
    fn lowest_scoring_route(&self) -> u32 {
        let mut heap: BinaryHeap<Position> = BinaryHeap::new();
        let mut visited = HashSet::new();
        heap.push(self.starting_position());

        while let Some(curr) = heap.pop() {
            if curr.coordinates == self.end {
                return curr.score;
            }

            for next in curr.next(self) {
                if visited.insert((next.coordinates, next.facing)) {
                    heap.push(next);
                }
            }
        }

        unreachable!("Failed to find route to end");
    }
    fn count_visited_by_best_routes(&self) -> usize {
        let mut heap: BinaryHeap<Position> = BinaryHeap::new();
        let mut visited: HashMap<(Coordinates, Facing), u32> = HashMap::new();
        let mut lowest_score = u32::MAX;
        let mut routes = Vec::new();

        heap.push(self.starting_position());

        while let Some(curr) = heap.pop() {
            if curr.coordinates == self.end {
                if curr.score < lowest_score {
                    lowest_score = curr.score;
                    routes = Vec::new();
                }

                if curr.score == lowest_score {
                    routes.push(curr.visited.clone())
                }
            }

            for next in curr.next(self) {
                if curr.score < lowest_score
                    && !visited
                        .get(&(next.coordinates, next.facing))
                        .is_some_and(|&s| s <= curr.score)
                {
                    visited.insert((next.coordinates, next.facing), next.score);
                    heap.push(next);
                }
            }
        }

        routes.iter().flatten().unique().count()
    }

    fn starting_position(&self) -> Position {
        Position::new(
            self.start.clone(),
            East,
            0,
            self.start.manhattan_distance(&self.end),
            vec![self.start.clone()],
        )
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    fn rotate_clockwise(&self) -> Facing {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn rotate_counterclockwise(&self) -> Facing {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn forwards(&self, (r, c): &Coordinates, maze: &Maze) -> Option<Coordinates> {
        let (dr, dc) = match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        };

        let (max_r, max_c) = maze.bounds;

        let r1 = r.checked_add_signed(dr).filter(|&r| r < max_r);
        let c1 = c.checked_add_signed(dc).filter(|&c| c < max_c);

        r1.zip(c1).filter(|coords| !maze.hedges.contains(coords))
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Position {
    coordinates: Coordinates,
    facing: Facing,
    score: u32,
    distance: usize,
    visited: Vec<Coordinates>,
}

impl Position {
    fn next(&self, maze: &Maze) -> Vec<Position> {
        vec![
            Some(Position {
                facing: self.facing.rotate_clockwise(),
                score: self.score + 1000,
                ..self.clone()
            }),
            Some(Position {
                facing: self.facing.rotate_counterclockwise(),
                score: self.score + 1000,
                ..self.clone()
            }),
            self.facing
                .forwards(&self.coordinates, maze)
                .map(|coordinates| Position {
                    coordinates,
                    score: self.score + 1,
                    distance: coordinates.manhattan_distance(&maze.end),
                    facing: self.facing,
                    visited: [self.visited.clone(), vec![coordinates]].concat(),
                }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn new(
        coordinates: Coordinates,
        facing: Facing,
        score: u32,
        distance: usize,
        visited: Vec<Coordinates>,
    ) -> Self {
        Self {
            coordinates,
            facing,
            score,
            distance,
            visited,
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| other.distance.cmp(&self.distance))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &String) -> Maze {
    let mut hedges = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut max_r = 0;
    let mut max_c = 0;

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
            max_c = max_c.max(c);
        }
        max_r = max_r.max(r);
    }

    Maze {
        hedges,
        start,
        end,
        bounds: (max_r + 1, max_c + 1),
    }
}

#[cfg(test)]
mod tests {
    use crate::day_16::*;
    use crate::helpers::test::assert_contains_in_any_order;
    
    fn example_maze() -> Maze {
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

        Maze {
            hedges,
            start: (13, 1),
            end: (1, 13),
            bounds: (15, 15),
        }
    }

    fn larger_example_maze() -> Maze {
        parse_input(
            &"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
                .to_string(),
        )
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

        assert_eq!(parse_input(&input), example_maze())
    }

    #[test]
    fn can_get_manhattan_distance() {
        assert_eq!((0, 0).manhattan_distance(&(0, 0)), 0);
        assert_eq!((13, 1).manhattan_distance(&(1, 13)), 24);
        assert_eq!((13, 2).manhattan_distance(&(1, 13)), 23);
    }

    #[test]
    fn can_get_next_moves() {
        let maze = example_maze();
        let start = example_maze().starting_position();
        let expected = vec![
            Position::new((13, 1), South, 1000, 24, vec![(13, 1)]),
            Position::new((13, 1), North, 1000, 24, vec![(13, 1)]),
            Position::new((13, 2), East, 1, 23, vec![(13, 1), (13, 2)]),
        ];

        assert_contains_in_any_order(start.next(&maze), expected);

        let start = Position::new(
            (9, 1),
            North,
            4,
            20,
            vec![(13, 1), (12, 1), (11, 1), (10, 1), (9, 1)],
        );
        let expected = vec![
            Position::new(
                (9, 1),
                East,
                1004,
                20,
                vec![(13, 1), (12, 1), (11, 1), (10, 1), (9, 1)],
            ),
            Position::new(
                (9, 1),
                West,
                1004,
                20,
                vec![(13, 1), (12, 1), (11, 1), (10, 1), (9, 1)],
            ),
        ];

        assert_contains_in_any_order(start.next(&maze), expected);
    }

    #[test]
    fn can_navigate_maze() {
        assert_eq!(example_maze().lowest_scoring_route(), 7036);
        assert_eq!(larger_example_maze().lowest_scoring_route(), 11048);
    }

    #[test]
    fn can_find_visited_tiles() {
        assert_eq!(example_maze().count_visited_by_best_routes(), 45);
        assert_eq!(larger_example_maze().count_visited_by_best_routes(), 64);
    }
}
