//! This is my solution for [Advent of Code - Day 13: _Claw Contraption_](https://adventofcode.com/2024/day/13)
//!
//! [`parse_input`] Uses [`Machine::from_str`] and [`Coords::from_str`] to build the input into a list of [`Machine`]s.
//!
//! [`sum_prize_costs`] solves both parts, taking an offset to be set to 10_000_000_000_000 for part 2. This uses
//! [`Machine::get_cost_for_prize`], and [`Machine::get_presses`] to solve the machine's equations.

use std::fs;
use std::str::FromStr;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-13-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 13.
pub fn run() {
    let contents = fs::read_to_string("res/day-13-input.txt").expect("Failed to read file");
    let machines = parse_input(&contents);

    println!(
        "The total cost for available prizes is {}",
        sum_prize_costs(&machines, 0)
    );

    println!(
        "The total cost for available prizes with offset is {}",
        sum_prize_costs(&machines, 10_000_000_000_000)
    );
}

/// A pair of 2d coordinates. Used for the button press delta's and the prize target
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Coords {
    x: i64,
    y: i64,
}

impl FromStr for Coords {
    type Err = ();

    /// Picks two comma separated numbers out of a line of machine specification, ignoring other characters on the line.
    ///
    /// The following lines parse to (94,34), (22, 67), and (8400, 5400).
    ///
    /// ```text
    /// Button A: X+94, Y+34
    /// Button B: X+22, Y+67
    /// Prize: X=8400, Y=5400
    /// ```
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn parse_part(part: &str) -> i64 {
            part.chars()
                .flat_map(|c| c.to_digit(10))
                .fold(0, |acc, digit| 10 * acc + digit as i64)
        }

        if let Some((_, coords)) = line.split_once(": ") {
            if let Some((x_part, y_part)) = coords.split_once(", ") {
                return Ok(Coords {
                    x: parse_part(x_part),
                    y: parse_part(y_part),
                });
            }
        }

        Err(())
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Machine {
    a: Coords,
    b: Coords,
    prize: Coords,
}

impl FromStr for Machine {
    type Err = ();

    /// Parse a block of machine specification, taking:
    /// * The first line as the delta for button A
    /// * The second line as the delta for button B
    /// * The third line as the position of the prize
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let a = lines.next().and_then(|line| line.parse().ok());
        let b = lines.next().and_then(|line| line.parse().ok());
        let prize = lines.next().and_then(|line| line.parse().ok());

        if let (Some(a), Some(b), Some(prize)) = (a, b, prize) {
            Ok(Machine { a, b, prize })
        } else {
            Err(())
        }
    }
}

impl Machine {
    /// For a machine, solve the pair of linear equations it represents (one in `x` and one in `y`). Due to integer
    /// division, the results may be rounded. Check they actually solve the equation, returning them if they do,
    /// otherwise return `None` as the prize isn't reachable with a whole number of presses.
    ///
    /// See also [Cramer's rule](https://en.wikipedia.org/wiki/Cramer%27s_rule).
    fn get_presses(&self, offset: i64) -> Option<(i64, i64)> {
        let Machine { a, b, prize } = self;

        let nb = (a.y * (prize.x + offset) - a.x * (prize.y + offset)) / (a.y * b.x - a.x * b.y);
        let na = (prize.x + offset - b.x * nb) / a.x;

        // check that a and b have not been rounded
        if na * a.x + nb * b.x == prize.x + offset && na * a.y + nb * b.y == prize.y + offset {
            Some((na, nb))
        } else {
            None
        }
    }

    /// Map the number of button presses for a prize, to its cost in tokens
    fn get_cost_for_prize(&self, offset: i64) -> Option<i64> {
        self.get_presses(offset).map(|(a, b)| 3 * a + b)
    }
}

/// Turn the puzzle input into a list of machines by parsing each block separated by a blank line
fn parse_input(input: &String) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|block| block.parse().unwrap())
        .collect()
}

/// Use [`Machine::get_cost_for_prize`] to get the total cost over all valid machines
fn sum_prize_costs(machines: &Vec<Machine>, offset: i64) -> i64 {
    machines
        .iter()
        .flat_map(|machine| machine.get_cost_for_prize(offset))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_13::*;
    
    fn example_machines() -> Vec<Machine> {
        vec![
            Machine {
                a: Coords { x: 94, y: 34 },
                b: Coords { x: 22, y: 67 },
                prize: Coords { x: 8400, y: 5400 },
            },
            Machine {
                a: Coords { x: 26, y: 66 },
                b: Coords { x: 67, y: 21 },
                prize: Coords { x: 12748, y: 12176 },
            },
            Machine {
                a: Coords { x: 17, y: 86 },
                b: Coords { x: 84, y: 37 },
                prize: Coords { x: 7870, y: 6450 },
            },
            Machine {
                a: Coords { x: 69, y: 23 },
                b: Coords { x: 27, y: 71 },
                prize: Coords { x: 18641, y: 10279 },
            },
        ]
    }

    #[test]
    fn can_parse_input() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"
        .to_string();

        assert_eq!(parse_input(&input), example_machines())
    }

    #[test]
    fn can_get_presses() {
        let machines = example_machines();

        assert_eq!(machines.get(0).unwrap().get_presses(0), Some((80, 40)));
        assert_eq!(machines.get(1).unwrap().get_presses(0), None);
        assert_eq!(machines.get(2).unwrap().get_presses(0), Some((38, 86)));
        assert_eq!(machines.get(3).unwrap().get_presses(0), None);
    }

    #[test]
    fn can_sum_costs() {
        assert_eq!(sum_prize_costs(&example_machines(), 0), 480);
        assert_eq!(
            sum_prize_costs(&example_machines(), 10_000_000_000_000),
            875318608908
        );
    }
}
