//! This is my solution for [Advent of Code - Day 15: _Warehouse Woes_](https://adventofcode.com/2024/day/15)
//!
//! [`parse_input`] uses [`SingleWarehouse::from_str`] and [`Move::try_from`] to parse the two sections of the input.
//!
//! [`Warehouse`] holds common logic for both parts' warehouse implementations. [`Warehouse::sum_gps`] provides the
//! puzzle solution for both parts, deferring to [`WarehouseExtensions::apply_moves`], and the part specific
//! implementations of [Warehouse::move_robot] and [`Warehouse::move_box`].
//!
//! [`SingleWarehouse`] provides the implementation for part 1.
//!
//! [`DoubleWarehouse`] provides the implementation for part 2, with [`SingleWarehouse::double`] to convert the
//! representation.

use crate::day_15::Move::{Down, Left, Right, Up};
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-15-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 15.
pub fn run() {
    let contents = fs::read_to_string("res/day-15-input.txt").expect("Failed to read file");
    let (warehouse, moves) = parse_input(&contents);

    println!(
        "After applying the moves the sum of the GPS coordinates is {}",
        warehouse.apply_moves(&moves).sum_gps()
    );

    println!(
        "After applying the moves to the doubled warehouse the sum of the GPS coordinates is {}",
        warehouse.double().apply_moves(&moves).sum_gps()
    )
}

/// Represents one of the move steps of the robot
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

impl Move {
    /// Represent the move as the delta to be applied to a [`Coordinate`]
    fn delta(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        }
    }

    /// Return the [`Coordinate`] after moving the provided origin in this direction, `None` if the move is outside
    /// the warehouse.
    fn apply_to(&self, (r, c): &Coordinate, (max_r, max_c): (usize, usize)) -> Option<Coordinate> {
        let (dr, dc) = self.delta();

        let r1 = r.checked_add_signed(dr).filter(|&r| r < max_r);
        let c1 = c.checked_add_signed(dc).filter(|&c| c < max_c);

        r1.zip(c1)
    }
}

/// Coordinates of a position in the warehouse
type Coordinate = (usize, usize);

trait Warehouse {
    /// Accessor needed by [`Warehouse::sum_gps`]
    fn boxes(&self) -> HashSet<Coordinate>;
    /// Move a box in the provided direction if not blocked, pushing further boxes as needed
    fn move_box(&mut self, pos: &Coordinate, mv: &Move) -> bool;
    /// Move a robot in the provided direction if not blocked, pushing boxes as needed
    fn move_robot(&self, mv: &Move) -> Self;

    /// The "GPS" coordinates of all boxes in the [`Warehouse`]
    fn sum_gps(&self) -> usize {
        self.boxes().iter().map(|&(r, c)| 100 * r + c).sum()
    }

    /// Common logic for parsing a [`Warehouse`], used by [`SingleWarehouse::from_str`], and
    /// [`DoubleWarehouse::from_str`].
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

/// Helper to enable providing a common implementation of applying moves to `Warehouse` + `Clone`
trait WarehouseExtensions {
    fn apply_moves(&self, moves: &Vec<Move>) -> Self;
}

impl<T: Warehouse + Clone> WarehouseExtensions for T {
    /// Return a copy of this [`Warehouse`] after the robot has followed the list of moves
    fn apply_moves(&self, moves: &Vec<Move>) -> Self {
        moves
            .iter()
            .fold(self.clone(), |warehouse, mv| warehouse.move_robot(mv))
    }
}

/// Warehouse implementation of part 1
#[derive(Eq, PartialEq, Debug, Clone)]
struct SingleWarehouse {
    walls: HashSet<Coordinate>,
    boxes: HashSet<Coordinate>,
    robot: Coordinate,
    bounds: (usize, usize),
}

impl FromStr for SingleWarehouse {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (walls, boxes, robot, max_r, max_c) = SingleWarehouse::parse_warehouse(input);

        Ok(SingleWarehouse {
            walls,
            boxes,
            robot,
            bounds: (max_r + 1, max_c + 1),
        })
    }
}

impl SingleWarehouse {
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

    /// Expand this warehouse into the doubled form used for part 2
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

impl Warehouse for SingleWarehouse {
    /// Provide common access to warehouse boxes
    fn boxes(&self) -> HashSet<Coordinate> {
        self.boxes.clone()
    }

    /// Recursively move boxes, changes will only be made if there is space to move all boxes
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

    /// Apply a move to the robot in the warehouse, moving boxes if needed. This will be a no-op if the move is
    /// blocked by a wall, or any of the box moves are.
    fn move_robot(&self, mv: &Move) -> Self {
        let mut new_warehouse = self.clone();
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

#[derive(Eq, PartialEq, Debug, Clone)]
struct DoubleWarehouse {
    walls: HashSet<Coordinate>,
    boxes: HashSet<Coordinate>,
    robot: Coordinate,
    bounds: (usize, usize),
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

impl Warehouse for DoubleWarehouse {
    /// Provide common access to warehouse boxes
    fn boxes(&self) -> HashSet<Coordinate> {
        self.boxes.clone()
    }

    /// Move this box, and push any box that is part in either of the two destination squares. If this fails, some
    /// boxes may have been moved and the current warehouse should be considered invalid.
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

    /// Move the robot pushing any boxes in the way. If the box move fails the partially updated grid is discarded
    /// and an unmodified clone returned instead.
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

/// Turn the puzzle input into a [`SingleWarehouse`], and list of [`Move`]s.
fn parse_input(input: &String) -> (SingleWarehouse, Vec<Move>) {
    let (warehouse, moves) = input.split_once("\n\n").unwrap();

    (
        warehouse.parse().unwrap(),
        moves.chars().flat_map(|char| char.try_into()).collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::day_15::*;
    
    fn small_example_warehouse() -> SingleWarehouse {
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

        SingleWarehouse {
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
            Left, Up, Up, Right, Right, Right, Down, Down, Left, Down, Right, Right, Down, Left,
            Left,
        ]
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
        let move_attempted = warehouse.move_robot(&Left);

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

    #[test]
    fn can_sum_gps() {
        assert_eq!(small_example_after_moves().sum_gps(), 2028);
        assert_eq!(larger_example_after_moves().sum_gps(), 10092);
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
}
