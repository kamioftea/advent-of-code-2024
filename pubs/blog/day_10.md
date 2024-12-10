---
day: 10
tags: [ post ]
header: 'Day 10: Hoof It'
---

Today was trying to find monotonically increasing hiking trails on a topographical map. The whole grid has
information I need to capture, so it is best stored as nested lists. The plan ia to find all the trailheads (height `0`,
and then recursively walk to the next number in the sequence until at the height `9` peaks. If only real hiking you
could magically start at any lowest point!

## Parsing the input

I wrap the `Vec<Vec<u8>` in a struct so that I can implement methods on it. Otherwise, the input maps to the
internal representation naturally.

```rust
type Coordinate = (usize, usize);

#[derive(Eq, PartialEq, Debug)]
struct TopographicalMap {
    cells: Vec<Vec<u8>>,
}

fn parse_input(input: &String) -> TopographicalMap {
    TopographicalMap {
        cells: input
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|c| c.to_digit(10))
                    .map(|num| num as u8)
                    .collect()
            })
            .collect(),
    }
}

fn small_example() -> TopographicalMap {
    TopographicalMap {
        cells: vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![9, 8, 7, 6],
        ],
    }
}

#[test]
fn can_parse_input() {
    let input = "0123
1234
8765
9876"
        .to_string();
    
    assert_eq!(
        parse_input(&input),
        TopographicalMap {
            cells: vec![
                vec![0, 1, 2, 3],
                vec![1, 2, 3, 4],
                vec![8, 7, 6, 5],
                vec![9, 8, 7, 6],
            ]
        }
    );
}
```

## Part 1 - Climb every mountain

The plan is to have a recursive function:

- _Base case:_ If at a height 9 point return a `HashSet` with the current coordinates
- _Recursive case:_ Otherwise, loop over all the adjacent cells, a return the union of sets from recursing all those
  that are exactly 1 unit higher.

First I'll handle getting adjacent cells, as pairs of coordinates and values. Sticking to functions that return
`Options`, then flattening the results handles cases where the cell is out of bounds.

```rust
impl TopographicalMap {
    fn adjacent(&self, (r, c): Coordinate) -> Vec<(Coordinate, u8)> {
        [
            r.checked_sub(1).zip(Some(c)),
            Some(r).zip(c.checked_add(1)),
            r.checked_add(1).zip(Some(c)),
            Some(r).zip(c.checked_sub(1)),
        ]
            .into_iter()
            .flatten()
            .flat_map(|coord| Some(coord).zip(self.get(coord)))
            .collect()
    }
    
    fn get(&self, (r, c): Coordinate) -> Option<u8> {
        self.cells.get(r).and_then(|row| row.get(c).copied())
    }
}

#[test]
fn can_find_adjacent_cells() {
    let topographical_map = small_example();
    
    assert_eq!(
        topographical_map.adjacent((1, 1)),
        vec![((0, 1), 1), ((1, 2), 3), ((2, 1), 7), ((1, 0), 1), ]
    );
    
    assert_eq!(topographical_map.adjacent((0, 0)), vec![((0, 1), 1), ((1, 0), 1), ]);
    
    assert_eq!(
        topographical_map.adjacent((3, 2)),
        vec![((2, 2), 6), ((3, 3), 6), ((3, 1), 8), ]
    )
}
```

From that I can build the recursive function. Using sets ends up a bit messy. I'll come back to that later, but
first I'll get the stars.

```rust
impl TopographicalMap {
    fn score_trailhead(&self, cell: Coordinate) -> usize {
        self.get_peaks(cell).len()
    }
    
    fn get_peaks(&self, cell: Coordinate) -> HashSet<Coordinate> {
        match self.get(cell) {
            Some(9) => vec![cell].into_iter().collect(),
            Some(n) => self
                .adjacent(cell)
                .iter()
                .filter(|(_, val)| *val == n + 1)
                .map(|(coords, _)| self.get_peaks(*coords))
                .reduce(|acc, val| acc.union(&val).cloned().collect())
                .unwrap_or(HashSet::new()),
            None => HashSet::new(),
        }
    }
}

fn larger_example() -> Grid {
    parse_input(
        &"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .to_string(),
    )
}

#[test]
fn can_score_trailhead() {
    let topographical_map = larger_example();
    
    assert_eq!(topographical_map.score_points((0, 2)), 5);
}
```

Next I need a way to find all the trailheads.

```rust
impl TopographicalMap {
    fn trailheads(&self) -> Vec<Coordinate> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                   .enumerate()
                   .filter(|(_, cell)| cell == &&0)
                   .map(move |(c, _)| (r, c))
            })
            .collect()
    }
}

#[test]
fn can_find_trailheads() {
    assert_eq!(small_example().trailheads(), vec![(0, 0)]);
    
    assert_eq!(
        larger_example().trailheads(),
        vec![
            (0, 2),
            (0, 4),
            (2, 4),
            (4, 6),
            (5, 2),
            (5, 5),
            (6, 0),
            (6, 6),
            (7, 1)
        ]
    )
}
```

And finally, combine the previous two steps to sum the score for each trailhead.

```rust
impl TopographicalMap {
    fn total_score(&self) -> usize {
        self.trailheads()
            .iter()
            .map(|&trailhead| self.score_points(trailhead))
            .sum()
    }
}

#[test]
fn can_score_topographical_map() {
    assert_eq!(larger_example().total_score(), 36);
}
```

## Part 2 - Trails all the way down

Part 2 is finding every permutation of the trails up peaks from the trailheads. I'd actually solved this as a bug
in part 1. My initial attempt had the recursive function return `1` when it found the base case of a cell at height
9, and the recursive function summing the branches that were valid. So I can grab that version from my IDE's history
and paste it in, with some renaming for uniqueness.

```rust
impl TopographicalMap {
    fn rate_points(&self, cell: Coordinate) -> usize {
        match self.get(cell) {
            Some(9) => 1,
            Some(n) => self
                .adjacent(cell)
                .iter()
                .filter(|(_, val)| *val == n + 1)
                .map(|(coords, _)| self.rate_points(*coords))
                .sum(),
            None => 0,
        }
    }
    
    fn total_rating(&self) -> usize {
        self.trailheads()
            .iter()
            .map(|&trailhead| self.rate_points(trailhead))
            .sum()
    }
}

#[test]
fn can_rate_trailhead() {
    assert_eq!(larger_example().rate_points((0, 2)), 20);
}

#[test]
fn can_rate_topographical_map() {
    assert_eq!(larger_example().total_rating(), 81);
}
```

## Refinement

Creating temporary `HashSet`s is expensive, and thinking about improvements to that I decided on a generic version
of the recursive function that does most of the work for solving both parts. If I return the coordinates of each
peak as a `Vec`, that is cheaper than a `HashSet`. Then once I have the list of all trail ends, I can count them all
for part 2, and count the unique coordinates for part 1.

```rust
impl TopographicalMap {
    fn get_peaks(&self, cell: Coordinate) -> Vec<Coordinate> {
        match self.get(cell) {
            Some(9) => vec![cell],
            Some(n) => self
                .adjacent(cell)
                .iter()
                .filter(|(_, val)| *val == n + 1)
                .map(|(coords, _)| self.get_peaks(*coords))
                .reduce(|acc, val| [acc, val].concat())
                .unwrap_or(Vec::new()),
            None => Vec::new(),
        }
    }
    
    fn score_trailhead(&self, trailhead: Coordinate) -> usize {
        self.get_peaks(trailhead).iter().unique().count()
    }
    
    fn rate_trailhead(&self, cell: Coordinate) -> usize {
        self.get_peaks(cell).iter().count()
    }
}
```

## Wrap up

I found today, especially part 2 that I'd accidentally already solved, pretty quick. And I think I've ended up with
a fairly clear and efficient solution.
