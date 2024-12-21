//! This is my solution for [Advent of Code - Day 21: _Keypad Conundrum_](https://adventofcode.com/2024/day/21)
//!
//! [`parse_input`] Turns the codes into their value and sequence of [`NumericButton`] presses. [`DirectionalButton`]
//! also exists to represent the keypads that move the robots rather than unlock the door. [`KeyPadButton`] is a sort
//! of meta type that generically adds the [`A`] for enter button to each keypad.
//!
//! [`KeyPad`] Holds most of the business logic, with [`keypad_chain`] being used to create the chains specific to
//! each part.
//!
//! [`Keys`] is the trait that provides how each keypad is laid out, and is implemented for each of the input button
//! types.
//!
//! [`KeyPad::key_presses`] solves the puzzle, given a chain of the relevant length of that part. It delegates the
//! movement between key presses to [`KeyPad::presses_for_pair`], which in turn generates the possible paths between
//! the pair, and recurses to the next controller in the chain using [`KeyPad::controller_presses`]. To make part 2
//! run quickly, [`KeyPad::presses_for_pair`] caches the result for each pair at that level.

use crate::day_21::DirectionalButton::*;
use crate::day_21::KeyPadButton::*;
use crate::day_21::NumericButton::*;
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
        sum_complexities(&codes, &mut keypad_chain(2))
    );

    println!(
        "To open the second door takes {} key presses",
        sum_complexities(&codes, &mut keypad_chain(25))
    );
}

/// The input buttons on pad that controls robot arm movements
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum DirectionalButton {
    Up,
    Down,
    Left,
    Right,
}

/// The input buttons on pad for entering numeric door unlock codes
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum NumericButton {
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

impl TryFrom<char> for NumericButton {
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

/// A meta-type for including the enter button on each keypad type
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum KeyPadButton<T> {
    Input(T),
    A,
}

/// Encapsulates the layout of each set of keypad buttons
trait Keys<T> {
    /// What is the coordinate of a given button
    fn coordinate(key: KeyPadButton<T>) -> Coordinates;
    /// Is this coordinate a valid button on this keypad
    fn contains(coord: &Coordinates) -> bool;
}

impl Keys<NumericButton> for NumericButton {
    fn coordinate(key: KeyPadButton<NumericButton>) -> Coordinates {
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

impl Keys<DirectionalButton> for DirectionalButton {
    fn coordinate(key: KeyPadButton<DirectionalButton>) -> Coordinates {
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

/// Helpers for moving within a keypad
trait CoordinateExtensions: Sized {
    fn apply_move(&self, mv: &DirectionalButton) -> Option<Self>;
}

impl CoordinateExtensions for Coordinates {
    /// The coordinate after pressing a specific direction key
    fn apply_move(&self, mv: &DirectionalButton) -> Option<Self> {
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

/// Encodes a KeyPad. The layout comes from the button input type (T)(
struct KeyPad<T> {
    controller: Option<Rc<RefCell<KeyPad<DirectionalButton>>>>,
    cache: HashMap<(KeyPadButton<T>, KeyPadButton<T>), usize>,
}

impl<T> KeyPad<T>
where
    T: Keys<T> + Copy + Clone + Eq + Hash,
{
    /// A new [`Keypad`] that expects a person to be pressing the keys
    fn direct_entry() -> KeyPad<T> {
        KeyPad::<T> {
            controller: None::<Rc<RefCell<KeyPad<DirectionalButton>>>>,
            cache: HashMap::new(),
        }
    }

    /// A new [`Keypad`] that expects a robot arm controlled by another pad to be pressing the keys
    fn controlled_by(controller: KeyPad<DirectionalButton>) -> KeyPad<T> {
        KeyPad::<T> {
            controller: Some(Rc::new(RefCell::new(controller))),
            cache: HashMap::new(),
        }
    }

    /// Given positive and negative unit length movement keys for an axis, and a start and end point on that axis,
    /// return the list of movements to move from the start to the end.
    fn repeat(
        positive: DirectionalButton,
        negative: DirectionalButton,
        a: u8,
        b: u8,
    ) -> Vec<DirectionalButton> {
        let char = if a < b { positive } else { negative };
        [char].repeat(a.abs_diff(b) as usize)
    }

    /// Given a list of moves, follow them and check it doesn't leave the key pad
    fn check_moves(moves: &Vec<&DirectionalButton>, start: &Coordinates) -> bool {
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

    /// Given a list of moves, pass those up the keypad chain to get the total key presses needed for that move.
    fn controller_presses(&mut self, moves: Vec<&DirectionalButton>) -> usize {
        match self.controller.clone() {
            Some(keypad) => {
                let buttons = moves.into_iter().cloned().collect();
                keypad.borrow_mut().key_presses(&buttons)
            }
            None => moves.len() + 1, // and A,
        }
    }

    /// Work out the valid paths between two keys and recurse the required movements up the keypad chain to find the
    /// route with the shortest number of presses. The result is cached for performance.
    fn presses_for_pair(&mut self, (a, b): (KeyPadButton<T>, KeyPadButton<T>)) -> usize {
        if let Some(&result) = self.cache.get(&(a, b)) {
            return result;
        }

        let (ra, ca) = T::coordinate(a);
        let (rb, cb) = T::coordinate(b);

        let moves: Vec<DirectionalButton> = chain(
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

    /// Solves the puzzle for this keypad chain, return the number of key presses needed at the top of the keypad
    /// chain to press the expected list of keys on this keypad.
    fn key_presses(&mut self, keys: &Vec<T>) -> usize {
        once(A)
            .chain(keys.iter().map(|&key| Input(key)))
            .chain(once(A))
            .tuple_windows()
            .map(|pair| self.presses_for_pair(pair))
            .sum()
    }
}

/// Encode a code as the list of numeric button presses needed. The terminating `A` is assumed. Also parse the code
/// as a number for complexity calculation
#[derive(Eq, PartialEq, Debug)]
struct Code {
    buttons: Vec<NumericButton>,
    value: usize,
}

/// Given a line of puzzle input parse it as a code.
fn parse_code(code: &str) -> Code {
    let buttons = code.chars().flat_map(NumericButton::try_from).collect();
    let value = code
        .chars()
        .filter(|c| c.is_digit(10))
        .join("")
        .parse()
        .unwrap();

    Code { buttons, value }
}

/// Turn the puzzle input into one code per line
fn parse_input(input: &String) -> Vec<Code> {
    input.lines().map(parse_code).collect()
}

/// Build a chain of keypads controlled by robot arms of the provided size. This will be 2 for part 1, and 25 for
/// part 2.
fn keypad_chain(length: usize) -> KeyPad<NumericButton> {
    let chain = (1..length).fold(KeyPad::direct_entry(), |prev, _| {
        KeyPad::controlled_by(prev)
    });
    KeyPad::controlled_by(chain)
}

/// Map the puzzles codes to their complexity and sum to get the puzzle solution
fn sum_complexities(codes: &Vec<Code>, door: &mut KeyPad<NumericButton>) -> usize {
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

    #[test]
    fn can_count_key_presses() {
        let mut key_pad = keypad_chain(2);

        assert_eq!(key_pad.key_presses(&example_codes()[0].buttons), 68);
        assert_eq!(key_pad.key_presses(&example_codes()[1].buttons), 60);
        assert_eq!(key_pad.key_presses(&example_codes()[2].buttons), 68);
        assert_eq!(key_pad.key_presses(&example_codes()[3].buttons), 64);
        assert_eq!(key_pad.key_presses(&example_codes()[4].buttons), 64);
    }

    #[test]
    fn can_sum_complexities() {
        assert_eq!(
            sum_complexities(&example_codes(), &mut keypad_chain(2)),
            126384
        )
    }
}
