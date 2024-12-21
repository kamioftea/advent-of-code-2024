//! This is my solution for [Advent of Code - Day 21: _Keypad Conundrum_](https://adventofcode.com/2024/day/21)
//!
//!

use crate::day_21::DirectionButton::*;
use crate::day_21::KeyPadButton::*;
use crate::day_21::NumberButton::*;
use itertools::{chain, Itertools};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::iter::once;
use std::rc::Rc;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-21-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 21.
pub fn run() {
    let contents = fs::read_to_string("res/day-21-input.txt").expect("Failed to read file");
    let codes = parse_input(&contents);

    println!(
        "To open the first door takes {} key presses",
        count_key_presses(&codes, &mut keypad_chain(2))
    );
    println!(
        "To open the second door takes {} key presses",
        count_key_presses(&codes, &mut keypad_chain(25))
    );
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum DirectionButton {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum NumberButton {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<char> for NumberButton {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Zero),
            '1' => Ok(One),
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            '9' => Ok(Nine),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum KeyPadButton<T> {
    Input(T),
    A,
}

trait Keys<T> {
    fn coordinate(key: KeyPadButton<T>) -> Coordinates;
    fn contains(coord: &Coordinates) -> bool;
}

impl Keys<NumberButton> for NumberButton {
    fn coordinate(key: KeyPadButton<NumberButton>) -> Coordinates {
        match key {
            Input(Zero) => (3, 1),
            Input(One) => (2, 0),
            Input(Two) => (2, 1),
            Input(Three) => (2, 2),
            Input(Four) => (1, 0),
            Input(Five) => (1, 1),
            Input(Six) => (1, 2),
            Input(Seven) => (0, 0),
            Input(Eight) => (0, 1),
            Input(Nine) => (0, 2),
            A => (3, 2),
        }
    }

    fn contains(coord: &Coordinates) -> bool {
        match coord {
            &(3, 0) => false,
            &(r, c) if r <= 3 && c <= 2 => true,
            _ => false,
        }
    }
}

impl Keys<DirectionButton> for DirectionButton {
    fn coordinate(key: KeyPadButton<DirectionButton>) -> Coordinates {
        match key {
            Input(Up) => (0, 1),
            Input(Right) => (1, 2),
            Input(Down) => (1, 1),
            Input(Left) => (1, 0),
            A => (0, 2),
        }
    }

    fn contains(coord: &Coordinates) -> bool {
        match coord {
            &(0, 0) => false,
            &(r, c) if r <= 1 && c <= 2 => true,
            _ => false,
        }
    }
}

type Coordinates = (u8, u8);

trait CoordinateExtensions: Sized {
    fn apply_move(&self, mv: &DirectionButton) -> Option<Self>;
}

impl CoordinateExtensions for Coordinates {
    fn apply_move(&self, mv: &DirectionButton) -> Option<Self> {
        let (r, c) = self;
        let (dr, dc) = match mv {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        };

        let r1 = r.checked_add_signed(dr);
        let c1 = c.checked_add_signed(dc);

        r1.zip(c1)
    }
}

struct KeyPad<T> {
    cache: HashMap<(KeyPadButton<T>, KeyPadButton<T>), usize>,
    controller: Option<Rc<RefCell<KeyPad<DirectionButton>>>>,
}

impl<T> KeyPad<T>
where
    T: Keys<T> + Copy + Clone + Eq + Hash,
{
    fn direct_entry() -> KeyPad<T> {
        KeyPad::<T> {
            cache: HashMap::new(),
            controller: None::<Rc<RefCell<KeyPad<DirectionButton>>>>,
        }
    }

    fn controlled_by(controller: KeyPad<DirectionButton>) -> KeyPad<T> {
        KeyPad::<T> {
            cache: HashMap::new(),
            controller: Some(Rc::new(RefCell::new(controller))),
        }
    }

    fn key_presses(&mut self, keys: &Vec<T>) -> usize {
        once(A)
            .chain(keys.iter().map(|&key| Input(key)))
            .chain(once(A))
            .tuple_windows()
            .map(|pair| self.presses_for_pair(pair))
            .sum()
    }

    fn presses_for_pair(&mut self, (a, b): (KeyPadButton<T>, KeyPadButton<T>)) -> usize {
        if let Some(&result) = self.cache.get(&(a, b)) {
            return result;
        }

        let (ra, ca) = T::coordinate(a);
        let (rb, cb) = T::coordinate(b);

        let moves: Vec<DirectionButton> = chain(
            Self::repeat(Down, Up, ra, rb),
            Self::repeat(Right, Left, ca, cb),
        )
        .collect();

        let count = moves
            .iter()
            .permutations(moves.len())
            .filter(|moves| Self::check_moves(moves, &(ra, ca)))
            .map(|moves| self.controller_presses(moves))
            .min()
            .expect("Failed to find safe route {a} -> {b}");

        self.cache.insert((a, b), count);

        count
    }

    fn check_moves(moves: &Vec<&DirectionButton>, start: &Coordinates) -> bool {
        let mut position = start.clone();
        for &mv in moves {
            match position.apply_move(mv) {
                Some(new_pos) => {
                    if !T::contains(&new_pos) {
                        return false;
                    }
                    position = new_pos
                }
                None => return false,
            }
        }

        true
    }

    fn controller_presses(&mut self, moves: Vec<&DirectionButton>) -> usize {
        match self.controller.clone() {
            Some(keypad) => {
                let buttons = moves.into_iter().cloned().collect();
                keypad.borrow_mut().key_presses(&buttons)
            }
            None => moves.len() + 1, // and A,
        }
    }

    fn repeat(
        positive: DirectionButton,
        negative: DirectionButton,
        a: u8,
        b: u8,
    ) -> Vec<DirectionButton> {
        let char = if a < b { positive } else { negative };
        [char].repeat(a.abs_diff(b) as usize)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Code {
    buttons: Vec<NumberButton>,
    value: usize,
}

fn parse_code(code: &str) -> Code {
    let buttons = code.chars().flat_map(NumberButton::try_from).collect();
    let value = code
        .chars()
        .filter(|c| c.is_digit(10))
        .join("")
        .parse()
        .unwrap();

    Code { buttons, value }
}

fn parse_input(input: &String) -> Vec<Code> {
    input.lines().map(parse_code).collect()
}

fn keypad_chain(length: usize) -> KeyPad<NumberButton> {
    let chain = (1..length).fold(KeyPad::direct_entry(), |prev, _| {
        KeyPad::controlled_by(prev)
    });
    KeyPad::controlled_by(chain)
}

fn count_key_presses(codes: &Vec<Code>, door: &mut KeyPad<NumberButton>) -> usize {
    codes
        .iter()
        .map(|code| door.key_presses(&code.buttons) * code.value)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_21::*;

    fn example_codes() -> Vec<Code> {
        vec![
            Code {
                buttons: vec![Zero, Two, Nine],
                value: 29,
            },
            Code {
                buttons: vec![Nine, Eight, Zero],
                value: 980,
            },
            Code {
                buttons: vec![One, Seven, Nine],
                value: 179,
            },
            Code {
                buttons: vec![Four, Five, Six],
                value: 456,
            },
            Code {
                buttons: vec![Three, Seven, Nine],
                value: 379,
            },
        ]
    }

    #[test]
    fn can_parse_input() {
        let input = "029A
980A
179A
456A
379A
"
        .to_string();

        assert_eq!(parse_input(&input), example_codes());
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_expand_keys() {
        assert_eq!(
            KeyPad::direct_entry().key_presses(&example_codes()[0].buttons),
            12
        );
    }

    #[test]
    fn can_expand_part_1() {
        let mut key_pad = keypad_chain(2);

        assert_eq!(key_pad.key_presses(&example_codes()[0].buttons), 68);
        assert_eq!(key_pad.key_presses(&example_codes()[1].buttons), 60);
        assert_eq!(key_pad.key_presses(&example_codes()[2].buttons), 68);
        assert_eq!(key_pad.key_presses(&example_codes()[3].buttons), 64);
        assert_eq!(key_pad.key_presses(&example_codes()[4].buttons), 64);
    }

    #[test]
    fn can_solve_part_1() {
        assert_eq!(
            count_key_presses(&example_codes(), &mut keypad_chain(2)),
            126384
        )
    }
}
