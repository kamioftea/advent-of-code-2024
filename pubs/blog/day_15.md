---
day: 15
tags: [ post ]
header: 'Day 15: Warehouse Woes'
---

Today is pushing boxes around a warehouse, the reducing their positions to the puzzle solution. There's quite a lot of
steps to get to the solution, but each seems well-defined and testable.

## Parsing the input

The warehouse grid is relatively sparse, but I need to keep track of multiple things. I decide to implement this as
multiple sets. I pondered not including the wall around the edge to make the sets of walls smaller, but it was
awkward having to have special cases to handle it, so it seemed easier to include them. I can always change that
later if needed.

```rust
type Coordinate = (usize, usize);

#[derive(Eq, PartialEq, Debug, Clone)]
struct Warehouse {
    walls: HashSet<Coordinate>,
    boxes: HashSet<Coordinate>,
    robot: Coordinate,
    bounds: (usize, usize),
}

impl FromStr for Warehouse {
    type Err = ();
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot = (0, 0);
        let mut max_r = 0;
        let mut max_c = 0;
        
        for (r, row) in input.lines().enumerate() {
            for (c, char) in row.chars().enumerate() {
                match char {
                    '#' => {
                        walls.insert((r, c));
                    }
                    'O' => {
                        boxes.insert((r, c));
                    }
                    '@' => robot = (r, c),
                    _ => {}
                }
                max_c = max_c.max(c);
            }
            max_r = max_r.max(r);
        }
        
        Ok(Warehouse {
            walls,
            boxes,
            robot,
            bounds: (max_r + 1, max_c + 1),
        })
    }
}
```

I also need to store the list of moves.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Move {
    type Error = ();
    
    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            '^' => Ok(Up),
            '>' => Ok(Right),
            'v' => Ok(Down),
            '<' => Ok(Left),
            _ => Err(()),
        }
    }
}
```

Finally split the input on the blank line and parse both parts.

```rust
fn parse_input(input: &String) -> (Warehouse, Vec<Move>) {
    let (warehouse, moves) = input.split_once("\n\n").unwrap();
    
    (
        warehouse.parse().unwrap(),
        moves.chars().flat_map(|char| char.try_into()).collect(),
    )
}

fn small_example_warehouse() -> Warehouse {
    #[rustfmt::skip]
    let walls = vec![
        (0, 0),(0, 1),(0, 2),(0, 3),(0, 4),(0, 5),(0, 6),(0, 7),
        (1, 0),                                          (1, 7),
        (2, 0),(2, 1),                                   (2, 7),
        (3, 0),                                          (3, 7),
        (4, 0),       (4, 2),                            (4, 7),
        (5, 0),                                          (5, 7),
        (6, 0),                                          (6, 7),
        (7, 0),(7, 1),(7, 2),(7, 3),(7, 4),(7, 5),(7, 6),(7, 7),
    ];
    
    Warehouse {
        walls: walls.into_iter().collect(),
        boxes: vec![(1, 3), (1, 5), (2, 4), (3, 4), (4, 4), (5, 4)]
            .into_iter()
            .collect(),
        robot: (2, 2),
        bounds: (8, 8),
    }
}

fn small_example_moves() -> Vec<Move> {
    vec![
        Left, Up, Up, Right, Right, Right, Down, Down, Left, Down,
        Right, Right, Down, Left, Left,
    ]
}

#[test]
fn can_parse_input() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
        .to_string();
    
    let (warehouse, moves) = parse_input(&input);
    assert_eq!(warehouse, small_example_warehouse());
    
    assert_eq!(moves, small_example_moves());
}
```

I also add a way to visualise the warehouse to help debug issues.

```rust
impl Warehouse {
    #[allow(dead_code)]
    fn render(&self) {
        for r in 0..self.bounds.0 {
            for c in 0..self.bounds.1 {
                if self.walls.contains(&(r, c)) {
                    print!("#");
                } else if self.boxes.contains(&(r, c)) {
                    print!("O");
                } else if self.robot == (r, c) {
                    print!("@");
                } else {
                    print!(" ")
                }
            }
            println!()
        }
        println!()
    }
}
```

## Part 1 - Push and shove

First I'll focus on moving the robot once. I add some helpers to `Coordinate`

```rust
impl Move {
    fn delta(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        }
    }
    
    fn apply_to(
        &self,
        (r, c): &Coordinate,
        (max_r, max_c): (usize, usize)
    ) -> Option<Coordinate> {
        let (dr, dc) = self.delta();
        
        let r1 = r.checked_add_signed(dr).filter(|&r| r < max_r);
        let c1 = c.checked_add_signed(dc).filter(|&c| c < max_c);
        
        r1.zip(c1)
    }
}
```

I decide to make copies of the warehouse for each step as it makes things clearer, but it should be possible to
switch to a mutable version later if the solution ends up taking too long. I build up the move logic in TTD cycles:

- Move robot into an empty space
- Move into a wall (no-op)
- Push a single block
- Push multiple blocks
- Push a block into a wall (no-op)

Within the process of moving a robot, if I need to move a box I recursively call `move_box` with a mutable box,
which only updates the warehouse after it's reached an empty space, moving the boxes in sequence as the recursive
stack unwinds. This way if it turns out the boxes can't be moved, no change is made.

```rust
impl Warehouse {
    fn move_box(&mut self, pos: &Coordinate, mv: &Move) -> bool {
        if let Some(new_pos) = mv.apply_to(pos, self.bounds) {
            if self.walls.contains(&new_pos) {
                false
            } else if self.boxes.contains(&new_pos) && !self.move_box(&new_pos, &mv) {
                false
            } else {
                self.boxes.remove(&pos);
                self.boxes.insert(new_pos)
            }
        } else {
            false
        }
    }
    
    fn move_robot(&self, mv: &Move) -> Self {
        let mut new_warehouse = self.clone();
        if let Some(new_pos) = mv.apply_to(self.robot, self.bounds) {
            if let Some(new_pos) = mv.apply_to(&self.robot, self.bounds) {
                if self.walls.contains(&new_pos) {
                    return new_warehouse;
                }
                
                if self.boxes.contains(&new_pos) && !new_warehouse.move_box(&new_pos, &mv) {
                    return new_warehouse;
                }
                
                new_warehouse.robot = new_pos
            }
            
            new_warehouse
        }
    }
}

#[test]
fn can_apply_move_into_empty() {
    let warehouse = small_example_warehouse();
    let moved_up = warehouse.move_robot(&Up);
    
    assert_eq!(moved_up.walls, warehouse.walls);
    assert_eq!(moved_up.boxes, warehouse.boxes);
    assert_eq!(moved_up.robot, (1, 2));
    
    let moved_right = warehouse.move_robot(&Right);
    
    assert_eq!(moved_right.walls, warehouse.walls);
    assert_eq!(moved_right.boxes, warehouse.boxes);
    assert_eq!(moved_right.robot, (2, 3));
    
    let moved_down = warehouse.move_robot(&Down);
    
    assert_eq!(moved_down.walls, warehouse.walls);
    assert_eq!(moved_down.boxes, warehouse.boxes);
    assert_eq!(moved_down.robot, (3, 2));
    
    let moved_left = moved_up.move_robot(&Left);
    
    assert_eq!(moved_left.walls, warehouse.walls);
    assert_eq!(moved_left.boxes, warehouse.boxes);
    assert_eq!(moved_left.robot, (1, 1));
}

#[test]
fn move_is_blocked_by_walls() {
    let warehouse = small_example_warehouse();
    let move_attempted = warehouse.move_robot(Left);
    
    assert_eq!(move_attempted, warehouse);
}

#[test]
fn can_push_boxes() {
    let warehouse = small_example_warehouse();
    
    let mut expected_boxes = warehouse.boxes.clone();
    expected_boxes.remove(&(1, 3));
    expected_boxes.insert((1, 4));
    
    let single_box_moved = warehouse.move_robot(&Up).move_robot(&Right);
    
    assert_eq!(single_box_moved.walls, warehouse.walls);
    assert_eq!(single_box_moved.boxes, expected_boxes);
    assert_eq!(single_box_moved.robot, (1, 3));
    
    let multi_boxes_moved = single_box_moved.move_robot(&Right);
    
    expected_boxes.remove(&(1, 4));
    expected_boxes.insert((1, 6));
    
    assert_eq!(multi_boxes_moved.walls, warehouse.walls);
    assert_eq!(multi_boxes_moved.boxes, expected_boxes);
    assert_eq!(multi_boxes_moved.robot, (1, 4));
    
    let boxes_blocked = multi_boxes_moved.move_robot(&Right);
    
    assert_eq!(boxes_blocked, multi_boxes_moved);
}
```

That working, I can use `fold` to apply the list of moves in sequence.

```rust
impl Warehouse {
    fn apply_moves(&self, moves: &Vec<Move>) -> Self {
        moves
            .iter()
            .fold(self.clone(), |warehouse, mv| warehouse.move_robot(mv))
    }
}

fn small_example_after_moves() -> SingleWarehouse {
    SingleWarehouse::from_str(
        "########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########",
    )
        .unwrap()
}

//noinspection SpellCheckingInspection
fn larger_example() -> (SingleWarehouse, Vec<Move>) {
    parse_input(
        &"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
            .to_string(),
    )
}

fn larger_example_after_moves() -> SingleWarehouse {
    SingleWarehouse::from_str(
        "##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########",
    )
        .unwrap()
}

#[test]
fn can_apply_move_list() {
    let small_warehouse = small_example_warehouse();
    let small_moves = small_example_moves();
    
    assert_eq!(
        small_warehouse.apply_moves(&small_moves),
        small_example_after_moves()
    );
    
    let (larger_warehouse, larger_moves) = larger_example();
    
    assert_eq!(
        larger_warehouse.apply_moves(&larger_moves),
        larger_example_after_moves()
    );
}
```

Finally, I need to take the final positions of the boxes, convert them into "GPS" coordinates and sum the result.

```rust
impl Warehouse {
    fn sum_gps(&self) -> usize {
        self.boxes.iter().map(|&(r, c)| 100 * r + c).sum()
    }
}

#[test]
fn can_sum_gps() {
    assert_eq!(small_example_after_moves().sum_gps(), 2028);
    assert_eq!(larger_example_after_moves().sum_gps(), 10092);
}
```

## Part 2 - Double trouble

This was honestly quite a relief. I'd been expecting something like find the list of moves that gives the lowest sum
of GPS co-ordinates. The twist in this case was doubling the size of the warehouse, and importantly the boxes. This
means they can now be offset from each other and pushing a box could end up with that box pushing two more, and so on.

A double warehouse is a lot like a warehouse, and mostly differs in how it is created and how it handles boxes. I'll
be using traits to hold the common functionality, but first I'll need a DoubleWarehouse struct to hold the
differences. It seems best to take the already parsed SingleWarehouse (renamed from part 1) and double it. I keep
only one entry for each box, the left-most cell it occupies, and will handle checking the other half in the movement
logic.

```rust
#[derive(Eq, PartialEq, Debug, Clone)]
struct DoubleWarehouse {
    walls: HashSet<Coordinate>,
    boxes: HashSet<Coordinate>,
    robot: Coordinate,
    bounds: (usize, usize),
}

impl SingleWarehouse {
    fn double(&self) -> DoubleWarehouse {
        let walls = self
            .walls
            .iter()
            .flat_map(|&(r, c)| vec![(r, c * 2), (r, c * 2 + 1)])
            .collect();
        let boxes = self.boxes.iter().map(|&(r, c)| (r, c * 2)).collect();
        let robot = (self.robot.0, self.robot.1 * 2);
        let bounds = (self.bounds.0, self.bounds.1 * 2);
        
        DoubleWarehouse {
            walls,
            boxes,
            robot,
            bounds,
        }
    }
}

fn example_to_double() -> SingleWarehouse {
    SingleWarehouse::from_str(
        "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######",
    )
        .unwrap()
}

#[test]
fn can_double_warehouse() {
    let warehouse = example_to_double();
    let double_warehouse = warehouse.double();
    
    assert_eq!(double_warehouse.walls.len(), 50);
    
    assert!(
        double_warehouse.walls.contains(&(1, 8)),
        "Inner wall should have first half at (1,8)"
    );
    assert!(
        double_warehouse.walls.contains(&(1, 9)),
        "Inner wall should have second half at (1,9)"
    );
    
    assert_eq!(double_warehouse.robot, (3, 10));
    
    let expected_boxes = vec![(3, 6), (3, 8), (4, 6)].into_iter().collect();
    assert_eq!(double_warehouse.boxes, expected_boxes)
}
```

That done, I move `sum_gps` and `apply_moves` into a shared trait `Warehouse`. Stubs for  `move_robot` and `move_box`
are added to the trait to be provided by implementations, as well as an accessor for `boxes` that `sum_gps` needs.
Apply moves needs to be able to clone a `Warehouse`, so there needs to be some boilerplate wrangling to provide a
common implemention for anything than implements both `Warehouse` and `Clone`.

```rust
trait Warehouse {
    fn boxes(&self) -> HashSet<Coordinate>;
    fn move_box(&mut self, pos: &Coordinate, mv: &Move) -> bool;
    fn move_robot(&self, mv: &Move) -> Self;
    
    fn sum_gps(&self) -> usize {
        self.boxes().iter().map(|&(r, c)| 100 * r + c).sum()
    }
}

trait WarehouseExtensions {
    fn apply_moves(&self, moves: &Vec<Move>) -> Self;
}

impl<T: Warehouse + Clone> WarehouseExtensions for T {
    fn apply_moves(&self, moves: &Vec<Move>) -> Self {
        moves
            .iter()
            .fold(self.clone(), |warehouse, mv| warehouse.move_robot(mv))
    }
}
```

I move the existing `move_box`, and `move_robot` into an `impl Warehouse for SingleWarehouse` block, and add the
accessor for `boxes`. All the tests are still passing, so I can move on to `impl Warehouse for DoubleWarehouse`.

Moving boxes could now push one or two boxes, and the one box could start in any of three places when pushing
vertically:

- Left of the new position, so the left of this box pushes on the right of the other,
- In the new position, so the whole of this box pushes on the whole of the other,
- Right of the new position, so the right of this box pushes on the left of the other.

I could have different logic for Up/Down, but if I ignore the position of the current box, the above logic will
check the right place, and that keeps things simpler.

The only other issue here is that the potential branching of box movement means that one half could have been moved
on rewinding that recursive call, only for the second branch to fail. I could do it in two passes - check then move,
but it seems simpler to have `move_robot` return a new clone of the unchanged source warehouse if the overall box
move fails.

The other change to `move_robot` is it also now needs to check two places for the start of a box, the target
location, and one to the left of that. The wall round the edge here helps as the robot is blocked from moving
anywhere that would cause it to try to check a space outside the bounds of the warehouse.

```rust
impl Warehouse for DoubleWarehouse {
    fn boxes(&self) -> HashSet<Coordinate> {
        self.boxes.clone()
    }
    
    fn move_box(&mut self, pos: &Coordinate, mv: &Move) -> bool {
        if let Some(left_new_pos) = mv.apply_to(pos, self.bounds) {
            let right_new_pos = (left_new_pos.0, left_new_pos.1 + 1);
            let possible_blocking_boxes = [
                (left_new_pos.0, left_new_pos.1 - 1),
                left_new_pos,
                right_new_pos,
            ];
            
            if self.walls.contains(&left_new_pos) || self.walls.contains(&right_new_pos) {
                false
            } else if possible_blocking_boxes
                .iter()
                .filter(|&maybe_blocker| maybe_blocker != pos)
                .all(|blocker| !self.boxes.contains(blocker) || self.move_box(blocker, mv))
            {
                self.boxes.remove(&pos);
                self.boxes.insert(left_new_pos)
            } else {
                false
            }
        } else {
            false
        }
    }
    
    fn move_robot(&self, mv: &Move) -> Self {
        let mut new_warehouse = self.clone();
        if let Some(new_pos) = mv.apply_to(&self.robot, self.bounds) {
            if self.walls.contains(&new_pos) {
                return new_warehouse;
            }
            
            let possible_start_of_box = (new_pos.0, new_pos.1 - 1);
            if (self.boxes.contains(&new_pos) && !new_warehouse.move_box(&new_pos, &mv))
                || (self.boxes.contains(&possible_start_of_box)
                && !new_warehouse.move_box(&possible_start_of_box, &mv))
            {
                // move_boxes may partially apply some moves
                return self.clone();
            }
            
            new_warehouse.robot = new_pos;
        }
        
        new_warehouse
    }
}

#[test]
fn can_move_boxes_in_double_warehouse() {
    let start = example_to_double().double();
    
    let expected_boxes = vec![(3, 5), (3, 7), (4, 6)].into_iter().collect();
    let after_left = start.move_robot(&Left);
    assert_eq!(after_left.robot, (3, 9));
    assert_eq!(after_left.boxes, expected_boxes);
    
    let expected_boxes = vec![(2, 5), (2, 7), (3, 6)].into_iter().collect();
    let after_up = after_left.apply_moves(&vec![Down, Down, Left, Left, Up]);
    assert_eq!(after_up.robot, (4, 7));
    assert_eq!(after_up.boxes, expected_boxes);
}
```

Everything else is already implemented, but I'd like to include some tests from the sample input. I'd rather
be able to parse the sample doubled warehouse than write out the internal implementation. I pull out the string
parsing from the `impl SingleWarehouse` into the `Warehouse` trait. I need to treat both `O` and `[`  as a box
location, `]` is ignored like `.`.

```rust
trait Warehouse {
    fn parse_warehouse(
        input: &str,
    ) -> (
        HashSet<(usize, usize)>,
        HashSet<(usize, usize)>,
        (usize, usize),
        usize,
        usize,
    ) {
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot = (0, 0);
        let mut max_r = 0;
        let mut max_c = 0;
        
        for (r, row) in input.lines().enumerate() {
            for (c, char) in row.chars().enumerate() {
                match char {
                    '#' => {
                        walls.insert((r, c));
                    }
                    '[' | 'O' => {
                        boxes.insert((r, c));
                    }
                    '@' => robot = (r, c),
                    _ => {}
                }
                max_c = max_c.max(c);
            }
            max_r = max_r.max(r);
        }
        (walls, boxes, robot, max_r, max_c)
    }
}

impl FromStr for DoubleWarehouse {
    type Err = ();
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (walls, boxes, robot, max_r, max_c) = DoubleWarehouse::parse_warehouse(input);
        
        Ok(DoubleWarehouse {
            walls,
            boxes,
            robot,
            bounds: (max_r + 1, max_c + 1),
        })
    }
}

#[test]
fn can_parse_double_warehouse() {
    let actual = DoubleWarehouse::from_str(
        "##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############",
    )
        .unwrap();
    
    let expected = example_to_double().double();
    
    assert_eq!(actual, expected);
}
```

Then I can test the double warehouse on the larger example.

```rust
#[test]
fn can_apply_moves_to_double_warehouse() {
    let (larger_warehouse, larger_moves) = larger_example();
    let actual = larger_warehouse.double().apply_moves(&larger_moves);
    
    let expected = DoubleWarehouse::from_str(
        "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################",
    )
        .unwrap();
    
    assert_eq!(actual, expected);
    assert_eq!(actual.sum_gps(), 9021);
}
```

## Wrap up

Today ended up being a lot of code, but each individual bit was moving towards a whole, and I got into a good flow
state, I'm glad it was a weekend though. It was good to remind myself about some of the intricacies of rust traits,
which I'm sure I'll find more usage for in later puzzles.

## Minor update 16th December - Supertraits

I found a better way of requiring clone for `Warehouse::apply_moves` than using `WarehouseExtensions`. I failed to
find the supertrait syntax when looking yesterday. Using this I can require anything that implements `Warehouse`
also implements `Clone`, and move `apply_moves` into the `impl Warehouse` block.

```rust
trait Warehouse: Clone {
    fn apply_moves(&self, moves: &Vec<Move>) -> Self {
        moves
            .iter()
            .fold(self.clone(), |warehouse, mv| warehouse.move_robot(mv))
    }
}
```
