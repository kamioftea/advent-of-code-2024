---
day: 16
tags: [ post ]
header: 'Day 16: Reindeer Maze'
---

Today is another day when previous advent of code experience has been invaluable. I was able to recognise that it
needed [A*](https://en.wikipedia.org/wiki/A*_search_algorithm), and I had previous code to crib from, including my
over optimised depth-first search from [Day 7](../day_7/).

## Parsing the input

I'm still going with the hedge tiles being stored in a `HashSet<Coordinates>` but I'm not sure that's much more
performant than using a `Vec<Vec<bool>>`, but it's more familiar, and I doubt that the `Maze` implementation will be
the bottleneck today. The parsing is standard code that I've used versions of throughout this year.

```rust
type Coordinate = (usize, usize);

#[derive(Eq, PartialEq, Debug)]
struct Maze {
    hedges: HashSet<Coordinate>,
    start: Coordinate,
    end: Coordinate,
    bounds: (usize, usize),
}

fn parse_input(input: &String) -> Maze {
    let mut hedges = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut max_r = 0;
    let mut max_c = 0;
    
    for (r, row) in input.lines().enumerate() {
        for (c, char) in row.chars().enumerate() {
            match char {
                '#' => {
                    hedges.insert((r, c));
                }
                'S' => start = (r, c),
                'E' => end = (r, c),
                _ => {}
            }
            max_c = max_c.max(c);
        }
        max_r = max_r.max(r);
    }
    
    Maze {
        hedges,
        start,
        end,
        bounds: (max_r + 1, max_c + 1),
    }
}

fn example_maze() -> Maze {
    #[rustfmt::skip]
        let hedges = vec![
( 0, 0),( 0, 1),( 0, 2),( 0, 3),( 0, 4),( 0, 5),( 0, 6),( 0, 7),( 0, 8),( 0, 9),( 0,10),( 0,11),( 0,12),( 0,13),( 0,14),
( 1, 0),                                                        ( 1, 8),                                        ( 1,14),
( 2, 0),        ( 2, 2),        ( 2, 4),( 2, 5),( 2, 6),        ( 2, 8),        ( 2,10),( 2,11),( 2,12),        ( 2,14),
( 3, 0),                                        ( 3, 6),        ( 3, 8),                        ( 3,12),        ( 3,14),
( 4, 0),        ( 4, 2),( 4, 3),( 4, 4),        ( 4, 6),( 4, 7),( 4, 8),( 4, 9),( 4,10),        ( 4,12),        ( 4,14),
( 5, 0),        ( 5, 2),        ( 5, 4),                                                        ( 5,12),        ( 5,14),
( 6, 0),        ( 6, 2),        ( 6, 4),( 6, 5),( 6, 6),( 6, 7),( 6, 8),        ( 6,10),( 6,11),( 6,12),        ( 6,14),
( 7, 0),                                                                                        ( 7,12),        ( 7,14),
( 8, 0),( 8, 1),( 8, 2),        ( 8, 4),        ( 8, 6),( 8, 7),( 8, 8),( 8, 9),( 8,10),        ( 8,12),        ( 8,14),
( 9, 0),                        ( 9, 4),                                        ( 9,10),        ( 9,12),        ( 9,14),
(10, 0),        (10, 2),        (10, 4),        (10, 6),(10, 7),(10, 8),        (10,10),        (10,12),        (10,14),
(11, 0),                                        (11, 6),                        (11,10),        (11,12),        (11,14),
(12, 0),        (12, 2),(12, 3),(12, 4),        (12, 6),        (12, 8),        (12,10),        (12,12),        (12,14),
(13, 0),                        (13, 4),                                        (13,10),                        (13,14),
(14, 0),(14, 1),(14, 2),(14, 3),(14, 4),(14, 5),(14, 6),(14, 7),(14, 8),(14, 9),(14,10),(14,11),(14,12),(14,13),(14,14),
        ].into_iter().collect();
    
    Maze {
        hedges,
        start: (13, 1),
        end: (1, 13),
        bounds: (15, 15),
    }
}

#[test]
fn can_parse_input() {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
        .to_string();
    
    assert_contains_in_any_order(parse_input(&input).hedges, example_maze().hedges);
    
    assert_eq!(parse_input(&input), example_maze())
}
```

## Part 1 - Reindeer Racing

For A* to work I need each node in the graph, in this case a position and facing, it needs to have an ordering such
that the node with the lowest estimated total score (i.e. lowest current score + guess at remaining score). The
heuristic for remaining score needs to be a lower bound on the possible distance. This occurs when there are no
hedges blocking the shortest path. This means the remaining cost will be the [manhattan distance](
https://en.wikipedia.org/wiki/Taxicab_geometry) + however many turns are needed &times; 1000:

- Zero if in line with the goal, and facing it.
- One if in line with the goal and facing perpendicular to it, or if not in line but the reindeer can walk forward to
  be in line with it
- Two if in line with the goal and facing away from it, or if not in line, and need to turn before walking forward
  will bring the reindeer in line.

First I need to be able to represent a step in the route. This needs to include all the things that are used for
ordering them for the BinaryHeap, (`score` and `distance` from the goal), as well as the current tile and facing,
and I might as well define the ordering now.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Facing {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Position {
    coordinates: Coordinates,
    facing: Facing,
    score: usize,
    distance: usize,
}

impl Position {
    pub fn new(
        coordinates: Coordinates,
        facing: Facing,
        score: usize,
        distance: usize,
    ) -> Self {
        Self {
            coordinates,
            facing,
            score,
            distance,
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.score + other.distance).cmp(&(self.score + self.distance))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

Next I need to be able to advance a position by the three options (turning clockwise or anticlockwise, and moving
forwards). For this I also need to be able to calculate the estimated distance for those steps. So first some helper
functions for facings and coordinates.

```rust
impl Facing {
    fn rotate_clockwise(&self) -> Facing {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    
    fn rotate_counterclockwise(&self) -> Facing {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
    
    fn forwards(&self, (r, c): &Coordinates, maze: &Maze) -> Option<Coordinates> {
        let (dr, dc) = match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        };
        
        let (max_r, max_c) = maze.bounds;
        
        let r1 = r.checked_add_signed(dr).filter(|&r| r < max_r);
        let c1 = c.checked_add_signed(dc).filter(|&c| c < max_c);
        
        r1.zip(c1).filter(|coords| !maze.hedges.contains(coords))
    }
}

trait CoordinateExtensions {
    fn manhattan_distance(&self, other: &Self) -> usize;
    fn turn_cost(&self, other: &Self, facing: &Facing) -> usize;
}

impl CoordinateExtensions for Coordinates {
    fn manhattan_distance(&self, other: &Self) -> usize {
        let (r0, c0) = self;
        let (r1, c1) = other;
        
        (r0.abs_diff(*r1) + c0.abs_diff(*c1)) as usize
    }
    
    fn turn_cost(&self, other: &Self, facing: &Facing) -> usize {
        let (r0, c0) = self;
        let (r1, c1) = other;
        
        match (r0.cmp(&r1), c0.cmp(&c1)) {
            (Ordering::Less, Ordering::Less) => match facing {
                South | East => 1000,
                North | West => 2000,
            },
            (Ordering::Less, Ordering::Equal) => match facing {
                South => 0,
                East | West => 1000,
                North => 2000,
            },
            (Ordering::Less, Ordering::Greater) => match facing {
                South | West => 1000,
                North | East => 2000,
            },
            (Ordering::Equal, Ordering::Less) => match facing {
                East => 0,
                North | South => 1000,
                West => 2000,
            },
            (Ordering::Equal, Ordering::Equal) => 0,
            (Ordering::Equal, Ordering::Greater) => match facing {
                West => 0,
                North | South => 1000,
                East => 2000,
            },
            (Ordering::Greater, Ordering::Less) => match facing {
                North | East => 1000,
                South | West => 2000,
            },
            (Ordering::Greater, Ordering::Equal) => match facing {
                North => 0,
                East | West => 1000,
                South => 2000,
            },
            (Ordering::Greater, Ordering::Greater) => match facing {
                North | West => 1000,
                South | East => 2000,
            },
        }
    }
}

#[test]
fn can_get_manhattan_distance() {
    assert_eq!((0, 0).manhattan_distance(&(0, 0)), 0);
    assert_eq!((13, 1).manhattan_distance(&(1, 13)), 24);
    assert_eq!((13, 2).manhattan_distance(&(1, 13)), 23);
}

#[test]
fn can_get_turn_cost() {
    // (0,0) (0,1) (0,2)
    // (1,0) (1,1) (1,2)
    // (2,0) (2,1) (2,2)
    
    assert_eq!((0, 0).turn_cost(&(1, 1), &North), 2000);
    assert_eq!((0, 0).turn_cost(&(1, 1), &East), 1000);
    assert_eq!((0, 0).turn_cost(&(1, 1), &South), 1000);
    assert_eq!((0, 0).turn_cost(&(1, 1), &West), 2000);
    
    assert_eq!((0, 1).turn_cost(&(1, 1), &North), 2000);
    assert_eq!((0, 1).turn_cost(&(1, 1), &East), 1000);
    assert_eq!((0, 1).turn_cost(&(1, 1), &South), 0);
    assert_eq!((0, 1).turn_cost(&(1, 1), &West), 1000);
    
    assert_eq!((0, 2).turn_cost(&(1, 1), &North), 2000);
    assert_eq!((0, 2).turn_cost(&(1, 1), &East), 2000);
    assert_eq!((0, 2).turn_cost(&(1, 1), &South), 1000);
    assert_eq!((0, 2).turn_cost(&(1, 1), &West), 1000);
    
    assert_eq!((1, 0).turn_cost(&(1, 1), &North), 1000);
    assert_eq!((1, 0).turn_cost(&(1, 1), &East), 0);
    assert_eq!((1, 0).turn_cost(&(1, 1), &South), 1000);
    assert_eq!((1, 0).turn_cost(&(1, 1), &West), 2000);
    
    assert_eq!((1, 1).turn_cost(&(1, 1), &North), 0);
    assert_eq!((1, 1).turn_cost(&(1, 1), &East), 0);
    assert_eq!((1, 1).turn_cost(&(1, 1), &South), 0);
    assert_eq!((1, 1).turn_cost(&(1, 1), &West), 0);
    
    assert_eq!((1, 2).turn_cost(&(1, 1), &North), 1000);
    assert_eq!((1, 2).turn_cost(&(1, 1), &East), 2000);
    assert_eq!((1, 2).turn_cost(&(1, 1), &South), 1000);
    assert_eq!((1, 2).turn_cost(&(1, 1), &West), 0);
    
    assert_eq!((2, 0).turn_cost(&(1, 1), &North), 1000);
    assert_eq!((2, 0).turn_cost(&(1, 1), &East), 1000);
    assert_eq!((2, 0).turn_cost(&(1, 1), &South), 2000);
    assert_eq!((2, 0).turn_cost(&(1, 1), &West), 2000);
    
    assert_eq!((2, 1).turn_cost(&(1, 1), &North), 0);
    assert_eq!((2, 1).turn_cost(&(1, 1), &East), 1000);
    assert_eq!((2, 1).turn_cost(&(1, 1), &South), 2000);
    assert_eq!((2, 1).turn_cost(&(1, 1), &West), 1000);
    
    assert_eq!((2, 2).turn_cost(&(1, 1), &North), 1000);
    assert_eq!((2, 2).turn_cost(&(1, 1), &East), 2000);
    assert_eq!((2, 2).turn_cost(&(1, 1), &South), 2000);
    assert_eq!((2, 2).turn_cost(&(1, 1), &West), 1000);
}
```

The turn cost could definitely be done with some matrix maths or similar, but that works.

Those in place I can implement a method that returns the possible next steps from a given position.

```rust
impl Position {
    pub fn new(
        coordinates: Coordinates,
        facing: Facing,
        score: usize,
        distance: usize
    ) -> Self {
        Self {
            coordinates,
            facing,
            score,
            distance,
        }
    }
    
    fn turn_to(&self, facing: Facing, maze: &Maze) -> Self {
        Position {
            facing,
            score: self.score + 1000,
            distance: self.coordinates.manhattan_distance(&maze.end)
                + self.coordinates.turn_cost(&maze.end, &facing),
            ..self.clone()
        }
    }
    
    fn step(&self, maze: &Maze) -> Option<Self> {
        if let Some(coordinates) = self.facing.forwards(&self.coordinates, maze) {
            Some(Position {
                coordinates,
                score: self.score + 1,
                distance: coordinates.manhattan_distance(&maze.end)
                    + coordinates.turn_cost(&maze.end, &self.facing),
                facing: self.facing,
            })
        } else {
            None
        }
    }
    
    fn next(&self, maze: &Maze) -> Vec<Position> {
        vec![
            Some(self.turn_to(self.facing.rotate_clockwise(), maze)),
            Some(self.turn_to(self.facing.rotate_counterclockwise(), maze)),
            self.step(maze),
        ]
            .into_iter()
            .flatten()
            .collect()
    }
}

#[test]
fn can_get_next_moves() {
    let maze = example_maze();
    let start = example_maze().starting_position();
    let expected = vec![
        Position::new((13, 1), South, 1000, 2024),
        Position::new((13, 1), North, 1000, 1024),
        Position::new((13, 2), East, 1, 1023),
    ];
    
    assert_contains_in_any_order(start.next(&maze), expected);
    
    let start = Position::new((9, 1), North, 1004, 1020);
    let expected = vec![
        Position::new((9, 1), East, 2004, 1020),
        Position::new((9, 1), West, 2004, 2020),
    ];
    
    assert_contains_in_any_order(start.next(&maze), expected);
}
```

All that is left is to implement the search algorithm.

- Put the starting position, facing east.
- Pop items of the list, and put the next steps back on if they're new positions (the sort order will ensure the
  cheapest path to a given tile is found first).
- If a position is ever at the end, that is the cheapest way to get there.

```rust
impl Maze {
    fn starting_position(&self) -> Position {
        Position::new(
            self.start.clone(),
            East,
            0,
            self.start.manhattan_distance(&self.end),
        )
    }
    
    fn lowest_scoring_route(&self) -> usize {
        let mut heap: BinaryHeap<Position> = BinaryHeap::new();
        let mut visited = HashSet::new();
        heap.push(self.starting_position());
        
        while let Some(curr) = heap.pop() {
            if curr.coordinates == self.end {
                return curr.score;
            }
            
            for next in curr.next(self) {
                if visited.insert((next.coordinates, next.facing)) {
                    heap.push(next);
                }
            }
        }
        
        unreachable!("Failed to find route to end");
    }
    
    fn starting_position(&self) -> Position {
        Position::new(
            self.start.clone(),
            East,
            0,
            self.start.manhattan_distance(&self.end),
        )
    }
}

fn larger_example_maze() -> Maze {
    parse_input(
        &"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            .to_string(),
    )
}

#[test]
fn can_navigate_maze() {
    assert_eq!(example_maze().lowest_scoring_route(), 7036);
    assert_eq!(larger_example_maze().lowest_scoring_route(), 11048);
}
```

## Part 2 - Amazing* views

The twist is I need to be able to track the route as I go, and keep searching until I've found all possible best routes.

The first is possible by keeping a visited list in the position.

```diff
  impl Position {
      pub fn new(
          coordinates: Coordinates,
          facing: Facing,
          score: usize,
          distance: usize,
+         visited: Vec<Coordinates>,
      ) -> Self {
          Self {
              coordinates,
              facing,
              score,
              distance,
+             visited,
          }
      }
  
      fn step(&self, maze: &Maze) -> Option<Self> {
          if let Some(coordinates) = self.facing.forwards(&self.coordinates, maze) {
              Some(Position {
                  coordinates,
                  score: self.score + 1,
                  distance: coordinates.manhattan_distance(&maze.end)
                      + coordinates.turn_cost(&maze.end, &self.facing),
                  facing: self.facing,
+                 visited: [self.visited.clone(), vec![coordinates]].concat(),
              })
          } else {
              None
          }
      }
  }
```

Plus a bunch of updates to tests, starting position to match. I also need to change the visited
`HashSet<(Coordinates, Facing)>` to a `HashMap<(Coordinates, Facing), usize>` to keep track of the score and allow
other positions that match that score. Then when routes find the end, keep track of their nodes to produce a count
at the end. Once a route has completed that score is also recorded, and any positions with a higher score can then
be discarded, as the score can only increase. Once there are no more nodes in the heap, merge the routes and count
the unique nodes as the solution.

The changes are different enough, that it's easier to have a separate method for part 2.

```rust
use std::usize;

impl Maze {
    fn count_visited_by_best_routes(&self) -> usize {
        let mut heap: BinaryHeap<Position> = BinaryHeap::new();
        let mut visited: HashMap<(Coordinates, Facing), usize> = HashMap::new();
        let mut lowest_score = usize::MAX;
        let mut routes = Vec::new();
        
        heap.push(self.starting_position());
        
        while let Some(curr) = heap.pop() {
            if curr.coordinates == self.end {
                if curr.score < lowest_score {
                    lowest_score = curr.score;
                    routes = Vec::new();
                }
                
                if curr.score == lowest_score {
                    routes.push(curr.visited.clone())
                }
            }
            
            for next in curr.next(self) {
                if (next.score + next.distance) <= lowest_score
                    && !visited
                    .get(&(next.coordinates, next.facing))
                    .is_some_and(|&s| s < next.score + next.distance)
                {
                    visited.insert(
                        (next.coordinates, next.facing),
                        next.score + next.distance
                    );
                    heap.push(next);
                }
            }
        }
        
        routes.iter().flatten().unique().count()
    }
}

#[test]
fn can_find_visited_tiles() {
    assert_eq!(example_maze().count_visited_by_best_routes(), 45);
    assert_eq!(larger_example_maze().count_visited_by_best_routes(), 64);
}
```

That works and gets me the star, but it takes over 1s, or 300ms when optimised. I remember this issue from a previous
year, and because there are so many lists of co-ordinates copying them into new positions gets expensive. The quick
fix is to make the positions smaller. The scores fit in a `u32`, the max score from part 1 was ~100k. The
coordinate pairs, which are the bulk of the position, can be a `u8`. These bring it down to ~80ms optimised. There
are much more efficient ways to implement this by storing them in `Arc`s that can have pointers back to the previous
route so there's not the need to store multiple copies, but it's not worth the time for this puzzle.

## Wrap up

It felt like the main challenge today was knowing the right algorithm to use and how to implement it. It was good to
be able to do that, but it felt more like rewarding knowledge rather than puzzle solving.
