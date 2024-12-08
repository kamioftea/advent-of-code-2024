---
day: 8
tags: [ post ]
header: 'Day 8: Resonant Collinearity'
---

Today's puzzle is finding increments along a line defined by pairs of antenna. The antenna are grouped into frequencies
by the character used to represent their location in the grid. The lines from each permutation of pairs within a group
need to be extended one unit back and one unit forward, then the unique list of nodes that generates is counted to give
the puzzle answer.

## Parsing the input

This is another sparse grid, so it makes sense to use a similar representation to the `Lab` from
[Day 6](../day_6/#parsing-the-input). The difference here is that I need to track the frequency character too, and
the flow of the lookups is from the antenna to coordinates, so no need to use a set.

```rust
type Coordinate = (usize, usize);

#[derive(Eq, PartialEq, Debug)]
struct AntennaMap {
    height: usize,
    width: usize,
    antenna: HashMap<char, Vec<Coordinate>>,
}
```

Reading the puzzle input requires splitting on lines and chars, and building the map up when a non-period character
is seen. I also separately store the width and height so we know when a node is out of bounds.

```rust
fn parse_input(input: &String) -> AntennaMap {
    let mut lines = input.lines();
    let width = lines.next().unwrap().len();
    let height = lines.count() + 1;
    let mut antenna: HashMap<char, Vec<Coordinate>> = HashMap::new();
    
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                antenna.entry(char).or_default().push((row, col))
            }
        }
    }
    
    AntennaMap {
        width,
        height,
        antenna,
    }
}

#[test]
fn can_parse_input() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
        .to_string();
    
    assert_eq!(parse_input(&input), example_map());
}

fn example_map() -> AntennaMap {
    AntennaMap {
        height: 12,
        width: 12,
        antenna: vec![
            ('0', vec![(1, 8), (2, 5), (3, 7), (4, 4)]),
            ('A', vec![(5, 6), (8, 8), (9, 9)]),
        ]
            .into_iter()
            .collect(),
    }
}
```

## Part 1 - Antinodes

Taking the line between two pairs of antenna as the unit length, the "antinodes" are the two points one more unit
length along the line in each direction. I did have to get out a pen and paper and work out the maths, but that then
converted directly into code.

The points also need to be cropped within the edges of the grid. Using `usize` gets the lower bound of `0` for free,
so I then need to filter out any beyond the upper bounds recorded during parsing.

```rust
fn find_antinodes_for_pair(
    &(r1, c1): &Coordinate,
    &(r2, c2): &Coordinate,
    (height, width): &(usize, usize),
) -> Vec<Coordinate> {
    let dr = r1 as isize - r2 as isize;
    let dc = c1 as isize - c2 as isize;
    
    vec![
        r1.checked_add_signed(dr).zip(c1.checked_add_signed(dc)),
        r2.checked_add_signed(-dr).zip(c2.checked_add_signed(-dc)),
    ]
        .iter()
        .flatten()
        .filter(|(r, c)| r < height && c < width)
        .cloned()
        .collect()
}

#[test]
fn can_find_antinodes_for_pair() {
    assert_contains_in_any_order(
        find_antinodes_for_pair(&(3, 4), &(5, 5), &(12, 12)),
        vec![(1, 3), (7, 6)],
    );
    assert_contains_in_any_order(
        find_antinodes_for_pair(&(4, 8), &(5, 5), &(12, 12)),
        vec![(6, 2), (3, 11)],
    );
    assert_contains_in_any_order(
        find_antinodes_for_pair(&(4, 8), &(5, 5), &(10, 10)),
        vec![(6, 2)],
    );
    assert_contains_in_any_order(
        find_antinodes_for_pair(&(1, 1), &(3, 3), &(10, 10)),
        vec![(5, 5)],
    );
}
```

To extend to working with all the antenna in a given frequency, I can use `Itertools::tuple_combinations` to provide
all the pairs, and then flat_map over `find_antinodes_for_pair` to get all the nodes for the group.

```rust
fn find_antinodes_for_frequency(
    antenna: &Vec<Coordinate>,
    bounds: &(usize, usize),
) -> Vec<Coordinate> {
    antenna
        .iter()
        .tuple_combinations()
        .flat_map(|(a1, a2)| find_antinodes_for_pair(a1, a2, bounds))
        .collect()
}

#[test]
fn can_find_antinodes_for_frequency() {
    assert_contains_in_any_order(
        find_antinodes_for_frequency(&vec![(3, 4), (4, 8), (5, 5)], &(10, 10)),
        vec![(1, 3), (7, 6), (6, 2), (2, 0)],
    );
}
```

The final step is to loop over each frequency, remove duplicates, and count the remaining coordinates.

```rust
fn count_antinodes_for_map(antenna_map: &AntennaMap) -> usize {
    let bounds = (antenna_map.height, antenna_map.width);
    antenna_map
        .antenna
        .values()
        .flat_map(|antenna| find_antinodes_for_frequency(antenna, &bounds))
        .unique()
        .count()
}

#[test]
fn can_count_antinodes_for_map() {
    assert_eq!(count_antinodes_for_map(&example_map()), 14);
}
```

## Part 2 - Resonant Harmonies

Part 2 is essentially the same problem, but the lines are extended until they leave the grid. I first go down a
rabbit hole trying to pass in a function that generates the iterator of points, but get into a mess of trying to
keep the borrow checker happy, and it starts to feel like I picked the wrong abstraction.

I switch to having a copy of `find_antinodes_for_pair`, and passing which to use as a function pointer.

```rust
fn find_antinodes_for_pair_with_resonant_harmonics(
    &(r1, c1): &Coordinate,
    &(r2, c2): &Coordinate,
    (height, width): &(usize, usize),
) -> Vec<Coordinate> {
    let dr = r1 as isize - r2 as isize;
    let dc = c1 as isize - c2 as isize;
    
    let increasing = iterate(0, |i| i + 1)
        .map(move |i| {
            r1.checked_add_signed(i * dr)
              .zip(c1.checked_add_signed(i * dc))
              .filter(|(r, c)| r < height && c < width)
        })
        .while_some();
    
    let decreasing = iterate(-1, |i| i - 1)
        .map(move |i| {
            r1.checked_add_signed(i * dr)
              .zip(c1.checked_add_signed(i * dc))
              .filter(|(r, c)| r < height && c < width)
        })
        .while_some();
    
    increasing.chain(decreasing).collect()
}

#[test]
fn can_find_antinodes_for_pair_with_resonant_harmonics() {
    assert_contains_in_any_order(
        find_antinodes_for_pair_with_resonant_harmonics(&(2, 3), &(3, 5), &(10, 10)),
        vec![(1, 1), (2, 3), (3, 5), (4, 7), (5, 9)],
    );
    assert_contains_in_any_order(
        find_antinodes_for_pair_with_resonant_harmonics(&(4, 3), &(3, 5), &(10, 10)),
        vec![(5, 1), (4, 3), (3, 5), (2, 7), (1, 9)],
    );
}
```

Then I update `find_antinodes_for_frequency` and `count_antinodes_for_map` to take a strategy for how to generate
the nodes for a pair: `fn(&Coordinate, &Coordinate, &(usize, usize)) -> Vec<Coordinate>`.

```rust
#[test]
fn can_find_antinodes_for_frequency_with_resonant_harmonics() {
    assert_contains_in_any_order(
        find_antinodes_for_frequency(
            find_antinodes_for_pair_with_resonant_harmonics,
            &vec![(0, 0), (1, 3), (2, 1)],
            &(10, 10),
        ),
        vec![
            (0, 0),
            (0, 5),
            (1, 3),
            (2, 1),
            (2, 6),
            (3, 9),
            (4, 2),
            (6, 3),
            (8, 4),
        ],
    );
}

assert_eq!(
    count_antinodes_for_map(
        find_antinodes_for_pair_with_resonant_harmonics,
        &example_map()
    ),
    34
);
```

That solves the puzzle but feels quite messy. I work out that part can be setup to take a means of extending the
line from one end in a direction, and applying that from each node. `Itertools::while_some` will keep taking and
unwrapping `Some` values until the first `None`, which will handle the cropping.

```rust
fn sequence_from_antenna(
    (r, c): Coordinate,
    (dr, dc): (isize, isize),
    (height, width): &(usize, usize),
) -> Vec<Coordinate> {
    iterate(0, |i| i + 1)
        .map(move |i| {
            r.checked_add_signed(i * dr)
             .zip(c.checked_add_signed(i * dc))
             .filter(|(r, c)| r < height && c < width)
        })
        .while_some()
        .collect()
}
```

The differentiator then becomes how the parts use that sequence. Part one takes just the 2nd element, part 2 takes
all of them.

```rust
fn find_antinodes_for_pair(
    (r1, c1): Coordinate,
    (r2, c2): Coordinate,
    bounds: &(usize, usize),
) -> Vec<Coordinate> {
    let dr = r1 as isize - r2 as isize;
    let dc = c1 as isize - c2 as isize;
    
    let increasing = sequence_from_antenna((r1, c1).clone(), (dr, dc).clone(), bounds);
    let decreasing = sequence_from_antenna((r2, c2), (-dr, -dc), bounds);
    
    increasing
        .iter()
        .dropping(1)
        .take(1)
        .chain(decreasing.iter().dropping(1).take(1))
        .cloned()
        .collect()
}

fn find_antinodes_for_pair_with_resonant_harmonics(
    (r1, c1): Coordinate,
    (r2, c2): Coordinate,
    bounds: &(usize, usize),
) -> Vec<Coordinate> {
    let dr = r1 as isize - r2 as isize;
    let dc = c1 as isize - c2 as isize;
    
    let increasing = sequence_from_antenna((r1, c1).clone(), (dr, dc).clone(), bounds);
    let decreasing = sequence_from_antenna((r2, c2), (-dr, -dc), bounds);
    
    increasing
        .iter()
        .chain(decreasing.iter())
        .cloned()
        .collect()
}
```

There's still a common pattern here, and instead of passing pointers to `find_antinodes_for_pair` and
`find_antinodes_for_pair_with_resonant_harmonics` I can instead pass something that selects the elements I want from
the sequence of nodes.

```rust
type SequenceModifier = fn(Vec<Coordinate>) -> Vec<Coordinate>;

fn antinode_pair_sequence_modifier(coordinate_sequence: Vec<Coordinate>) -> Vec<Coordinate> {
    coordinate_sequence
        .into_iter()
        .dropping(1)
        .take(1)
        .collect()
}

fn resonant_harmonies_sequence_modifier(coordinate_sequence: Vec<Coordinate>) -> Vec<Coordinate> {
    coordinate_sequence
}

// Now handles both parts
fn find_antinodes_for_pair(
    (r1, c1): Coordinate,
    (r2, c2): Coordinate,
    bounds: &(usize, usize),
    sequence_modifier: SequenceModifier,
) -> Vec<Coordinate> {
    let dr = r1 as isize - r2 as isize;
    let dc = c1 as isize - c2 as isize;
    
    let increasing = sequence_from_antenna((r1, c1).clone(), (dr, dc).clone(), bounds);
    let decreasing = sequence_from_antenna((r2, c2), (-dr, -dc), bounds);
    
    [sequence_modifier(increasing), sequence_modifier(decreasing)].concat()
}
```

## Wrap up

I wasted a lot of time finding the right abstraction today. It's still not perfect, and it feels like with a better
understanding of rust I could be passing around `Iterator`s rather than `Vec`s, but I couldn't satisfy the borrow
checker when I wanted to pass around closures/function pointers as part of that. It's quick enough that it doesn't
really matter, though.
