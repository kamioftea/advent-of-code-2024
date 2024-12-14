//! This is my solution for [Advent of Code - Day 14: _Restroom Redoubt_](https://adventofcode.com/2024/day/14)
//!
//! [`parse_input`] uses [`Robot::from_str`] to build up a list of [`Robot`]s.
//!
//! [`total_safety_factor_after_steps`] is used to solve part 1, delegating to [`simulate_robots`] and
//! [`total_safety_factor`] which groups robots into [`Quadrant`]s and calculates the product.
//!
//! [`guess_tree_seconds`] uses [`iterate_seconds`] to loop through all the possible positions of the robots, find
//! the one with the lowest [`total_safety_factor`] as a proxy for the robots clustering into a tree.
//! [`render_robots`] can be used to show the robot's current position visually

use crate::day_14::Quadrant::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::iter::successors;
use std::str::FromStr;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-14-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 14.
pub fn run() {
    let contents = fs::read_to_string("res/day-14-input.txt").expect("Failed to read file");
    let robots = parse_input(&contents);

    let bounds = (103, 101);
    println!(
        "The total safety factor after 100 steps is {}",
        total_safety_factor_after_steps(&robots, 100, &bounds)
    );

    println!(
        "The tree is formed after {} seconds",
        guess_tree_seconds(&robots, &bounds)
    );
}

/// A robot's position on the grid (row, column)
type Position = (usize, usize);
/// The speed a robot is travelling along each axis (row, column)
type Velocity = (isize, isize);

/// The four areas of the grid used to calculate the `total_safety_factor`
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// A robot patrolling the grid
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity }
    }
}

impl FromStr for Robot {
    type Err = ();

    /// Expected format `p=10,5 v=-1,2`
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn parse_part<T>(part: &str) -> (T, T)
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            let (_, values) = part.split_once("=").unwrap();
            // Coordinates are x, y
            let (c, r) = values.split_once(",").unwrap();
            (r.parse::<T>().unwrap(), c.parse::<T>().unwrap())
        }

        let (position, velocity) = line.split_once(" ").ok_or(())?;

        Ok(Robot::new(parse_part(position), parse_part(velocity)))
    }
}

impl Robot {
    /// Calculate the robot's position after `steps` seconds on the provided grid
    fn simulate(&self, steps: usize, &(max_r, max_c): &(usize, usize)) -> Robot {
        let Robot {
            position: (r, c),
            velocity: (dr, dc),
        } = *self;

        let dr = ((dr % max_r as isize) + max_r as isize) as usize;
        let dc = ((dc % max_c as isize) + max_c as isize) as usize;

        let new_pos = ((r + dr * steps) % max_r, (c + dc * steps) % max_c);

        Robot::new(new_pos, self.velocity)
    }
}

/// Turn the input file into the internal representation
fn parse_input(input: &String) -> Vec<Robot> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// Calculate the position of all robots after `steps` seconds
fn simulate_robots(robots: &Vec<Robot>, steps: usize, bounds: &(usize, usize)) -> Vec<Robot> {
    robots
        .iter()
        .map(|robot| robot.simulate(steps, bounds))
        .collect()
}

/// For a given position, calculate which quadrant of the grid it is on. If the position is on one of the centre lines
/// of the grid, this returns `None`
fn partition_position((r, c): Position, (max_r, max_c): &(usize, usize)) -> Option<Quadrant> {
    let mid_r = max_r / 2;
    let mid_c = max_c / 2;

    if r < mid_r && c < mid_c {
        Some(TopLeft)
    } else if r < mid_r && c > mid_c {
        Some(TopRight)
    } else if r > mid_r && c < mid_c {
        Some(BottomLeft)
    } else if r > mid_r && c > mid_c {
        Some(BottomRight)
    } else {
        None
    }
}

/// The product of the count of robots in each of the four quadrants
fn total_safety_factor(robots: &Vec<Robot>, bounds: &(usize, usize)) -> usize {
    let counts = robots
        .iter()
        .flat_map(|&robot| partition_position(robot.position, bounds))
        .counts();

    [TopLeft, TopRight, BottomLeft, BottomRight]
        .iter()
        .fold(1, |acc, quadrant| acc * counts.get(quadrant).unwrap_or(&0))
}

/// The solution to part 1 - simulate `steps` seconds and then get the `total_safety_factor` of the new positions.
fn total_safety_factor_after_steps(
    robots: &Vec<Robot>,
    steps: usize,
    bounds: &(usize, usize),
) -> usize {
    let positions = simulate_robots(robots, steps, bounds);
    total_safety_factor(&positions, bounds)
}

/// An iterator over the unique arrangements of the robots on the grid
fn iterate_seconds<'a>(
    robots: &'a Vec<Robot>,
    bounds: &'a (usize, usize),
) -> impl Iterator<Item = Vec<Robot>> + 'a {
    let first = robots.clone();

    successors(Some(robots.clone()), move |robots| {
        let next = simulate_robots(robots, 1, bounds);
        if next != first {
            Some(next)
        } else {
            None
        }
    })
}

/// Guesses which second shows the image by finding which has the lowest `total_safety_factor`
fn guess_tree_seconds(robots: &Vec<Robot>, bounds: &(usize, usize)) -> usize {
    let (pos, _) = iterate_seconds(robots, bounds)
        .enumerate()
        .min_by_key(|(_, robots)| total_safety_factor(robots, bounds))
        .unwrap();

    pos
}

#[allow(dead_code)]
/// Render the position of the robots on an ascii art grid.
fn render_robots(robots: &Vec<Robot>, &(r_max, c_max): &(usize, usize), show_middle_lines: bool) {
    robots.iter().for_each(
        |Robot {
             position: (r, c),
             velocity: (dr, dc),
         }| println!("p={c},{r} v={dc},{dr}"),
    );

    let positions: HashSet<Position> = robots.iter().map(|robot| robot.position).collect();
    let r_mid = r_max / 2;
    let c_mid = c_max / 2;

    println!(
        "+{0}{1}{0}+",
        "-".repeat(c_mid),
        if show_middle_lines { "+" } else { "-" }
    );

    for r in 0..r_max {
        print!(
            "{}",
            if show_middle_lines && r == r_mid {
                "+"
            } else {
                "|"
            }
        );
        for c in 0..c_max {
            if show_middle_lines && r == r_mid {
                if c == c_mid {
                    print!("+")
                } else {
                    print!("-")
                }
            } else if show_middle_lines && c == { c_mid } {
                print!("|")
            } else if positions.contains(&(r, c)) {
                print!("#")
            } else {
                print!(" ");
            }
        }
        println!(
            "{}",
            if show_middle_lines && r == r_mid {
                "+"
            } else {
                "|"
            }
        );
    }
    println!(
        "+{0}{1}{0}+",
        "-".repeat(c_mid),
        if show_middle_lines { "+" } else { "-" }
    );
}

#[cfg(test)]
mod tests {
    use crate::day_14::*;
    use crate::helpers::test::assert_contains_in_any_order;
    
    fn example_robots() -> Vec<Robot> {
        vec![
            Robot::new((4, 0), (-3, 3)),
            Robot::new((3, 6), (-3, -1)),
            Robot::new((3, 10), (2, -1)),
            Robot::new((0, 2), (-1, 2)),
            Robot::new((0, 0), (3, 1)),
            Robot::new((0, 3), (-2, -2)),
            Robot::new((6, 7), (-3, -1)),
            Robot::new((0, 3), (-2, -1)),
            Robot::new((3, 9), (3, 2)),
            Robot::new((3, 7), (2, -1)),
            Robot::new((4, 2), (-3, 2)),
            Robot::new((5, 9), (-3, -3)),
        ]
    }

    fn positions_after_100_steps() -> Vec<(usize, usize)> {
        vec![
            (0, 6),
            (0, 6),
            (0, 9),
            (2, 0),
            (3, 1),
            (3, 2),
            (4, 5),
            (5, 3),
            (5, 4),
            (5, 4),
            (6, 1),
            (6, 6),
        ]
    }

    #[test]
    fn can_parse_input() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
        .to_string();

        assert_eq!(parse_input(&input), example_robots())
    }

    #[test]
    fn can_simulate_robot() {
        let robot = Robot::new((4, 2), (-3, 2));
        assert_eq!(robot.simulate(0, &(7, 11)).position, (4, 2));
        assert_eq!(robot.simulate(1, &(7, 11)).position, (1, 4));
        assert_eq!(robot.simulate(2, &(7, 11)).position, (5, 6));
        assert_eq!(robot.simulate(3, &(7, 11)).position, (2, 8));
        assert_eq!(robot.simulate(4, &(7, 11)).position, (6, 10));
        assert_eq!(robot.simulate(5, &(7, 11)).position, (3, 1));
    }

    #[test]
    fn can_simulate_robots() {
        let positions: Vec<Position> = simulate_robots(&example_robots(), 100, &(7, 11))
            .iter()
            .map(|r| r.position)
            .collect();

        assert_eq!(
            positions.iter().filter(|&&p| p == (0, 6)).count(),
            2,
            "There should be two robots at position (0,6)"
        );
        assert_eq!(
            positions.iter().filter(|&&p| p == (5, 4)).count(),
            2,
            "There should be two robots at position (5, 4)"
        );

        assert_contains_in_any_order(positions, positions_after_100_steps());
    }

    #[test]
    fn can_partition_robots() {
        assert_eq!(
            positions_after_100_steps()
                .into_iter()
                .map(|pos| partition_position(pos, &(7, 11)))
                .collect::<Vec<Option<Quadrant>>>(),
            vec![
                Some(TopRight),
                Some(TopRight),
                Some(TopRight),
                Some(TopLeft),
                None,
                None,
                None,
                Some(BottomLeft),
                Some(BottomLeft),
                Some(BottomLeft),
                Some(BottomLeft),
                Some(BottomRight),
            ]
        );
    }

    #[test]
    fn can_calculate_total_safety_factor() {
        assert_eq!(
            total_safety_factor_after_steps(&example_robots(), 100, &(7, 11)),
            12
        );
        assert_eq!(
            total_safety_factor_after_steps(&example_robots(), 0, &(7, 11)),
            0
        )
    }

    fn tree_example_robots() -> Vec<Robot> {
        parse_input(
            &"p=5,4 v=-2,2
p=9,4 v=-1,-1
p=10,5 v=-1,2
p=4,4 v=2,-1
p=6,1 v=3,1
p=2,4 v=2,3
p=10,2 v=-1,-3
p=0,0 v=-1,-2
p=2,6 v=-3,2
p=10,6 v=-1,-1
p=10,1 v=-2,3
p=0,6 v=2,4
p=4,6 v=3,2
p=4,1 v=2,-1
p=4,2 v=-3,-2
p=5,0 v=-1,-2
p=7,0 v=-3,4
p=6,1 v=-2,2"
                .to_string(),
        )
    }

    #[test]
    fn can_find_frame_with_lowest_safety_factor() {
        assert_eq!(guess_tree_seconds(&tree_example_robots(), &(7, 11)), 72);
    }
}
