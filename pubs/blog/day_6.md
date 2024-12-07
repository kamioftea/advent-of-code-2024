---
day: 6
tags: [ post ]
header: 'Day 6: Guard Gallivant'
---

Today's puzzle was a large grid with a few obstructions and a patrolling guard. The task was to follow the path that
the guard takes if she turns right every time one of the obstructions blocks her path.

## Parsing the input

This style of sparse grid is a common feature in advent of code. I've learnt from past puzzles that it is more
efficient to store the positions of the features in a coordinate set, rather than trying to store the whole grid. A
thing that has always bugged me (and caused silly bugs) in grid puzzles is that the intuitive ordering is row then
column (which would be y, then x). Further y is upside down with the origin at the top. After going down a bit of a
link rabbit-hole yesterday, I ended up reading [Advent of Code: How to Leaderboard](
https://blog.vero.site/post/advent-leaderboard) and came across this tip:

> Whenever possible I will index two-dimensional tables with r (row) and c (column), as opposed to x and y, which
> “go the other way” if the input is provided as a list of lines, as is common.
>
> -- [betaveros](https://beta.vero.site/)

So I'll be trying that out today. First some types.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

type Position = (usize, usize);

#[derive(Eq, PartialEq, Debug, Clone)]
struct Lab {
    width: usize,
    height: usize,
    obstructions: HashSet<Position>,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Guard {
    position: Position,
    direction: Direction,
}
```

Then I need to walk through the input file, adding any obstruction positions to a `HashSet<Position>` and initialising
the guard, when I see her position. I end up creating a convenience function for creating new Guards, as it'll get
used a lot in testing, but `Lab` will is fine with the more verbose struct literal.

```rust
impl Guard {
    fn new(position: Position, direction: Direction) -> Guard {
        Guard {
            position,
            direction,
        }
    }
}

fn parse_input(input: &String) -> (Lab, Guard) {
    let mut lines = input.lines();
    let width = lines.next().unwrap().len();
    let height = lines.count() + 1;
    let mut guard = None;
    let mut obstructions = HashSet::new();
    
    for (row, line) in input.lines().enumerate() {
        for (column, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    obstructions.insert((row, column));
                }
                '^' => {
                    guard = Some(Guard::new((row, column), UP));
                }
                _ => (),
            }
        }
    }
    
    (
        Lab {
            width,
            height,
            obstructions,
        },
        guard.unwrap(),
    )
}

fn example_lab() -> Lab {
    Lab {
        width: 10,
        height: 10,
        obstructions: vec![
            (0, 4),
            (1, 9),
            (3, 2),
            (4, 7),
            (6, 1),
            (7, 8),
            (8, 0),
            (9, 6),
        ]
            .into_iter()
            .collect(),
    }
}

#[test]
fn can_parse_input() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        .to_string();
    
    let (lab, guard) = parse_input(&input);
    
    assert_eq!(lab, example_lab());
    assert_eq!(guard, Guard::new((6, 4), UP));
}
```

## One step at a time

The overall plan for solving this is:

* Implement the guard taking one step, accounting for obstructions
* Keep taking steps until the next position is off the grid
* Record each position visited in a `HashSet<Position>`
* The size of that set is the puzzle answer

Using `usize` coordinates makes the step logic a bit messy because negative deltas require `checked_add_signed`, but
that leads to the function returning `None` when the guard walks off the grid. I then decide to pull that out into
helper methods, and it cleans up somewhat.

```rust
impl Guard {
    // ...
    
    fn step_row(
        &self,
        delta: isize,
        &Lab { height, .. }: &Lab
    ) -> Option<Position> {
        let (row, column) = self.position;
        let new_row = row.checked_add_signed(delta).filter(|&c| c < height);
        new_row.zip(Some(column.clone()))
    }
    
    fn step_column(
        &self,
        delta: isize,
        &Lab { width, .. }: &Lab
    ) -> Option<Position> {
        let (row, column) = self.position;
        let new_column = column.checked_add_signed(delta).filter(|&c| c < width);
        Some(row.clone()).zip(new_column)
    }
    
    fn next_position(&self, lab: &Lab) -> Option<(usize, usize)> {
        match self.direction {
            UP => self.step_row(-1, lab),
            RIGHT => self.step_column(1, lab),
            DOWN => self.step_row(1, lab),
            LEFT => self.step_column(-1, lab),
        }
    }
}
```

Once the next position is determined, I need to check if the guard has collided with an obstruction. If she has, her
position won't change, but she'll turn instead.

```rust
impl Guard {
    // ...
    fn with_position(&self, position: Position) -> Guard {
        Guard {
            position,
            direction: self.direction,
        }
    }
    
    fn with_direction(&self, direction: Direction) -> Guard {
        Guard { direction, ..*self }
    }
    
    fn take_step(&self, lab: &Lab) -> Option<Guard> {
        match self.next_position(lab) {
            Some(position) if lab.obstructions.contains(&position) => {
                Some(self.with_direction(self.direction.turn()))
            }
            Some(position) => Some(self.with_position(position)),
            None => None,
        }
    }
}
```

The tests need to cover:

* Walking in each direction when there is no obstruction
* Turning in each direction when there is an obstruction
* Walking off each edge of the map.

```rust
#[test]
fn can_take_step() {
    let lab = example_lab();
    
    let examples = vec![
        (Guard::new((6, 4), UP), Some(Guard::new((5, 4), UP))),
        (Guard::new((1, 4), UP), Some(Guard::new((1, 4), RIGHT))),
        (Guard::new((1, 4), RIGHT), Some(Guard::new((1, 5), RIGHT))),
        (Guard::new((1, 8), RIGHT), Some(Guard::new((1, 8), DOWN))),
        (Guard::new((1, 8), DOWN), Some(Guard::new((2, 8), DOWN))),
        (Guard::new((6, 8), DOWN), Some(Guard::new((6, 8), LEFT))),
        (Guard::new((6, 8), LEFT), Some(Guard::new((6, 7), LEFT))),
        (Guard::new((6, 2), LEFT), Some(Guard::new((6, 2), UP))),
        (Guard::new((0, 0), UP), None),
        (Guard::new((9, 9), RIGHT), None),
        (Guard::new((9, 9), DOWN), None),
        (Guard::new((0, 0), LEFT), None),
    ];
    
    for (guard, expected) in examples {
        assert_eq!(guard.take_step(&lab), expected)
    }
}
```

Finally, I need to walk the guard through the lab until she walks off the grid, then count the unique locations
visited. I can do this with a `while let` loop. It's a bit awkward, but it works.

```rust
fn count_guard_positions(guard: &Guard, lab: &Lab) -> usize {
    let visited = &mut HashSet::new();
    let mut guard = Some(guard.clone());
    
    while let Some(current_guard) = guard {
        visited.insert((current_guard.row, current_guard.column));
        guard = current_guard.take_step(lab);
    }
    
    visited.len()
}

#[test]
fn can_count_guard_positions() {
    assert_eq!(
        count_guard_positions(&Guard::new((6, 4), UP), &example_lab()),
        41
    );
}
```

I later work out I can use [`core::iter::successors`](
https://doc.rust-lang.org/nightly/core/iter/sources/successors/fn.successors.html) to tidy this up.

```rust
fn route_iter<'a>(
    guard: &'a Guard,
    lab: &'a Lab
) -> impl Iterator<Item=Guard> + 'a {
    successors(Some(guard.clone()), |g| g.take_step(lab))
}

fn count_guard_positions(guard: &Guard, lab: &Lab) -> usize {
    route_iter(guard, &lab).map(|g| g.position).unique().count()
}
```

## You want paradoxes? 'Cause that's how you get paradoxes!

The twist is to find all the places in the lab where an obstruction could be added to cause the guard to walk in a
loop. First I decide to work out if a Lab and starting position would be a loop. This is done similarly to part 1,
but we need to store the direction the guard was facing. If it ever repeats (same position and direction), then the
guard will keep looping from that point.

```rust
fn is_loop(guard: &Guard, lab: &Lab) -> bool {
    route_iter(&guard, &lab).duplicates().next().is_some()
}

#[test]
fn can_check_if_route_loops() {
    let lab = example_lab();
    let guard = Guard::new(6, 4, UP);
    
    assert_eq!(is_loop(&guard, &lab), false);
    
    let looping_positions = vec![(6, 3), (7, 6), (7, 7), (8, 1), (8, 3), (9, 7)];
    
    for position in looping_positions {
        assert!(
            is_loop(&guard, &lab.with_obstruction(position)),
            "Should loop with an obstruction at {position:?}"
        )
    }
}
```

Next I need to try all the possible places an obstruction could go, and count those that loop

```rust
impl Lab {
    fn with_obstruction(&self, position: Position) -> Lab {
        let mut new_lab = self.clone();
        new_lab.obstructions.insert(position);
        new_lab
    }
}

fn count_obstructions_causing_loops(guard: &Guard, lab: &Lab) -> usize {
    (0..lab.height)
        .flat_map(move |r| (0..lab.width).map(move |c| (r, c)))
        .filter(|&position| will_loop(&guard, &lab.with_obstruction(position)))
        .count()
}

#[test]
fn can_count_obstructions() {
    assert_eq!(
        count_obstructions_causing_loops(&Guard::new(6, 4, UP), &example_lab()),
        6
    )
}
```

This works, but is very inefficient, taking ~79s (~6s with `cargo run --release`).

## Optimisations

The first thing I try is not copying the Lab for each attempt.

```rust
impl Lab {
    fn with_obstruction(&mut self, position: (usize, usize)) -> bool {
        self.obstructions.insert(position)
    }
    
    fn without_obstruction(&mut self, position: (usize, usize)) -> bool {
        self.obstructions.remove(&position)
    }
}

fn count_obstructions_causing_loops(guard: &Guard, lab: &Lab) -> usize {
    let mut lab = lab.clone();
    let mut counter = 0;
    for row in 0..lab.height {
        for column in 0..lab.width {
            if lab.with_obstruction((row.clone(), column.clone())) {
                if will_loop(&guard, &lab) {
                    counter += 1;
                }
                lab.without_obstruction((row, column));
            }
        }
    }
    counter
}
```

This takes it down to ~64s / 5s, but not much of an improvement. I try a variant with mutating rather than copying
the guards, but it makes no difference and I roll it back.

My next idea is that the guard's route is only going to be affected by obstructions added to the route calculated in
part 1. Additionally, I can start from the position and direction of the previous step, cutting out a bunch of work
for loops that have a long walk-in first. This is implemented by finding the next position the guard would step to,
putting an obstacle there, then checking if the remaining path ends up looping.

This took a few attempts to get right, the first few having duplicate obstruction positions, or not accounting for
the existing obstructions properly. I ended up tracking the places that extra obstacles had been placed to ignore
duplicates.

```rust
fn count_obstructions_causing_loops(guard: &Guard, lab: &Lab) -> usize {
    let mut mut_lab = lab.clone();
    let mut counter = 0;
    let mut tried = HashSet::new();
    // Can't be placed on the staring position
    tried.insert(guard.position);
    
    for guard_position in route_iter(guard, lab) {
        if let Some(position) = guard_position.next_position(&lab) {
            if tried.insert(position) && mut_lab.with_obstruction(position) {
                if is_loop(&guard_position, &mut_lab) {
                    counter += 1;
                }
                mut_lab.without_obstruction(position);
            }
        }
    }
    
    counter
}
```

This gets it down to 5s / 0.5s, which I decide is quick enough and call it there.

### Edit 7th Dec 2024 - adding `rayon` / parallelism

Having added `rayon` to [optimise day 7](../day_7/#optimisations), I decided to also parallelise the obstruction
position work. There was a bit more to do here, as I needed to:

* Revert the changes to use a mutable `Lab` when adding obstructions, and instead make immutable copies
* Rework the for-loop to be an iterator, using `unique_by` to replace the set of attempted obstruction locations.

```rust
fn count_obstructions_causing_loops(guard: &Guard, lab: &Lab) -> usize {
    route_iter(guard, lab)
        .flat_map(|g| Some(g).zip(g.next_position(lab)))
        .filter(|(_, pos)| *pos != guard.position)
        .unique_by(|(_, pos)| *pos)
        .par_bridge()
        .filter(|(g, pos)| is_loop(g, &lab.with_obstruction(*pos)))
        .count()
}
```

This resulted in a ~4-5x speed up on a 14 core processor.

## Wrap up

Today was a good day for refactoring. I got to having a slow, messy, but working solution - then reworked it until
it was fast and readable enough.
