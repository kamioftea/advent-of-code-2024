//! This is my solution for [Advent of Code - Day 9: _Disk Fragmenter_](https://adventofcode.com/2024/day/9)
//!
//! [`parse_input`] Marks each of the entries as a [`FILE`] or [`SPACE`], along with caching their
//! position on disk, and the id of the files.
//!
//! [`calculate_checksum`] solves the puzzle, delegating to [`pack_files`] which calculates the final position of the
//! files. [`fill_space_with_fragmentation`] Is the logic for filling in disk space for part 1,
//! [`fill_space_without_fragmentation`] for part 2.

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
        calculate_checksum(&disk_map, fill_space_with_fragmentation)
    );

    println!(
        "The checksum is {}",
        calculate_checksum(&disk_map, fill_space_without_fragmentation)
    );
}

/// A file on disk
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

/// A space on disk
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Space {
    pos: usize,
    size: u8,
}

/// An enum to union both types of disk usage
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum DiskUsage {
    FILE(File),
    SPACE(Space),
}

impl DiskUsage {
    /// A new file wrapped in the union type
    fn new_file(id: usize, pos: usize, size: u8) -> DiskUsage {
        FILE(File { id, pos, size })
    }

    /// A new space wrapped in the union type
    fn new_space(pos: usize, size: u8) -> DiskUsage {
        SPACE(Space { pos, size })
    }

    /// Access size regardless of usage type
    fn size(&self) -> u8 {
        match self {
            FILE(file) => file.size,
            SPACE(space) => space.size,
        }
    }
}

/// Turn input into alternating file/space entries. Filtering out any with size 0
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

/// This represents the difference between the parts.
///
/// `&mut Vec<File>`: the output file list
/// `&mut VecDeque<DiskUsage>`: the unprocessed entries in the disk usage map
/// `Space`: The leftmost space to fill
/// `File`: The file to be moved into a space
type SpaceFiller = fn(&mut Vec<File>, &mut VecDeque<DiskUsage>, Space, File) -> ();

/// Part 1 space filler - split files to fully fill in every hole
fn fill_space_with_fragmentation(
    files: &mut Vec<File>,
    usage: &mut VecDeque<DiskUsage>,
    space: Space,
    file: File,
) {
    // consume the space at the front
    usage.pop_front();

    // A file being moved will be in its final position
    files.push(File::new(file.id, space.pos, file.size.min(space.size)));

    if file.size < space.size {
        // return remaining space to the front
        usage.push_front(DiskUsage::new_space(
            space.pos + file.size as usize,
            space.size - file.size,
        ));
    } else if file.size > space.size {
        // return remainder of file to the back
        usage.push_back(DiskUsage::new_file(
            file.id,
            file.pos,
            file.size - space.size,
        ))
    }
}

/// Part 2 space filler - only move files into spaces they fit
///
/// This ignores the passed in space as the space to fill needs to be searched for.
fn fill_space_without_fragmentation(
    files: &mut Vec<File>,
    usage: &mut VecDeque<DiskUsage>,
    _space: Space,
    file: File,
) {
    // Find a large enough space from the front iff possible
    // Keep a stack of unused Usages to restore once done
    let mut stack = Vec::new();
    loop {
        let next = usage.pop_front();
        match next {
            // Found a space
            Some(SPACE(space)) if space.size >= file.size => {
                // File now in its final position
                files.push(File::new(file.id, space.pos, file.size));
                if space.size > file.size {
                    // Return remaining space
                    usage.push_front(DiskUsage::new_space(
                        space.pos + file.size as usize,
                        space.size - file.size,
                    ))
                }
                break;
            }
            Some(usage) => stack.push(usage),
            // File won't fit, leave it in place
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

/// The common logic for reading from both ends of the disk usage map, outputting files as their final position
/// becomes known.
fn pack_files(disk_map: &VecDeque<DiskUsage>, space_filler: SpaceFiller) -> Vec<File> {
    let mut files = Vec::new();
    let mut usage = disk_map.clone();

    while let Some(&front) = usage.front() {
        match front {
            // A file at the front is in its final position
            FILE(file) => {
                usage.pop_front();
                files.push(file);
            }
            // A Space should be filled from the back
            SPACE(space) => {
                if let Some(FILE(file)) = usage.pop_back() {
                    space_filler(&mut files, &mut usage, space, file);
                }
                // Else go try the outer loop again -
                // - Some(space) has been consumed from the back
                // - None will also exit the outer loop
            }
        }
    }

    files
}

/// Reduces the list of packed files to the puzzle solution
fn calculate_checksum(disk_map: &VecDeque<DiskUsage>, space_filler: SpaceFiller) -> usize {
    pack_files(disk_map, space_filler)
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
            pack_files(&example_disk(), fill_space_with_fragmentation),
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
            calculate_checksum(&example_disk(), fill_space_with_fragmentation),
            1928
        )
    }

    #[test]
    fn can_generate_unfragmented_blocks() {
        assert_contains_in_any_order(
            pack_files(&example_disk(), fill_space_without_fragmentation),
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
            calculate_checksum(&example_disk(), fill_space_without_fragmentation),
            2858
        )
    }
}
