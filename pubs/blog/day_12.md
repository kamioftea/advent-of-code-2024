---
day: 12
tags: [ post ]
header: 'Day 12: Garden Groups'
---

Today is finding regions in a grid of plots grouped by the crop they grow, then doing some maths on the area and
perimeter of the identified regions. It feels like a good day to be better at matrix maths, but I'll muddle through.

## Parsing the input

The grid itself can be represented as a list of lists of characters. I can lift the implementation from
[Day 10](../day_10/#parsing-the-input) without the parsing to an integer.

```rust
#[derive(Eq, PartialEq, Debug)]
struct Garden {
    plots: Vec<Vec<char>>,
}

fn parse_input(input: &String) -> Garden {
    Garden {
        plots: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

fn example_garden() -> Garden {
    Garden {
        plots: vec![
            vec!['A', 'A', 'A', 'A'],
            vec!['B', 'B', 'C', 'D'],
            vec!['B', 'B', 'C', 'C'],
            vec!['E', 'E', 'E', 'C'],
        ],
    }
}

//noinspection SpellCheckingInspection
#[test]
fn can_parse_input() {
    let input = "AAAA
BBCD
BBCC
EEEC"
        .to_string();
    
    assert_eq!(parse_input(&input), example_garden())
}
```

## Part 1 - Bucket fill

The first task of part one is splitting the grid into regions. This can be done with a sort-of repeated bucket fill,
that also captures the length of the perimeter as it goes.

Like day 10, I'll need a `get` and `adjacent`. Later in the puzzle I end up pulling out a `Delta` type rather than
using `(isize, isize)`, and I think it makes this clearer, so I'll use that version in here.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Delta(isize, isize);

impl Delta {
    const UP: Delta = Delta(-1, 0);
    const RIGHT: Delta = Delta(0, 1);
    const DOWN: Delta = Delta(1, 0);
    const LEFT: Delta = Delta(0, -1);
}

impl Garden {
    fn get(&self, (r, c): Plot) -> Option<char> {
        self.plots.get(r).and_then(|row| row.get(c).copied())
    }
    
    fn adjacent(&self, origin: Plot) -> Vec<(Plot, char)> {
        [
            Delta::UP.apply_to(origin),
            Delta::RIGHT.apply_to(origin),
            Delta::DOWN.apply_to(origin),
            Delta::LEFT.apply_to(origin),
        ]
            .into_iter()
            .flatten()
            .flat_map(|coord| Some(coord).zip(self.get(coord)))
            .collect()
    }
}
```

From these, the plan is:

- Keep a `HashSet` of visited plots
- Iterate through each cell in the grid, if it hasn't already been included in a region:
    - Build the region by bucket-filling from that point
    - This is done by recursively walking to each neighbour
        - If it's not the same crop, we crossed the edge of the region and can add 1 to the perimeter
        - If it is the same crop, and it's not already in the region, add it to the region and recurse
    - There's a special case here for edges that return < 4 neighbours. Each missing neighbour also indicates an
      edge to the region and the perimeter needs to be incremented like when visiting a plot with the wrong crop.
- Add that whole region to the visited set to prevent it being duplicated.

Firstly the inner bucket fill. The recursive function takes extra arguments that are implementation details, so I'm
going to try a pattern I picked up writing scala where:

- Write the function with the API you want callers to use
- Nest the recursive function inside where it doesn't pollute the API
- Call the recursive function with the initial arguments for the recursion

```rust
#[derive(Eq, PartialEq, Debug)]
struct Region {
    crop: char,
    plots: HashSet<Plot>,
    perimeter: usize,
}

impl Region {
    fn new(crop: char) -> Region {
        Region {
            crop,
            plots: HashSet::new(),
            perimeter: 0,
        }
    }
}

impl Garden {
    fn walk_region(&self, start: Plot) -> Region {
        fn walk_region_iter(garden: &Garden, plot: Plot, region: &mut Region) {
            let crop = garden.get(plot).unwrap();
            if crop != region.crop {
                region.perimeter += 1;
                return;
            }
            
            if !region.plots.insert(plot) {
                // already visited
                return;
            }
            
            let adjacent = garden.adjacent(plot);
            // Any cells missing are outside the grid and so that side has an edge
            region.perimeter += 4 - adjacent.len();
            
            adjacent
                .iter()
                .for_each(|&(next_plot, _)| walk_region_iter(garden, next_plot, region))
        }
        
        let mut region = Region::new(self.get(start).unwrap());
        walk_region_iter(self, start, &mut region);
        region
    }
}

#[test]
fn can_find_region() {
    let garden = example_garden();
    
    let region_a = Region {
        crop: 'A',
        plots: vec![(0, 0), (0, 1), (0, 2), (0, 3)].into_iter().collect(),
        perimeter: 10,
    };
    let region_b = Region {
        crop: 'B',
        plots: vec![(1, 0), (1, 1), (2, 0), (2, 1)].into_iter().collect(),
        perimeter: 8,
    };
    let region_c = Region {
        crop: 'C',
        plots: vec![(1, 2), (2, 2), (2, 3), (3, 3)].into_iter().collect(),
        perimeter: 10,
    };
    let region_d = Region {
        crop: 'D',
        plots: vec![(1, 3)].into_iter().collect(),
        perimeter: 4,
    };
    let region_e = Region {
        crop: 'E',
        plots: vec![(3, 0), (3, 1), (3, 2)].into_iter().collect(),
        perimeter: 8,
    };
    
    assert_eq!(garden.walk_region((0, 0)), region_a);
    assert_eq!(garden.walk_region((1, 0)), region_b);
    assert_eq!(garden.walk_region((1, 2)), region_c);
    assert_eq!(garden.walk_region((1, 3)), region_d);
    assert_eq!(garden.walk_region((3, 0)), region_e);
}
```

The outer loop is loop that keeps calling `walk_region` whenever it finds a plot not in an existing region. I sneak
the test for this into the bottom of `can_find_region()`.

```rust
impl Garden {
    fn iter_plots<'a>(&'a self) -> impl Iterator<Item=Plot> + 'a {
        self.plots
            .iter()
            .enumerate()
            .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, _)| (r, c)))
    }
    
    fn find_regions(&self) -> Vec<Region> {
        let mut visited: HashSet<Plot> = HashSet::new();
        let mut regions = Vec::new();
        
        for (r, c) in self.iter_plots() {
            if !visited.contains(&(r, c)) {
                let region = self.walk_region((r, c));
                visited.extend(&region.plots);
                regions.push(region);
            }
        }
        
        regions
    }
}

#[test]
fn can_find_region() {
    // ...
    assert_contains_in_any_order(
        garden.find_regions(),
        vec![region_a, region_b, region_c, region_d, region_e],
    );
}
```

That is the majority of the work done for part 1. All that is left is mapping the regions to their scores and
summing the total.

```rust
impl Garden {
    fn total_fencing_cost(&self) -> usize {
        self.find_regions()
            .iter()
            .map(|region| region.plots.len() * region.perimeter)
            .sum()
    }
}

//noinspection SpellCheckingInspection
fn enclave_example() -> Garden {
    parse_input(
        &"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            .to_string(),
    )
}

//noinspection SpellCheckingInspection
fn larger_example() -> Garden {
    parse_input(
        &"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"
            .to_string(),
    );
}

#[test]
fn can_calculate_costs() {
    assert_eq!(example_garden().total_fencing_cost(), 140);
    assert_eq!(enclave_example().total_fencing_cost(), 772);
    assert_eq!(larger_example().total_fencing_cost(), 1930);
}
```

## Part 2 - The fences on the plots go round and round

Part 2 is a more complex version of the same. Each edge now only counts once, so I need a way to work out which bits
of the perimeter is part of the same edge. I can walk around each edge, incrementing the edge each time it turns a
corner.

Once I've found an edge I can follow it by checking whether nearby cells are in the grid.

- If the edge outwards, making the edge of the shape convex (from the outside), the cell one space forward and left
  is also part of the shape. I can jump there and turn left.
- If it's a straight line, then the one directly forward is set, I can go forward and check again.
- Otherwise, there is an edge directly in front, turn right without stepping and try again.

The first and second case are a corner, so also increment the edge count by one.

```text
    +---+---+
    | A | B |
+---+ - + - +
| E | D | C |
+---+---+---+
```

- If I start at A then the cell going clockwise I start facing rightwards. The cell above B is not in the shape, so
  the edge doesn't turn right. Straight on is B, so I move there keeping the same facing.
- For B, the concave space is still not in the shape, neither is straight on, so stay in B and turn right,
- The same happens, moving into C then turning, incrementing the count again.
- C to D to E are all straight on.
- In E, the two cells are outside, so turn, and then ahead is still blocked, so turn again, incrementing the count
  on each turn.
- From E, heading rightwards is the first concave corner. A is in the shape, so move to A and turn left.
- Finally, the way ahead is blocked, so turn, and I'm back to a position and facing already seen so that perimeter is
  done.

```text
Count:    0                 0                1                 1                 2
          X                     X                                         
    +---+---+         +---+---+        +---+---+         +---+---+         +---+---+
    | > | O |         |   | > | X      |   | V |         |   |   |         |   |   | 
+---+ - + - +     +---+ - + - +    +---+ - + - +     +---+ - + - +     +---+ - + - +
|   |   |   |     |   |   |   |    |   |   | O | X   |   |   | V |     |   | O | < | 
+---+---+---+     +---+---+---+    +---+---+---+     +---+---+---+     +---+---+---+
                                                               X   X         X
                                                              
                                                              
Count:    2                 2                 3                 4                 5
                                                                          X   X
    +---+---+         +---+---+         +---+---+         +---+---+         +---+---+
    |   |   |         |   |   |   X   X |   |   |         | O |   |         | ^ |   | 
+---+ - + - +     +---+ - + - +     +---+ - + - +     +---+ - + - +     +---+ - + - +
| O | < |   |   X | < |   |   |     | ^ |   |   |     | > |   |   |     |   |   |   | 
+---+---+---+     +---+---+---+     +---+---+---+     +---+---+---+     +---+---+---+
  X             X
   
Count:    6     
                
    +---+---+   
    | > |   |   
+---+ - + - +   
|   |   |   |   
+---+---+---+                                             
```

Technically a C shape that touches itself on a diagonal, the concave test "jumps the gap". This is covered because
when that happens the inner fence isn't walked, and it will be crossed looking for other edges and still get counted
later in the process.

I need to keep track of which type of side I'm following. I initially implemented this with both a side and a
direction, but the direction was only used to derive the deltas for cells to check, and that could be done from the
side. Keeping both in sync was tedious and error-prone, so it removed the middleman.

Enumerating all of these is a lot of boilerplate, but it makes the gnarly edge-walking logic a bit easier to read.

```rust
impl Delta {
    fn add(&self, other: &Self) -> Self {
        Delta(self.0 + other.0, self.1 + other.1)
    }
    
    fn apply_to(&self, (r, c): Plot) -> Option<Plot> {
        r.checked_add_signed(self.0)
         .zip(c.checked_add_signed(self.1))
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Side {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

impl Side {
    fn concave_delta(&self) -> Delta {
        match self {
            Side::TOP => Delta::UP.add(&Delta::RIGHT),
            Side::RIGHT => Delta::RIGHT.add(&Delta::DOWN),
            Side::BOTTOM => Delta::DOWN.add(&Delta::LEFT),
            Side::LEFT => Delta::LEFT.add(&Delta::UP),
        }
    }
    
    fn cross_outwards_delta(&self) -> Delta {
        match self {
            Side::TOP => Delta::UP,
            Side::RIGHT => Delta::RIGHT,
            Side::BOTTOM => Delta::DOWN,
            Side::LEFT => Delta::LEFT,
        }
    }
    
    fn follow_clockwise_delta(&self) -> Delta {
        self.turn_clockwise().cross_outwards_delta()
    }
    
    fn turn_counterclockwise(&self) -> Side {
        match self {
            Side::TOP => Side::LEFT,
            Side::RIGHT => Side::TOP,
            Side::BOTTOM => Side::RIGHT,
            Side::LEFT => Side::BOTTOM,
        }
    }
    
    fn turn_clockwise(&self) -> Side {
        match self {
            Side::TOP => Side::RIGHT,
            Side::RIGHT => Side::BOTTOM,
            Side::BOTTOM => Side::LEFT,
            Side::LEFT => Side::TOP,
        }
    }
}
```

Then I can use those to walk an edge once discovered. There aren't good test examples for this, but it's also an
implementation detail, so I defer adding testing to the edge counting where there are plenty of examples, and I can
come back and add tests if I need to better understand any failing counts.

```rust
impl Region {
    fn contains(&self, plot: &Option<Plot>) -> bool {
        if let Some(coord) = plot {
            self.plots.iter().contains(coord)
        } else {
            false
        }
    }
    
    fn walk_perimeter(
        &self,
        plot: Plot,
        side: Side,
        visited: &mut HashSet<(Plot, Side)>,
        edge_count: usize,
    ) -> usize {
        if !visited.insert((plot, side)) {
            return edge_count;
        }
        
        let next_concave = side.concave_delta().apply_to(plot);
        let next_straight = side.follow_clockwise_delta().apply_to(plot);
        
        if self.contains(&next_concave) {
            self.walk_perimeter(
                next_concave.unwrap(),
                side.turn_counterclockwise(),
                visited,
                edge_count + 1,
            )
        } else if self.contains(&next_straight) {
            self.walk_perimeter(next_straight.unwrap(), side, visited, edge_count)
        } else {
            self.walk_perimeter(plot, side.turn_clockwise(), visited, edge_count + 1)
        }
    }
}
```

To actually find the edges in the first place I loop through all the plots in the region and try moving in all four
directions. If that plot is not in the region, an edge has been found. Turn right to be parallel to the edge in a
clockwise direction and walk that edge as described above. Keep a common set of visited node/direction pairs, so
that when I hit that same perimeter from a different cell or direction, it doesn't get counted again.

```rust
impl Region {
    fn count_edges(&self) -> usize {
        let mut visited = HashSet::new();
        let mut edge_count = 0;
        for &plot in self.plots.iter() {
            for side in [Side::TOP, Side::RIGHT, Side::BOTTOM, Side::LEFT] {
                if !self.contains(&side.cross_outwards_delta().apply_to(plot)) {
                    edge_count += self.walk_perimeter(plot, side, &mut visited, 0)
                }
            }
        }
        
        edge_count
    }
}

#[test]
fn can_count_edges() {
    let basic = Region {
        crop: 'A',
        plots: vec![(0, 0)].into_iter().collect(),
        perimeter: 4,
    };
    assert_eq!(basic.count_edges(), 4);
    
    let region_a = Region {
        crop: 'A',
        plots: vec![(0, 0), (0, 1), (0, 2), (0, 3)].into_iter().collect(),
        perimeter: 10,
    };
    assert_eq!(region_a.count_edges(), 4);
    
    let regions = enclave_example().find_regions();
    let with_holes = regions.iter().find(|r| r.crop == 'O').unwrap();
    assert_eq!(with_holes.count_edges(), 20);
}
```

That done, there needs to be a version of the cost calculation that uses the part 2 edge-counting. There are a range
of examples to use for tests here. These caught some bugs where I'd got the directions mixed up, but they were
fairly quick to identify and fix.

```rust
impl Garden {
    fn total_fencing_cost_with_discount(&self) -> usize {
        self.find_regions()
            .iter()
            .map(|region| region.plots.len() * region.count_edges())
            .sum()
    }
}

//noinspection SpellCheckingInspection
#[test]
fn can_calculate_costs_with_discount() {
    assert_eq!(example_garden().total_fencing_cost_with_discount(), 80);
    assert_eq!(enclave_example().total_fencing_cost_with_discount(), 436);
    assert_eq!(larger_example().total_fencing_cost_with_discount(), 1206);
    
    let example_e = parse_input(
        &"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"
            .to_string(),
    );
    assert_eq!(example_e.total_fencing_cost_with_discount(), 236);
    
    let example_diagnonal = parse_input(
        &"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"
            .to_string(),
    );
    assert_eq!(example_diagnonal.total_fencing_cost_with_discount(), 368);
}
```

## Wrap up

Today had some very complex logic, there was ASCII art and everything. With a bit of maths, it probably could be
reduced a bit, but it would also make it harder to read. I think I've been able to use some expressive types
that make the code read like what it is doing. I suppose the real test is next time advent of code throws up a puzzle
like this and I have to come back here and work out how it all works.
