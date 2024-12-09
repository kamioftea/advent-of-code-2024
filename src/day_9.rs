//! This is my solution for [Advent of Code - Day 9: _Disk Fragmenter_](https://adventofcode.com/2024/day/9)
//!
//!

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-9-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 9.
pub fn run() {
    let contents = fs::read_to_string("res/day-9-input.txt").expect("Failed to read file");
    let disk_map = parse_input(&contents);

    println!(
        "The checksum is {}",
        calculate_checksum(&disk_map, disk_blocks_fragmented)
    );

    println!(
        "The checksum is {}",
        calculate_checksum(&disk_map, disk_blocks_unfragmented)
    );
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

fn disk_blocks_fragmented(disk_map: &Vec<u8>) -> Vec<Block> {
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

fn get_usage(disk_map: &Vec<u8>) -> (Vec<(usize, usize, u8)>, Vec<(usize, u8)>) {
    let mut files = Vec::new();
    let mut free_space = Vec::new();
    let mut pos = 0usize;

    disk_map
        .iter()
        .tuples()
        .enumerate()
        .for_each(|(idx, (&file, &free))| {
            if file > 0 {
                files.push((pos, idx, file));
                pos += file as usize;
            }

            if free > 0 {
                free_space.push((pos, free));
                pos += free as usize;
            }
        });

    if disk_map.len() % 2 == 1 {
        files.push((pos, disk_map.len() / 2, disk_map[disk_map.len() - 1]));
    }

    (files, free_space)
}

fn disk_blocks_unfragmented(disk_map: &Vec<u8>) -> Vec<Block> {
    let mut blocks = Vec::new();
    let (files, mut free_space) = get_usage(&disk_map);

    for (file_pos, file_idx, file_size) in files.into_iter().rev() {
        if let Some((space_idx, &(space_pos, space_size))) = free_space
            .iter()
            .enumerate()
            .find(|(_, &(space_pos, space_size))| space_size >= file_size && space_pos < file_pos)
        {
            for i in 0..file_size as usize {
                blocks.push((space_pos + i, file_idx));
            }
            if space_size > file_size {
                let space = free_space.get_mut(space_idx).unwrap();
                *space = (space_pos + file_size as usize, space_size - file_size);
            } else {
                free_space.remove(space_idx);
            }
        } else {
            for i in 0..file_size as usize {
                blocks.push((file_pos + i, file_idx));
            }
        }
    }

    blocks
}

fn calculate_checksum(disk_map: &Vec<u8>, strategy: fn(&Vec<u8>) -> Vec<Block>) -> usize {
    strategy(disk_map).iter().map(|(pos, id)| pos * id).sum()
}

#[cfg(test)]
mod tests {
    use crate::day_9::*;
    use crate::helpers::test::assert_contains_in_any_order;
    
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
    fn can_generate_fragmented_blocks() {
        assert_eq!(
            disk_blocks_fragmented(&example_disk()),
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
    fn can_calculate_checksum_fragmented() {
        assert_eq!(
            calculate_checksum(&example_disk(), disk_blocks_fragmented),
            1928
        )
    }

    #[test]
    fn can_get_disk_usage() {
        assert_eq!(
            get_usage(&example_disk()),
            (
                vec![
                    (0, 0, 2),
                    (5, 1, 3),
                    (11, 2, 1),
                    (15, 3, 3),
                    (19, 4, 2),
                    (22, 5, 4),
                    (27, 6, 4),
                    (32, 7, 3),
                    (36, 8, 4),
                    (40, 9, 2)
                ],
                vec![
                    (2, 3),
                    (8, 3),
                    (12, 3),
                    (18, 1),
                    (21, 1),
                    (26, 1),
                    (31, 1),
                    (35, 1)
                ]
            )
        )
    }

    #[test]
    fn can_generate_unfragmented_blocks() {
        assert_contains_in_any_order(
            disk_blocks_unfragmented(&example_disk()),
            vec![
                (0, 0),
                (1, 0),
                (2, 9),
                (3, 9),
                (4, 2),
                (5, 1),
                (6, 1),
                (7, 1),
                (8, 7),
                (9, 7),
                (10, 7),
                (12, 4),
                (13, 4),
                (15, 3),
                (16, 3),
                (17, 3),
                (22, 5),
                (23, 5),
                (24, 5),
                (25, 5),
                (27, 6),
                (28, 6),
                (29, 6),
                (30, 6),
                (36, 8),
                (37, 8),
                (38, 8),
                (39, 8),
            ],
        )
    }

    #[test]
    fn can_calculate_checksum_unfragmented() {
        assert_eq!(
            calculate_checksum(&example_disk(), disk_blocks_unfragmented),
            2858
        )
    }
}
