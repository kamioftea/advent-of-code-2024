//! This is my solution for [Advent of Code - Day 4: _Ceres Search_](https://adventofcode.com/2024/day/4)
//!
//!

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
    )
}

#[derive(Eq, PartialEq, Debug)]
struct Wordsearch {
    cells: Vec<Vec<char>>,
}

type CellCoords = (usize, usize);

fn apply_delta(
    (x, y): &CellCoords,
    (dx, dy): &(isize, isize),
    magnitude: usize,
) -> Option<CellCoords> {
    x.checked_add_signed(dx * magnitude as isize)
        .zip(y.checked_add_signed(dy * magnitude as isize))
}

impl Wordsearch {
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

    fn get_word(&self, start: &CellCoords, length: usize, delta: &(isize, isize)) -> String {
        (0..length)
            .flat_map(|magnitude| apply_delta(start, delta, magnitude))
            .flat_map(|coord| self.char_at(&coord))
            .join("")
    }

    fn char_at(&self, &(x, y): &CellCoords) -> Option<&char> {
        self.cells.get(y).and_then(|row| row.get(x))
    }

    fn word_count(&self, search: &String) -> usize {
        let start = search.chars().next().expect("Word must not be empty");
        self.find_all(&start)
            .iter()
            .flat_map(|coord| self.words_from(coord, search.len()))
            .filter(|word| word == search)
            .count()
    }

    fn is_x_mas(&self, coord: &CellCoords) -> bool {
        let top_left =
            apply_delta(coord, &(-1, -1), 1).map(|start| self.get_word(&start, 3, &(1, 1)));
        let top_right =
            apply_delta(coord, &(1, -1), 1).map(|start| self.get_word(&start, 3, &(-1, 1)));

        self.char_at(coord) == Some(&'A')
            && (top_left == Some("MAS".to_string()) || top_left == Some("SAM".to_string()))
            && (top_right == Some("MAS".to_string()) || top_right == Some("SAM".to_string()))
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

    //noinspection SpellCheckingInspection
    #[test]
    fn can_count_xmasses() {
        assert_eq!(example_wordsearch().word_count(&"XMAS".to_string()), 4);

        let bigger_example = Wordsearch::from_str(
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
        .unwrap();

        assert_eq!(bigger_example.word_count(&"XMAS".to_string()), 18)
    }

    #[test]
    fn can_find_x_masses() {
        assert_eq!(example_wordsearch().is_x_mas(&(1, 1)), false);
        assert_eq!(example_wordsearch().is_x_mas(&(4, 2)), true);
    }

    #[test]
    fn can_count_x_masses() {}
}
