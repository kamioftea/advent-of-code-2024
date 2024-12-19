//! This is my solution for [Advent of Code - Day 19: _Linen Layout_](https://adventofcode.com/2024/day/19)
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-19-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 19.
pub fn run() {
    let _contents = fs::read_to_string("res/day-19-input.txt").expect("Failed to read file");
}

#[cfg(test)]
mod tests {}
