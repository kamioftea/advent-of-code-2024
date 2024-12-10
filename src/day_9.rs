//! This is my solution for [Advent of Code - Day 9: _Disk Fragmenter_](https://adventofcode.com/2024/day/9)
//!
//!

use std::collections::VecDeque;
use std::fs;
use DiskUsage::*;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-9-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 9.
pub fn run() {
    let contents = fs::read_to_string("res/day-9-input.txt").expect("Failed to read file");
    let disk_map = parse_input(&contents);

    println!(
        "The checksum is {}",
        calculate_checksum(&disk_map, disk_files_fragmented)
    );

    println!(
        "The checksum is {}",
        calculate_checksum(&disk_map, disk_files_unfragmented)
    );
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct File {
    id: usize,
    pos: usize,
    size: u8,
}

impl File {
    fn new(id: usize, pos: usize, size: u8) -> File {
        File { id, pos, size }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Space {
    pos: usize,
    size: u8,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum DiskUsage {
    FILE(File),
    SPACE(Space),
}

impl DiskUsage {
    fn new_file(id: usize, pos: usize, size: u8) -> DiskUsage {
        FILE(File { id, pos, size })
    }

    fn new_space(pos: usize, size: u8) -> DiskUsage {
        SPACE(Space { pos, size })
    }

    fn size(&self) -> u8 {
        match self {
            FILE(file) => file.size,
            SPACE(space) => space.size,
        }
    }
}

fn parse_input(input: &String) -> VecDeque<DiskUsage> {
    let mut is_file = true;
    let mut pos = 0;

    input
        .chars()
        .flat_map(|char| char.to_digit(10))
        .enumerate()
        .map(|(idx, size)| {
            let usage = if is_file {
                DiskUsage::new_file(idx / 2, pos, size as u8)
            } else {
                DiskUsage::new_space(pos, size as u8)
            };

            is_file = !is_file;
            pos += size as usize;

            usage
        })
        .filter(|usage| usage.size() > 0)
        .collect()
}

fn disk_files_fragmented(disk_map: &VecDeque<DiskUsage>) -> Vec<File> {
    let mut files = Vec::new();
    let mut usage = disk_map.clone();

    while let Some(&front) = usage.front() {
        match front {
            FILE(file) => {
                usage.pop_front();
                files.push(file);
            }
            SPACE(space) => {
                if let Some(FILE(file)) = usage.pop_back() {
                    usage.pop_front();
                    if file.size < space.size {
                        usage.push_front(DiskUsage::new_space(
                            space.pos + file.size as usize,
                            space.size - file.size,
                        ));
                    }

                    files.push(File::new(file.id, space.pos, file.size.min(space.size)));

                    if file.size > space.size {
                        usage.push_back(DiskUsage::new_file(
                            file.id,
                            file.pos,
                            file.size - space.size,
                        ))
                    }
                }
            }
        }
    }

    files
}

fn disk_files_unfragmented(disk_map: &VecDeque<DiskUsage>) -> Vec<File> {
    let mut files = Vec::new();
    let mut usage = disk_map.clone();

    while let Some(&front) = usage.front() {
        match front {
            FILE(file) => {
                usage.pop_front();
                files.push(file);
            }
            SPACE(_) => {
                if let Some(FILE(file)) = usage.pop_back() {
                    let mut stack = Vec::new();
                    loop {
                        let next = usage.pop_front();
                        match next {
                            Some(SPACE(space)) if space.size >= file.size => {
                                files.push(File::new(file.id, space.pos, file.size));
                                if space.size > file.size {
                                    usage.push_front(DiskUsage::new_space(
                                        space.pos + file.size as usize,
                                        space.size - file.size,
                                    ))
                                }
                                break;
                            }
                            Some(usage) => stack.push(usage),
                            None => {
                                files.push(file);
                                break;
                            }
                        }
                    }

                    while let Some(rewind) = stack.pop() {
                        usage.push_front(rewind);
                    }
                }
            }
        }
    }

    files
}

fn calculate_checksum(
    disk_map: &VecDeque<DiskUsage>,
    strategy: fn(&VecDeque<DiskUsage>) -> Vec<File>,
) -> usize {
    strategy(disk_map)
        .iter()
        .flat_map(
            |&File {
                 id,
                 pos: start,
                 size,
             }| (start..(start + size as usize)).map(move |pos| pos * id),
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_9::*;
    use crate::helpers::test::assert_contains_in_any_order;
    
    fn example_disk() -> VecDeque<DiskUsage> {
        vec![
            DiskUsage::new_file(0, 0, 2),
            DiskUsage::new_space(2, 3),
            DiskUsage::new_file(1, 5, 3),
            DiskUsage::new_space(8, 3),
            DiskUsage::new_file(2, 11, 1),
            DiskUsage::new_space(12, 3),
            DiskUsage::new_file(3, 15, 3),
            DiskUsage::new_space(18, 1),
            DiskUsage::new_file(4, 19, 2),
            DiskUsage::new_space(21, 1),
            DiskUsage::new_file(5, 22, 4),
            DiskUsage::new_space(26, 1),
            DiskUsage::new_file(6, 27, 4),
            DiskUsage::new_space(31, 1),
            DiskUsage::new_file(7, 32, 3),
            DiskUsage::new_space(35, 1),
            DiskUsage::new_file(8, 36, 4),
            DiskUsage::new_file(9, 40, 2),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn can_parse_input() {
        let input = "2333133121414131402".to_string();

        assert_eq!(parse_input(&input), example_disk());
    }

    #[test]
    fn can_generate_fragmented_blocks() {
        assert_eq!(
            disk_files_fragmented(&example_disk()),
            vec![
                File::new(0, 0, 2),
                File::new(9, 2, 2),
                File::new(8, 4, 1),
                File::new(1, 5, 3),
                File::new(8, 8, 3),
                File::new(2, 11, 1),
                File::new(7, 12, 3),
                File::new(3, 15, 3),
                File::new(6, 18, 1),
                File::new(4, 19, 2),
                File::new(6, 21, 1),
                File::new(5, 22, 4),
                File::new(6, 26, 1),
                File::new(6, 27, 1),
            ]
        );
    }

    #[test]
    fn can_calculate_checksum_fragmented() {
        assert_eq!(
            calculate_checksum(&example_disk(), disk_files_fragmented),
            1928
        )
    }

    #[test]
    fn can_generate_unfragmented_blocks() {
        assert_contains_in_any_order(
            disk_files_unfragmented(&example_disk()),
            vec![
                File::new(0, 0, 2),
                File::new(9, 2, 2),
                File::new(2, 4, 1),
                File::new(1, 5, 3),
                File::new(7, 8, 3),
                File::new(4, 12, 2),
                File::new(3, 15, 3),
                File::new(5, 22, 4),
                File::new(6, 27, 4),
                File::new(8, 36, 4),
            ],
        )
    }

    #[test]
    fn can_calculate_checksum_unfragmented() {
        assert_eq!(
            calculate_checksum(&example_disk(), disk_files_unfragmented),
            2858
        )
    }
}
