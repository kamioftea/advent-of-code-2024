//! This is my solution for [Advent of Code - Day 4: _Ceres Search_](https://adventofcode.com/2024/day/4)
//!
//! [`Wordsearch`] and its methods solve te solution today. [`Wordsearch::from_str`] handles parsing the puzzle input.
//!
//! [`Wordsearch::word_count`] solves part 1, using [`apply_delta`], [`Wordsearch::char_at`], [`Wordsearch::get_word`],
//! and [`Wordsearch::words_from`], and [`Wordsearch::find_all`].
//!
//! [`Wordsearch::count_x_masses`] solves part 2, using [`Wordsearch::is_x_mas`], which in turn reuses some of the
//! part 1 helpers

use itertools::Itertools;
use std::fs;
use std::str::FromStr;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-4-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 4.
pub fn run() {
    let contents = fs::read_to_string("res/day-4-input.txt").expect("Failed to read file");

    let wordsearch = Wordsearch::from_str(&contents).unwrap();

    println!(
        "There are {} XMASes",
        wordsearch.word_count(&"XMAS".to_string())
    );

    println!("There are {} X-MASes", wordsearch.count_x_masses());
}

/// A wordsearch grid
#[derive(Eq, PartialEq, Debug)]
struct Wordsearch {
    cells: Vec<Vec<char>>,
}

/// A coordinate to a cell in [`Wordsearch`]
type CellCoords = (usize, usize);

/// Get `Some(coordinate)` that is `magnitude` distance along a line with a given `delta`. Returning None if applying
/// `delta * magnitude` is out of bounds.
fn apply_delta(
    (x, y): &CellCoords,
    (dx, dy): &(isize, isize),
    magnitude: usize,
) -> Option<CellCoords> {
    x.checked_add_signed(dx * magnitude as isize)
        .zip(y.checked_add_signed(dy * magnitude as isize))
}

impl Wordsearch {
    /// Return a list of all the cell coordinates that contain the provided `letter`
    fn find_all(&self, letter: &char) -> Vec<CellCoords> {
        let mut coords = Vec::new();
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell == letter {
                    coords.push((x, y))
                }
            }
        }

        coords
    }

    /// Find all the words in the 8 possible axes from a given start, of the given `length`. These will be cropped if
    /// any overflow the edges of the [`Wordsearch`].
    fn words_from(&self, start: &CellCoords, length: usize) -> Vec<String> {
        let deltas = vec![
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ];
        deltas
            .iter()
            .map(|delta| self.get_word(start, length, delta))
            .collect()
    }

    /// Return the letters along a given delta from a starting coordinate pair, of the given `length`. It will be
    /// cropped if it overflows the wordsearch.
    fn get_word(&self, start: &CellCoords, length: usize, delta: &(isize, isize)) -> String {
        (0..length)
            .flat_map(|magnitude| apply_delta(start, delta, magnitude))
            .flat_map(|coord| self.char_at(&coord))
            .join("")
    }

    /// If the coordinate pair given is within the grid return `Some(letter)` otherwise None.
    fn char_at(&self, &(x, y): &CellCoords) -> Option<&char> {
        self.cells.get(y).and_then(|row| row.get(x))
    }

    /// Solves part 1: Find all instances of the `search` word in the wordsearch
    fn word_count(&self, search: &String) -> usize {
        let start = search.chars().next().expect("Word must not be empty");
        self.find_all(&start)
            .iter()
            .flat_map(|coord| self.words_from(coord, search.len()))
            .filter(|word| word == search)
            .count()
    }

    /// For a given center point, return true if it is the centre of an `X-MAS`.
    ///
    /// Valid matches are:
    ///
    /// ```text
    /// M.M  M.S  S.M  S.S
    /// .A.  .A.  .A.  .A.
    /// S.S  M.S  S.M  M.M
    /// ```
    fn is_x_mas(&self, coord: &CellCoords) -> bool {
        let top_left =
            apply_delta(coord, &(-1, -1), 1).map(|start| self.get_word(&start, 3, &(1, 1)));
        let top_right =
            apply_delta(coord, &(1, -1), 1).map(|start| self.get_word(&start, 3, &(-1, 1)));

        self.char_at(coord) == Some(&'A')
            && (top_left == Some("MAS".to_string()) || top_left == Some("SAM".to_string()))
            && (top_right == Some("MAS".to_string()) || top_right == Some("SAM".to_string()))
    }

    /// Solves part 2: Find all the A's in the grid and count those that are the center of an `X-MAS`
    fn count_x_masses(&self) -> usize {
        self.find_all(&'A')
            .iter()
            .filter(|coord| self.is_x_mas(coord))
            .count()
    }
}

impl FromStr for Wordsearch {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s.lines().map(|l| l.chars().collect()).collect();

        Ok(Wordsearch { cells })
    }
}

#[cfg(test)]
mod tests {
    use crate::day_4::*;
    
    #[test]
    fn can_parse_input() {
        let input = "..X...
.SAMXM
.A..A.
XMAS.S
.X....";

        assert_eq!(Wordsearch::from_str(input), Ok(example_wordsearch()));
    }

    fn example_wordsearch() -> Wordsearch {
        let cells = vec![
            vec!['.', '.', 'X', '.', '.', '.'],
            vec!['.', 'S', 'A', 'M', 'X', 'M'],
            vec!['.', 'A', '.', '.', 'A', '.'],
            vec!['X', 'M', 'A', 'S', '.', 'S'],
            vec!['.', 'X', '.', '.', '.', '.'],
        ];

        Wordsearch { cells }
    }

    #[test]
    fn can_find_all_xs() {
        assert_eq!(
            example_wordsearch().find_all(&'X'),
            vec![(2, 0), (4, 1), (0, 3), (1, 4)]
        )
    }

    #[test]
    fn can_find_words() {
        assert_eq!(
            example_wordsearch().words_from(&(2, 0), 4),
            vec![
                "X..".to_string(),
                "X".to_string(),
                "X".to_string(),
                "X".to_string(),
                "X...".to_string(),
                "XMAS".to_string(),
                "XA.A".to_string(),
                "XS.".to_string()
            ]
        )
    }

    #[test]
    fn can_count_xmasses() {
        assert_eq!(example_wordsearch().word_count(&"XMAS".to_string()), 4);
        assert_eq!(bigger_example().word_count(&"XMAS".to_string()), 18)
    }

    fn bigger_example() -> Wordsearch {
        Wordsearch::from_str(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        )
        .unwrap()
    }

    #[test]
    fn can_check_for_an_x_mas() {
        assert_eq!(example_wordsearch().is_x_mas(&(1, 1)), false);
        assert_eq!(example_wordsearch().is_x_mas(&(4, 2)), true);
    }

    #[test]
    fn can_count_x_masses() {
        assert_eq!(example_wordsearch().count_x_masses(), 1);
        assert_eq!(bigger_example().count_x_masses(), 9);
    }
}
