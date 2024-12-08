---
day: 4
tags: [ post ]
header: 'Day 4: Ceres Search'
---

Today the task is to find all instances of the word `XMAS` in a wordsearch grid. Grid puzzles are a staple of advent
of code, so I was expecting one to turn up eventually. There will likely be more later. This also had me reaching
for a struct, and an `impl` block. It feels more natural to represent a grid as a specific thing. With hindsight my
implementation could have used `type Wordsearch = Vec<Vec<char>>`.

## Parse the input

First, I'll define the desired struct. It has been more performant to store this as a single level `Vec<char>` in
the past, and use accessor methods to turn coordinates into an index, but it's not worth doing that unless I
actually hit issues with performance.

```rust
#[derive(Eq, PartialEq, Debug)]
struct Wordsearch {
    cells: Vec<Vec<char>>,
}
```

Then I'll implement a FromStr so that I can parse the puzzle input into a `Wordsearch`. This involve

```rust
impl FromStr for Wordsearch {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s.lines().map(|l| l.chars().collect()).collect();
        
        Ok(Wordsearch { cells })
    }
}

fn example_wordsearch() -> Wordsearch {
    let cells = vec![
        vec!['.', '.', 'X', '.', '.', '.'],
        vec!['.', 'S', 'A', 'M', 'X', 'M'],
        vec!['.', 'A', '.', '.', 'A', '.'],
        vec!['X', 'M', 'A', 'S', '.', 'S'],
        vec!['.', 'X', '.', '.', '.', '.'],
    ];
    
    Wordsearch { cells }
}

#[test]
fn can_parse_input() {
    let input = "..X...
.SAMXM
.A..A.
XMAS.S
.X....";
    
    assert_eq!(Wordsearch::from_str(input), Ok(example_wordsearch()));
}
```

## Searching for \[the true meaning of] XMAS

The ask is to find all instances of the word XMAS in the wordsearch, along either axis or either diagonal, either
forwards or backwards. My thinking here is this can be broken down into:

- Find the coordinates of all the `X`'s
- For each find the eight 4-letter words starting at that `X`
- Flatten that out into one big list and count the `XMAS`es

To find all the `X`s I used nested `for` loops and a mutable list to collect the matching coordinates. Doing this
with iterators leads to lifetime issues because of the way lambdas capture variables.

```rust
impl Wordsearch {
    fn find_all(&self, letter: &char) -> Vec<CellCoords> {
        let mut coords = Vec::new();
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell == letter {
                    coords.push((x, y))
                }
            }
        }
        
        coords
    }
}

#[test]
fn can_find_all_xs() {
    assert_eq!(
        example_wordsearch().find_all(&'X'),
        vec![(2, 0), (4, 1), (0, 3), (1, 4)]
    )
}
```

Turning those into words involves some smaller steps:

* For each direction represented as a delta step in that direction from the start
* If that is a valid grid cell, get the character in that cell
* Merge the found cells for a direction into a string
* Return the list of eight strings

```rust
fn apply_delta(
    (x, y): &CellCoords,
    (dx, dy): &(isize, isize),
    magnitude: usize,
) -> Option<CellCoords> {
    x.checked_add_signed(dx * magnitude as isize)
     .zip(y.checked_add_signed(dy * magnitude as isize))
}

impl Wordsearch {
    // ...
    fn char_at(&self, &(x, y): &CellCoords) -> Option<&char> {
        self.cells.get(y).and_then(|row| row.get(x))
    }
    
    fn get_word(
        &self, start: &CellCoords,
        length: usize,
        delta: &(isize, isize)
    ) -> String {
        (0..length)
            .flat_map(|magnitude| apply_delta(start, delta, magnitude))
            .flat_map(|coord| self.char_at(&coord))
            .join("")
    }
    
    fn words_from(&self, start: &CellCoords, length: usize) -> Vec<String> {
        let deltas = vec![
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ];
        deltas
            .iter()
            .map(|delta| self.get_word(start, length, delta))
            .collect()
    }
}

#[test]
fn can_find_words() {
    assert_eq!(
        example_wordsearch().words_from(&(2, 0), 4),
        vec![
            "X..".to_string(),
            "X".to_string(),
            "X".to_string(),
            "X".to_string(),
            "X...".to_string(),
            "XMAS".to_string(),
            "XA.A".to_string(),
            "XS.".to_string()
        ]
    )
}
```

The behavior of `Option::zip`, `Option::flat_map`, `Vec::get`, `Iterator::flat_map`, and `Iterator::join` work well
together to handle the failure conditions and return a variable length string, cropped where the word overlaps the
edge of the grid.

To turn that list of words into the puzzle solution I can count all the instances of `XMAS`, using the list of `X`
coordinates as the starting points.

```rust
impl Wordsearch {
    // ...
    fn word_count(&self, search: &String) -> usize {
        let start = search.chars().next().expect("Word must not be empty");
        self.find_all(&start)
            .iter()
            .flat_map(|coord| self.words_from(coord, search.len()))
            .filter(|word| word == search)
            .count()
    }
}

fn bigger_example() -> Wordsearch {
    Wordsearch::from_str(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    )
        .unwrap()
}

#[test]
fn can_count_xmasses() {
    assert_eq!(example_wordsearch().word_count(&"XMAS".to_string()), 4);
    assert_eq!(bigger_example().word_count(&"XMAS".to_string()), 18)
}
```

## Part 2 - X marks the spot

I was expecting some flavour of expanding or wrapping round the grid, but instead I have a different pattern to match.

I ponder about finding the `A`s and then comparing the two pairs of diagonal corners. Then I notice that as I
already have a word finder, it'll be easier to use that and take the words from each top corner and see if they spell
`MAS` or `SAM`.

```rust
impl Wordsearch {
    // ...
    fn is_x_mas(&self, coord: &CellCoords) -> bool {
        let top_left =
            apply_delta(coord, &(-1, -1), 1)
                .map(|start| self.get_word(&start, 3, &(1, 1)));
        let top_right =
            apply_delta(coord, &(1, -1), 1)
                .map(|start| self.get_word(&start, 3, &(-1, 1)));
        
        self.char_at(coord) == Some(&'A')
            && (top_left == Some("MAS".to_string())
            || top_left == Some("SAM".to_string())
        )
            && (top_right == Some("MAS".to_string())
            || top_right == Some("SAM".to_string())
        )
    }
}

#[test]
fn can_check_for_an_x_mas() {
    assert_eq!(example_wordsearch().is_x_mas(&(1, 1)), false);
    assert_eq!(example_wordsearch().is_x_mas(&(4, 2)), true);
}
```

Turning that into a solution is very similar to part 1:

- Find all the `A`s
- Count those that are the centre of an `X-MAS`

```rust
impl Wordsearch {
    // ...
    fn count_x_masses(&self) -> usize {
        self.find_all(&'A')
            .iter()
            .filter(|coord| self.is_x_mas(coord))
            .count()
    }
}

#[test]
fn can_count_x_masses() {
    assert_eq!(example_wordsearch().count_x_masses(), 1);
    assert_eq!(bigger_example().count_x_masses(), 9);
}
```

## Wrap up

I have mixed feelings about my solutions today. I'm quite happy about how the plan broke down into smaller parts,
and I think the code conveys its intent quite well. I'm less happy about using strings all over the place to avoid
the borrow checker. It doesn't feel very "Rusty". I'd like to come back with a bit more time and have another stab
at implementing this with some custom iterators and try to make it a bit more performant. I'll update here if I
manage to.
