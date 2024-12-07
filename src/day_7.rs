//! This is my solution for [Advent of Code - Day 7: _Bridge Repair_](https://adventofcode.com/2024/day/7)
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-7-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 7.
pub fn run() {
    let _contents = fs::read_to_string("res/day-7-input.txt").expect("Failed to read file");
}

#[cfg(test)]
mod tests {

}