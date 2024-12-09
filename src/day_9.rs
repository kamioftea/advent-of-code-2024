//! This is my solution for [Advent of Code - Day 9: _Disk Fragmenter_](https://adventofcode.com/2024/day/9)
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-9-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 9.
pub fn run() {
    let _contents = fs::read_to_string("res/day-9-input.txt").expect("Failed to read file");
}

#[cfg(test)]
mod tests {

}