---
day: 18
tags: [ post ]
header: 'Day 18: RAM Run'
---

Today was another grid traversal puzzle. It seems like part two might be finding a route whilst the grid changed over
time.

## Parsing the input

The input is a list of co-ordinates, the first half of which have already corrupted their target space. The grid
size / goal also varies between the sample and the actual puzzle, so I'll encode that too.

```rust
type Coordinates = (u8, u8);

#[derive(Eq, PartialEq, Debug)]
struct MemorySpace {
    corrupted: Vec<Coordinates>,
    goal: (u8, u8),
}

fn parse_coordinate(s: &str) -> Option<Coordinates> {
    let mut parts = s.split(",").flat_map(|c| c.parse().ok());
    
    parts.next().zip(parts.next())
}

fn parse_input(input: &String, size: u8) -> MemorySpace {
    let corrupted = input
        .lines()
        .flat_map(|line| parse_coordinate(line).map(|(c, r)| (r, c)))
        .collect();
    
    MemorySpace {
        corrupted,
        goal: (size, size),
    }
}

fn example_space() -> MemorySpace {
    MemorySpace {
        corrupted: vec![
            (4, 5),
            (2, 4),
            (5, 4),
            (0, 3),
            (1, 2),
            (3, 6),
            (4, 2),
            (5, 1),
            (6, 0),
            (3, 3),
            (6, 2),
            (1, 5),
            (2, 1),
            (5, 5),
            (5, 2),
            (5, 6),
            (4, 1),
            (4, 0),
            (4, 6),
            (1, 1),
            (1, 6),
            (0, 1),
            (5, 0),
            (6, 1),
            (0, 2),
        ]
            .into_iter()
            .collect(),
        goal: (6, 6),
    }
}
#[test]
fn can_parse_input() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"
        .to_string();
    
    assert_eq!(parse_input(&input, 6), example_space());
}
```

## Part 1 - Corrupted core

The first part is another implementation of A*. I set up some coordinate utilities.

```rust
trait CoordinateExtensions: Sized {
    fn manhattan_distance(&self, other: &Self) -> u32;
    fn step(&self, delta: (i8, i8), bounds: &(u8, u8)) -> Option<Self>;
}

impl CoordinateExtensions for Coordinates {
    fn manhattan_distance(&self, other: &Self) -> u32 {
        let (r0, c0) = self;
        let (r1, c1) = other;
        
        (r0.abs_diff(*r1) + c0.abs_diff(*c1)) as u32
    }
    
    fn step(&self, delta: (i8, i8), (r_max, c_max): &(u8, u8)) -> Option<Self> {
        let (r, c) = self;
        let (dr, dc) = delta;
        
        let r1 = r.checked_add_signed(dr).filter(|r| r <= r_max);
        let c1 = c.checked_add_signed(dc).filter(|c| c <= c_max);
        
        r1.zip(c1)
    }
}
```

Then I need a route in progress, with an ordering. And a way to get the next positions in the grid from the current one.

```rust
#[derive(Eq, PartialEq, Debug, Clone)]
struct Position {
    coordinates: Coordinates,
    travelled: u32,
    estimate: u32,
    visited: Vec<Coordinates>,
}

impl Position {
    fn next(&self, memory_space: &MemorySpace) -> Vec<Self> {
        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            .flat_map(|delta| self.coordinates.step(delta, &memory_space.goal))
            .map(|coordinates| {
                let mut visited = self.visited.clone();
                visited.push(coordinates);
                Position {
                    coordinates,
                    travelled: self.travelled + 1,
                    estimate: coordinates.manhattan_distance(&memory_space.goal),
                    visited,
                }
            })
            .collect()
    }
    
    pub fn new(
        coordinates: Coordinates,
        travelled: u32,
        estimate: u32,
        visited: Vec<Coordinates>,
    ) -> Self {
        Self {
            coordinates,
            travelled,
            estimate,
            visited,
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.travelled + other.estimate).cmp(&(self.travelled + self.estimate))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn can_get_next_positions() {
    let memory_space = example_space();
    
    let pos = Position::new((0, 0), 0, 12, vec![(0, 0)]);
    assert_contains_in_any_order(
        pos.next(&memory_space),
        vec![
            Position::new((0, 1), 1, 11, vec![(0, 0), (0, 1)]),
            Position::new((1, 0), 1, 11, vec![(0, 0), (1, 0)]),
        ],
    );
    
    let pos = Position::new((1, 1), 2, 10, vec![(0, 0), (0, 1), (1, 1)]);
    assert_contains_in_any_order(
        pos.next(&memory_space),
        vec![
            Position::new((1, 0), 3, 11, vec![(0, 0), (0, 1), (1, 1), (1, 0)]),
            Position::new((0, 1), 3, 11, vec![(0, 0), (0, 1), (1, 1), (0, 1)]),
            Position::new((1, 2), 3, 9, vec![(0, 0), (0, 1), (1, 1), (1, 2)]),
            Position::new((2, 1), 3, 9, vec![(0, 0), (0, 1), (1, 1), (2, 1)]),
        ],
    );
    
    let pos = Position::new((6, 6), 12, 0, vec![(6, 6)]);
    assert_contains_in_any_order(
        pos.next(&memory_space),
        vec![
            Position::new((5, 6), 13, 1, vec![(6, 6), (5, 6)]),
            Position::new((6, 5), 13, 1, vec![(6, 6), (6, 5)]),
        ],
    );
}
```

Then I need to implement the grid search. This is very similar to [Day 16](../day_16/#part-1---reindeer-racing).

```rust
impl MemorySpace {
    fn steps_to_goal(&self, bytes: usize) -> Option<Position> {
        let mut heap: BinaryHeap<Position> = BinaryHeap::new();
        let mut visited = HashMap::new();
        let blocked: HashSet<Coordinates> =
            self.corrupted.iter().take(bytes).cloned().collect();
        
        heap.push(self.starting_position());
        
        while let Some(curr) = heap.pop() {
            if curr.coordinates == self.goal {
                return Some(curr);
            }
            
            for next in curr
                .next(self)
                .into_iter()
                .filter(|pos| !blocked.contains(&pos.coordinates))
            {
                if !visited
                    .get(&next.coordinates)
                    .is_some_and(|&distance| distance <= next.travelled)
                {
                    visited.insert(next.coordinates, next.travelled);
                    heap.push(next);
                }
            }
        }
        
        None
    }
    
    fn starting_position(&self) -> Position {
        let start = (0, 0);
        Position::new(start, 0, start.manhattan_distance(&self.goal), vec![start])
    }
    
    #[test]
    fn can_find_steps_to_goal() {
        let position = example_space().steps_to_goal(12).unwrap();
        assert_eq!(position.travelled, 22);
    }
}
```

## Part 2 - The sky is falling

I didn't have to find routes whilst the grid changed, but instead find the corrupted cell that is first to block all
routes from the start to the exit.

I decide to implement a two-step process

1. Given I'm storing the path visited, staring with the route from part 1, find the first block that blocks the
   current route.
2. Re-run part 1 from using the more corrupted grid, then recurse with the newer path and position in the list of
   blocks.
3. When finding a path fails, the previous coordinate is the solution.

```rust
impl MemorySpace {
    fn route_blocked_at(&self, position: &Position, bytes: usize) -> Coordinates {
        let route: HashSet<Coordinates> = position.visited.iter().cloned().collect();
        
        let (idx, blocked_coords) = self
            .corrupted
            .iter()
            .enumerate()
            .dropping(bytes)
            .find(|&(_, coord)| route.contains(coord))
            .unwrap();
        
        if let Some(pos) = self.steps_to_goal(idx + 1) {
            self.route_blocked_at(&pos, idx)
        } else {
            blocked_coords.clone()
        }
    }
}

#[test]
fn can_find_steps_to_goal() {
    let space = example_space();
    let position = space.steps_to_goal(12).unwrap();
    assert_eq!(space.route_blocked_at(&position, 0), (1, 6));
}
```

Binary search would probably be more efficient, but this is running in 60ms, so I'm happy to leave it at that.

## Wrap up

Today felt very similar to a couple of previous days, but it was good practice.
