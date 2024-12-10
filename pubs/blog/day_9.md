---
day: 9
tags: [ post ]
header: 'Day 9: Disk Fragmenter'
---

Today is taking a condensed representation of files and free space on a disk, moving later files into the earlier
free space, then taking the blocks of the disks that contain parts of files into a checksum.

## Parsing the input

My plan is to have a "pointer" to the start and end of the file data, so I can store the actual map of the disk
usage in a fairly raw format.

```rust
fn parse_input(input: &String) -> Vec<u8> {
    input
        .chars()
        .flat_map(|char| char.to_digit(10))
        .map(|num| num as u8)
        .collect()
}

fn example_disk() -> Vec<u8> {
    vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2]
}

#[test]
fn can_parse_input() {
    let input = "2333133121414131402".to_string();
    
    assert_eq!(parse_input(&input), example_disk());
}
```

I do want to be able to set up the pointers as well.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct DiskPointer {
    index: usize,
    is_file: bool,
    size: u8,
}

fn get_disk_pointers(disk_map: &Vec<u8>) -> (DiskPointer, DiskPointer) {
    let head = DiskPointer { index: 0, is_file: true, size: disk_map[0] };
    let tail_index = disk_map.len() - 1;
    let tail = DiskPointer {
        index: tail_index,
        is_file: tail_index % 2 == 0,
        size: disk_map[tail_index]
    };
    
    (head, tail)
}

#[test]
fn can_build_disk_pointers() {
    let (head, tail) = get_disk_pointers(&example_disk());
    
    assert_eq!(
        head,
        DiskPointer {
            index: 0,
            is_file: true,
            size: 2,
        }
    );
    
    assert_eq!(
        tail,
        DiskPointer {
            index: 18,
            is_file: true,
            size: 2,
        }
    );
}
```

## Part 1 - Fragmenting files

The plan is to walk the pointer at the front of the file forwards, outputting file nodes for files. For free space,
use the pointer at the end of the disk to load as much of the last file as will fit, moving onto the next if it's
exhausted.

```rust
fn disk_blocks_fragmented(disk_map: &Vec<u8>) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut pos = 0usize;
    let (mut head, mut tail) = get_disk_pointers(&disk_map);
    
    // Until the pointers meet in the middle
    while head.index < tail.index {
        if head.is_file {
            // A file: Push the file blocks to output
            for _ in 0..head.size {
                blocks.push((pos, head.index / 2));
                pos += 1;
            }
        } else {
            // A blank space: fill it from files at the end
            let mut to_fill = head.size;
            while to_fill > 0 {
                // If the file is smaller than the space move it all, 
                // otherwise take a chunk of the file that fill the space
                let to_take = to_fill.min(tail.size);
                for _ in 0..to_take {
                    blocks.push((pos, tail.index / 2));
                    pos += 1;
                }
                
                to_fill -= to_take;
                tail.size -= to_take;
                
                // If the file has been consumed, decrement the tail pointer back
                // two steps to the next file from the end
                if tail.size == 0 {
                    tail.index -= 2;
                    tail.size = disk_map[tail.index];
                }
            }
        }
        
        // Increment the head pointer
        head.index = head.index + 1;
        head.is_file = !head.is_file;
        head.size = disk_map[head.index]
    }
    
    if head.index == tail.index {
        for _ in 0..tail.size {
            blocks.push((pos, tail.index / 2));
            pos += 1;
        }
    }
    
    blocks
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
```

It's messy, but I'll try to clean it up later when I have the stars locked in.

To get the solution I need to combine the pairs into their product, and sum the result.

```rust
fn calculate_checksum(disk_map: &Vec<u8>) -> usize {
    disk_blocks(disk_map).iter().map(|(pos, id)| pos * id).sum()
}

#[test]
fn can_calculate_checksum() {
    assert_eq!(
        calculate_checksum(&example_disk()),
        1928
    )
}
```

## Part 2 - Moving files

For part 2, I need to only move files into gaps where they fit whole, but I need to keep looking through the gaps
until I find a match. If there are none, then the file is left where it is, and the next file is tried.

Doing this with a pair of pointers will get even messier. After a bit of thinking, I decide it's best to change the
representation and split the files and spaces into two lists. Storing the start and size for both, and also
capturing the id for the file. `Itertools::tuples` can get the disk map in pairs of `(file, space)`, but I do need
to remember to capture the final file, too.

```rust
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
```

Then I can loop through the files backwards, moving them to a valid space if one exists, otherwise I can output
their nodes in their current position, as the task is to only do a single pass, and the order the blocks get output
doesn't matter, only the file id and size are needed. A space is valid if it is at least the size of the file, and
starts at an earlier position on the disk.

If the file fills the free space, then that free space needs to be removed from the list, otherwise it should be
replaced with the remaining space after adding the file.

```rust
fn disk_blocks_unfragmented(disk_map: &Vec<u8>) -> Vec<Block> {
    let mut blocks = Vec::new();
    let (files, mut free_space) = get_usage(&disk_map);
    
    for (file_pos, file_idx, file_size) in files.into_iter().rev() {
        if let Some((space_idx, &(space_pos, space_size))) = free_space
            .iter()
            .enumerate()
            .find(
                |(_, &(space_pos, space_size))|
                    space_size >= file_size && space_pos < file_pos
            )
        {
            // A space exists, output the new position of this file
            for i in 0..file_size as usize {
                blocks.push((space_pos + i, file_idx));
            }
            
            if space_size > file_size {
                // Space is not filled, Update the start and size of the
                // remaining space
                let space = free_space.get_mut(space_idx).unwrap();
                *space = (space_pos + file_size as usize, space_size - file_size);
            } else {
                // Otherwise the space has been filled
                free_space.remove(space_idx);
            }
        } else {
            // No space to move the file, output blocks for its current position
            for i in 0..file_size as usize {
                blocks.push((file_pos + i, file_idx));
            }
        }
    }
    
    blocks
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
```

To get the puzzle solution, I update calculate checksum to take a strategy for moving file blocks.

```rust
fn calculate_checksum(
    disk_map: &Vec<u8>,
    strategy: fn(&Vec<u8>) -> Vec<Block>
) -> usize {
    strategy(disk_map).iter().map(|(pos, id)| pos * id).sum()
}

#[test]
fn can_calculate_checksum_unfragmented() {
    assert_eq!(
        calculate_checksum(&example_disk(), disk_blocks_unfragmented),
        2858
    )
}
```

## Refactoring and reworking

When refactoring part 1 to use two lists, files and spaces, it seems like it would be easier to use a single list
with enums. So I have a go at rewriting it like that. I end up wrapping a value type within each enum variant so
that I can still have a list of only files.

```rust
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
```

I move building that into `parse_input`

```rust
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
```

Then I rewrite `disk_blocks_fragmented` to build up a list of `Files` by first peeking at the next entry from the front
of the list.

- If it's a file, that's its final position, so I can consume it and push it to the output list.
- If it's a space, take the next entry from the back,
    - If that is a file, then output as much as will fit as an output file, put any remaining space/file back in the
      disk map for later use
    - Otherwise, ignore it. The outer loop then tries again and will get the file now at the back next time

I then rewrite `disk_blocks_unfragmented`, and end up with a very similar structure, the only place that differs is
where to put the file from the back when looking to fill a space. After a few rounds of refactoring I end up pulling
the SpaceFiller out into a common type, and extract that bit of the `disk_blocks_*` functions.

```rust
/// This represents the difference between the parts.
/// &mut Vec<File>: the output file list
/// &mut VecDeque<DiskUsage>: the unprocessed entries in the disk usage map
/// Space: The leftmost space to fill
/// File: The file to be moved into a space
type SpaceFiller = fn(&mut Vec<File>, &mut VecDeque<DiskUsage>, Space, File) -> ();

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
```

These still gnarly and hard to follow, but I think that is inherent complexity of doing this in an efficient way. At
least this way, the complexity is mostly isolated to these two functions.

That done, there is now a common function for packing the files.

```rust
fn pack_files(
    disk_map: &VecDeque<DiskUsage>,
    space_filler: SpaceFiller
) -> Vec<File> {
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
```

Note this has also been refactored to return one entry per file, so that I'm not generating and passing around a
huge list of blocks. Expanding those can be done in the checksum.

```rust
fn calculate_checksum(
    disk_map: &VecDeque<DiskUsage>,
    space_filler: SpaceFiller
) -> usize {
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
```

The tests need some sweeping changes to account for the various new function signatures, but they don't change their
behaviour.

## Wrap up

The refactoring is definitely quicker ~350ms to ~80ms which is close to 4x faster. The code is not the easiest to
follow, but I think that is partially due to complexity. It does still feel like there should be a cleaner way to
represent this, but it'll do.
