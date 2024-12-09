//! This is my solution for [Advent of Code - Day 9: _Disk Fragmenter_](https://adventofcode.com/2024/day/9)
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-9-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 9.
pub fn run() {
    let contents = fs::read_to_string("res/day-9-input.txt").expect("Failed to read file");
    let disk_map = parse_input(&contents);

    println!("The checksum is {}", calculate_checksum(&disk_map));
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct DiskPointer {
    index: usize,
    is_file: bool,
    size: u8,
}

impl DiskPointer {
    fn new(index: usize, is_file: bool, size: u8) -> DiskPointer {
        DiskPointer {
            index,
            is_file,
            size,
        }
    }
}

type Block = (usize, usize);

fn parse_input(input: &String) -> Vec<u8> {
    input
        .chars()
        .flat_map(|char| char.to_digit(10))
        .map(|num| num as u8)
        .collect()
}

fn get_disk_pointers(disk_map: &Vec<u8>) -> (DiskPointer, DiskPointer) {
    let start = DiskPointer::new(0, true, disk_map[0]);
    let end_index = disk_map.len() - 1;
    let end = DiskPointer::new(end_index, end_index % 2 == 0, disk_map[end_index]);

    (start, end)
}

fn disk_blocks(disk_map: &Vec<u8>) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut pos = 0usize;
    let (mut start, mut end) = get_disk_pointers(&disk_map);

    while start.index < end.index {
        if start.is_file {
            for _ in 0..start.size {
                blocks.push((pos, start.index / 2));
                pos += 1;
            }
        } else {
            let mut to_fill = start.size;
            while to_fill > 0 {
                let to_take = to_fill.min(end.size);
                for _ in 0..to_take {
                    blocks.push((pos, end.index / 2));
                    pos += 1;
                }

                to_fill -= to_take;
                end.size -= to_take;

                if end.size == 0 {
                    end.index -= 2;
                    end.size = disk_map[end.index];
                }
            }
        }

        start.index = start.index + 1;
        start.is_file = !start.is_file;
        start.size = disk_map[start.index]
    }

    if start.index == end.index {
        for _ in 0..end.size {
            blocks.push((pos, end.index / 2));
            pos += 1;
        }
    }

    blocks
}

fn calculate_checksum(disk_map: &Vec<u8>) -> usize {
    disk_blocks(disk_map).iter().map(|(pos, id)| pos * id).sum()
}

#[cfg(test)]
mod tests {
    use crate::day_9::*;
    
    fn example_disk() -> Vec<u8> {
        vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2]
    }

    #[test]
    fn can_parse_input() {
        let input = "2333133121414131402".to_string();

        assert_eq!(parse_input(&input), example_disk());
    }

    #[test]
    fn can_build_disk_pointers() {
        let (start, end) = get_disk_pointers(&example_disk());

        assert_eq!(
            start,
            DiskPointer {
                index: 0,
                is_file: true,
                size: 2,
            }
        );

        assert_eq!(
            end,
            DiskPointer {
                index: 18,
                is_file: true,
                size: 2,
            }
        );
    }

    #[test]
    fn can_generate_blocks() {
        assert_eq!(
            disk_blocks(&example_disk()),
            vec![
                (0, 0),
                (1, 0),
                (2, 9),
                (3, 9),
                (4, 8),
                (5, 1),
                (6, 1),
                (7, 1),
                (8, 8),
                (9, 8),
                (10, 8),
                (11, 2),
                (12, 7),
                (13, 7),
                (14, 7),
                (15, 3),
                (16, 3),
                (17, 3),
                (18, 6),
                (19, 4),
                (20, 4),
                (21, 6),
                (22, 5),
                (23, 5),
                (24, 5),
                (25, 5),
                (26, 6),
                (27, 6)
            ]
        );
    }

    #[test]
    fn can_calculate_checksum() {
        assert_eq!(calculate_checksum(&example_disk()), 1928)
    }
}
