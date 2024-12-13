//! This is my solution for [Advent of Code - Day 13: _Claw Contraption_](https://adventofcode.com/2024/day/13)
//!
//!

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
        sum_prize_costs(&machines)
    );
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Coords {
    x: i32,
    y: i32,
}

impl FromStr for Coords {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn parse_part(part: &str) -> i32 {
            part.chars()
                .flat_map(|c| c.to_digit(10))
                .fold(0, |acc, digit| 10 * acc + digit as i32)
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let a = lines.next().and_then(|line| line.parse().ok());
        let b = lines.next().and_then(|line| line.parse().ok());
        let prize = lines.next().and_then(|line| line.parse().ok());
        if let (Some(a), Some(b), Some(prize)) = (a, b, prize) {
            return Ok(Machine { a, b, prize });
        }

        Err(())
    }
}

impl Machine {
    fn get_presses(&self) -> Option<(i32, i32)> {
        let b = (self.a.y * self.prize.x - self.a.x * self.prize.y)
            / (self.a.y * self.b.x - self.a.x * self.b.y);
        let a = (self.prize.x - self.b.x * b) / self.a.x;

        if a * self.a.x + b * self.b.x == self.prize.x
            && a * self.a.y + b * self.b.y == self.prize.y
        {
            Some((a, b))
        } else {
            None
        }
    }

    fn get_cost_for_prize(&self) -> Option<i32> {
        self.get_presses().map(|(a, b)| 3 * a + b)
    }
}

fn parse_input(input: &String) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|block| block.parse().unwrap())
        .collect()
}

fn sum_prize_costs(machines: &Vec<Machine>) -> i32 {
    machines.iter().flat_map(Machine::get_cost_for_prize).sum()
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

        assert_eq!(machines.get(0).unwrap().get_presses(), Some((80, 40)));
        assert_eq!(machines.get(1).unwrap().get_presses(), None);
        assert_eq!(machines.get(2).unwrap().get_presses(), Some((38, 86)));
        assert_eq!(machines.get(3).unwrap().get_presses(), None);
    }

    #[test]
    fn can_sum_costs() {
        assert_eq!(sum_prize_costs(&example_machines()), 480)
    }
}
