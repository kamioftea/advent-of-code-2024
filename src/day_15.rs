//! This is my solution for [Advent of Code - Day 15: _Warehouse Woes_](https://adventofcode.com/2024/day/15)
//!
//!

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
    let (grid, moves) = parse_input(&contents);

    println!(
        "After applying the moves the sum of the GPS coordinates is {}",
        grid.apply_moves(&moves).sum_gps()
    )
}

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
    fn delta(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        }
    }

    fn apply_to(&self, (r, c): &Coordinate, (max_r, max_c): (usize, usize)) -> Option<Coordinate> {
        let (dr, dc) = self.delta();

        let r1 = r.checked_add_signed(dr).filter(|&r| r < max_r);
        let c1 = c.checked_add_signed(dc).filter(|&c| c < max_c);

        r1.zip(c1)
    }
}

type Coordinate = (usize, usize);

#[derive(Eq, PartialEq, Debug, Clone)]
struct Grid {
    walls: HashSet<Coordinate>,
    boxes: HashSet<Coordinate>,
    robot: Coordinate,
    bounds: (usize, usize),
}

impl FromStr for Grid {
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

        Ok(Grid {
            walls,
            boxes,
            robot,
            bounds: (max_r + 1, max_c + 1),
        })
    }
}

impl Grid {
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
        let mut new_grid = self.clone();
        if let Some(new_pos) = mv.apply_to(&self.robot, self.bounds) {
            if self.walls.contains(&new_pos) {
                return new_grid;
            }

            if self.boxes.contains(&new_pos) && !new_grid.move_box(&new_pos, &mv) {
                return new_grid;
            }

            new_grid.robot = new_pos
        }

        new_grid
    }

    fn apply_moves(&self, moves: &Vec<Move>) -> Self {
        moves
            .iter()
            .fold(self.clone(), |grid, mv| grid.move_robot(mv))
    }

    fn sum_gps(&self) -> usize {
        self.boxes.iter().map(|&(r, c)| 100 * r + c).sum()
    }

    fn double(&self) -> Grid {
        let walls = self
            .walls
            .iter()
            .flat_map(|&(r, c)| vec![(r, c * 2), (r, c * 2 + 1)])
            .collect();
        let boxes = self.boxes.iter().map(|&(r, c)| (r, c * 2)).collect();
        let robot = (self.robot.0, self.robot.1 * 2);
        let bounds = (self.bounds.0, self.bounds.1 * 2);

        Grid {
            walls,
            boxes,
            robot,
            bounds,
        }
    }
}

fn parse_input(input: &String) -> (Grid, Vec<Move>) {
    let (grid, moves) = input.split_once("\n\n").unwrap();

    (
        grid.parse().unwrap(),
        moves.chars().flat_map(|char| char.try_into()).collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::day_15::*;
    
    fn small_example_grid() -> Grid {
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

        Grid {
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

        let (grid, moves) = parse_input(&input);
        assert_eq!(grid, small_example_grid());

        assert_eq!(moves, small_example_moves());
    }

    #[test]
    fn can_apply_move_into_empty() {
        let grid = small_example_grid();
        let moved_up = grid.move_robot(&Up);

        assert_eq!(moved_up.walls, grid.walls);
        assert_eq!(moved_up.boxes, grid.boxes);
        assert_eq!(moved_up.robot, (1, 2));

        let moved_right = grid.move_robot(&Right);

        assert_eq!(moved_right.walls, grid.walls);
        assert_eq!(moved_right.boxes, grid.boxes);
        assert_eq!(moved_right.robot, (2, 3));

        let moved_down = grid.move_robot(&Down);

        assert_eq!(moved_down.walls, grid.walls);
        assert_eq!(moved_down.boxes, grid.boxes);
        assert_eq!(moved_down.robot, (3, 2));

        let moved_left = moved_up.move_robot(&Left);

        assert_eq!(moved_left.walls, grid.walls);
        assert_eq!(moved_left.boxes, grid.boxes);
        assert_eq!(moved_left.robot, (1, 1));
    }

    #[test]
    fn move_is_blocked_by_walls() {
        let grid = small_example_grid();
        let move_attempted = grid.move_robot(&Left);

        assert_eq!(move_attempted, grid);
    }

    #[test]
    fn can_push_boxes() {
        let grid = small_example_grid();

        let mut expected_boxes = grid.boxes.clone();
        expected_boxes.remove(&(1, 3));
        expected_boxes.insert((1, 4));

        let single_box_moved = grid.move_robot(&Up).move_robot(&Right);

        assert_eq!(single_box_moved.walls, grid.walls);
        assert_eq!(single_box_moved.boxes, expected_boxes);
        assert_eq!(single_box_moved.robot, (1, 3));

        let multi_boxes_moved = single_box_moved.move_robot(&Right);

        expected_boxes.remove(&(1, 4));
        expected_boxes.insert((1, 6));

        assert_eq!(multi_boxes_moved.walls, grid.walls);
        assert_eq!(multi_boxes_moved.boxes, expected_boxes);
        assert_eq!(multi_boxes_moved.robot, (1, 4));

        let boxes_blocked = multi_boxes_moved.move_robot(&Right);

        assert_eq!(boxes_blocked, multi_boxes_moved);
    }

    #[test]
    fn can_apply_move_list() {
        let small_grid = small_example_grid();
        let small_moves = small_example_moves();

        assert_eq!(
            small_grid.apply_moves(&small_moves),
            small_example_after_moves()
        );

        let (larger_grid, larger_moves) = parse_input(
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
        );

        assert_eq!(
            larger_grid.apply_moves(&larger_moves),
            larger_example_after_moves()
        );
    }

    fn larger_example_after_moves() -> Grid {
        Grid::from_str(
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

    fn small_example_after_moves() -> Grid {
        Grid::from_str(
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

    #[test]
    fn can_sum_gps() {
        assert_eq!(small_example_after_moves().sum_gps(), 2028);
        assert_eq!(larger_example_after_moves().sum_gps(), 10092);
    }

    #[test]
    fn can_double_grid() {
        let grid = Grid::from_str(
            "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######",
        )
        .unwrap();

        // ##############
        // ##......##..##
        // ##..........##
        // ##....[][]@.##
        // ##....[]....##
        // ##..........##
        // ##############
        let double_grid = grid.double();

        assert_eq!(double_grid.walls.len(), 50);

        assert!(
            double_grid.walls.contains(&(1, 8)),
            "Inner wall should have first half at (1,8)"
        );
        assert!(
            double_grid.walls.contains(&(1, 9)),
            "Inner wall should have second half at (1,9)"
        );

        assert_eq!(double_grid.robot, (3, 10));

        let expected_boxes = vec![(3, 6), (3, 8), (4, 6)].into_iter().collect();
        assert_eq!(double_grid.boxes, expected_boxes)
    }
}
