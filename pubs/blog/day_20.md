---
day: 20
tags: [ post ]
header: 'Day 20: Race Condition'
---

Another 2D grid puzzle today. This time with a single racetrack through the grid, but with the opportunity to
briefly cheat and cut through the barrier to shorten the route.

## Parsing the input

Parsing 2D grids into sets of spaces is routine by now. The difference today is it is the empty spaces that are more
sparse and more of interest to the puzzle. I decide to store the spaces in the track, including start and end in a
`HashSet<Coordinates>`, and also record the start and end.

```rust
type Coordinates = (usize, usize);

#[derive(Eq, PartialEq, Debug)]
struct RaceTrack {
    course: HashSet<Coordinates>,
    start: Coordinates,
    end: Coordinates,
}

fn parse_input(input: &String) -> RaceTrack {
    let mut course = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    
    for (r, row) in input.lines().enumerate() {
        for (c, char) in row.chars().enumerate() {
            match char {
                '.' => {
                    course.insert((r, c));
                }
                'S' => {
                    course.insert((r, c));
                    start = (r, c);
                }
                'E' => {
                    course.insert((r, c));
                    end = (r, c);
                }
                _ => {}
            }
        }
    }
    
    RaceTrack { course, start, end }
}

fn example_track() -> RaceTrack {
    #[rustfmt::skip]
        let course = vec![
        ( 1, 1),( 1, 2),( 1, 3),        ( 1, 5),( 1, 6),( 1, 7),        ( 1, 9),( 1,10),( 1,11),( 1,12),( 1,13),
        ( 2, 1),        ( 2, 3),        ( 2, 5),        ( 2, 7),        ( 2, 9),                        ( 2,13),
        ( 3, 1),        ( 3, 3),( 3, 4),( 3, 5),        ( 3, 7),        ( 3, 9),        ( 3,11),( 3,12),( 3,13),
                                                        ( 4, 7),        ( 4, 9),        ( 4,11),
                                                        ( 5, 7),        ( 5, 9),        ( 5,11),( 5,12),( 5,13),
                                                        ( 6, 7),        ( 6, 9),                        ( 6,13),
                        ( 7, 3),( 7, 4),( 7, 5),        ( 7, 7),( 7, 8),( 7, 9),        ( 7,11),( 7,12),( 7,13),
                        ( 8, 3),                                                        ( 8,11),
        ( 9, 1),( 9, 2),( 9, 3),                        ( 9, 7),( 9, 8),( 9, 9),        ( 9,11),( 9,12),( 9,13),
        (10, 1),                                        (10, 7),        (10, 9),                        (10,13),
        (11, 1),        (11, 3),(11, 4),(11, 5),        (11, 7),        (11, 9),        (11,11),(11,12),(11,13),
        (12, 1),        (12, 3),        (12, 5),        (12, 7),        (12, 9),        (12,11),
        (13, 1),(13, 2),(13, 3),        (13, 5),(13, 6),(13, 7),        (13, 9),(13,10),(13,11),
        ].into_iter().collect();
    
    RaceTrack {
        course,
        start: (3, 1),
        end: (7, 5),
    }
}

#[test]
fn can_parse_input() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"
        .to_string();
    
    assert_eq!(parse_input(&input), example_track());
}
```

## Part 1 - Glitches in the matrix

First a standard helper to move around the track.

```rust
trait CoordinateExtensions: Sized {
    fn apply(&self, delta: &(isize, isize)) -> Option<Self>;
}

impl CoordinateExtensions for Coordinates {
    fn apply(&self, delta: &(isize, isize)) -> Option<Self> {
        let (r, c) = self;
        let (dr, dc) = delta;
        
        let r1 = r.checked_add_signed(*dr);
        let c1 = c.checked_add_signed(*dc);
        
        r1.zip(c1)
    }
}
```

Next I implement the following:

- Walk round the track, recording the distance to that location by finding the adjacent track space that hasn't
  already been visited. There is always exactly one until the end is reached.
- For the two adjacent pieces of barrier, if the space beyond that is one that's already been visited, that's a
  possible cheat move. Store the start and end pair, and the amount of time saved based on the index when that space
  was first visited.
- Break once the end is reached.

It's not very clean code, but I don't have a good grasp of where to subdivide it, so I defer that until part 2.

```rust
impl RaceTrack {
    fn cheats(&self) -> HashMap<(Coordinates, Coordinates), usize> {
        let mut visited = HashMap::new();
        let mut cheats = HashMap::new();
        let mut position = self.start;
        
        for index in 0.. {
            visited.insert(position, index);
            let mut next = position;
            for delta in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let maybe_adjacent = position.apply(&delta);
                let maybe_next = maybe_adjacent.filter(|coords|
                    self.course.contains(coords)
                );
                
                if maybe_adjacent.is_some() && maybe_next.is_none() {
                    let adjacent = maybe_adjacent.unwrap();
                    let maybe_cheat = adjacent
                        .apply(&delta)
                        .and_then(|coords| Some(coords).zip(visited.get(&coords)));
                    if let Some((start_pos, &start_index)) = maybe_cheat {
                        cheats.insert((start_pos, position), index - start_index - 2);
                    }
                }
                
                let without_visited = maybe_next.filter(|coords|
                    !visited.contains_key(coords)
                );
                if without_visited.is_some() {
                    next = without_visited.unwrap();
                }
            }
            
            if position == self.end {
                break;
            }
            
            if next == position {
                unreachable!("{position:?} failed to find next position")
            }
            
            position = next;
        }
        
        cheats
    }
}

#[test]
fn can_list_cheats() {
    let cheats = example_track().cheats();
    
    assert_eq!(cheats.get(&((1, 7), (1, 9))), Some(&12));
    assert_eq!(cheats.get(&((7, 9), (7, 11))), Some(&20));
    assert_eq!(cheats.get(&((7, 8), (9, 8))), Some(&38));
    assert_eq!(cheats.get(&((7, 7), (7, 5))), Some(&64));
    
    assert_eq!(cheats.len(), 44);
}
```

For the puzzle solution I need to filter out any that don't save at least 100 picoseconds, and then count the rest.

```rust
impl RaceTrack {
    fn count_cheats_from(&self, threshold: usize) -> usize {
        self.cheats()
            .iter()
            .map(|(_, saving)| saving)
            .filter(|&&saving| saving >= threshold)
            .count()
    }
}

#[test]
fn can_count_significant_cheats() {
    let track = example_track();
    
    assert_eq!(track.count_cheats_from(4), 30);
    assert_eq!(track.count_cheats_from(15), 5);
}
```

## Part 2 - Blatant cheating

The update for part 2 is that now the phase out that allows cheating can now last for up to 20 picoseconds. I ponder
doing more graph traversal, but notice that the key is the distance between the possible pairs of locations on the
track. Since the route doesn't matter, only the start and end, the [manhattan distance](
https://en.wikipedia.org/wiki/Taxicab_geometry) will be the shortest route to cheat between two sections of track.
If I limit the distance to the max cheat length, I can compare all pairs of positions on the track, and filter out
those where the cheat is too long or doesn't save enough time.

First I'll refactor the messy first part to separate out building the track from calculating the cheats:

```rust
impl RaceTrack {
    fn get_track_positions(&self) -> Vec<(usize, Coordinates)> {
        let mut visited = Vec::new();
        let mut position = self.start;
        let mut prev = self.start;
        
        for index in 0.. {
            visited.push((index, position));
            if position == self.end {
                break;
            }
            
            for delta in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                if let Some(next) = position
                    .apply(&delta)
                    .filter(|coords| self.course.contains(coords))
                    .filter(|coords| coords != &prev)
                {
                    prev = position;
                    position = next;
                    break;
                }
            }
        }
        
        visited
    }
}

#[test]
fn can_find_track() {
    let track = example_track();
    let positions = track.get_track_positions();
    
    assert_eq!(positions.len(), track.course.len());
    assert_eq!(positions[0], (0, (3, 1)));
    assert_eq!(positions[84], (84, (7, 5)));
}
```

Next I need to update finding the cheats to get the track position combinations. At this point I add using both the
existing `saving_threshold` and the `max_cheat` into the cheat generation as it's easier to filter them out as the
result is being built. Count cheats from also needs updating to take the new cheat_length argument, and the part two
examples can be added to the existing test.

```rust
impl RaceTrack {
    fn cheats(
        &self,
        saving_threshold: usize,
        cheat_length: usize,
    ) -> HashMap<(Coordinates, Coordinates), usize> {
        let track = self.get_track_positions();
        
        track
            .iter()
            .tuple_combinations()
            .flat_map(|(&(start_idx, start_coord), &(end_idx, end_coord))| {
                let manhattan_distance = start_coord.manhattan_distance(&end_coord);
                if manhattan_distance > cheat_length {
                    None
                } else {
                    Some((start_coord, end_coord))
                        .zip((end_idx - start_idx).checked_sub(manhattan_distance))
                        .filter(|&(_, distance)| distance >= saving_threshold)
                }
            })
            .collect()
    }
    
    
    fn count_cheats_from(&self, threshold: usize, cheat_length: usize) -> usize {
        self.cheats(threshold, cheat_length)
            .iter()
            .map(|(_, saving)| saving)
            .count()
    }
}

fn can_count_significant_cheats() {
    let track = example_track();
    
    assert_eq!(track.count_cheats_from(4, 2), 30);
    assert_eq!(track.count_cheats_from(15, 2), 5);
    
    assert_eq!(track.count_cheats_from(50, 20), 285);
    assert_eq!(track.count_cheats_from(72, 20), 29);
}
```

That's working but it's taking 200ms. Some of that will be that the combinations of track pairs is pretty
inefficient, but there are some other factors.

- Returning the cheats as a `HashMap<(Coordinates, Coordinates), usize>` with the start and end coordinates as the
  key. This is only used in a test, which is testing an implementation detail. I don't need to keep that test, so I can
  switch out the `HashMap` for a `Vec` of the cheats. Down to ~80ms
- That done `count_cheats_from` is only counting the returned `Vec` of cheat savings. This can be inlined into
  `cheats` so that I don't need to store the full list in memory. This only saves another ~10ms, but also simplifies
  the code.

## Wrap up

I'm still enjoying the grid puzzles. When I implemented part 1 it felt a little rote, but the twist in part two
meant I ended up with a solution unlike the other gird puzzles so far.
