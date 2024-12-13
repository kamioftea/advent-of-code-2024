//! This is my solution for [Advent of Code - Day 12: _Garden Groups_](https://adventofcode.com/2024/day/12)
//!
//! [`parse_input`] turns the input file it a [`Garden`] as a `Vec<Vec<char>>`.
//!
//! [`Garden::find_regions`] splits the Garden into [`Region`]s. [`Garden::total_fencing_cost`] solves part 1 using
//! the data collected when finding the regions. [`Garden::total_fencing_cost_with_discount`] solves part 2, using
//! [`Region::count_edges`] to find the unique edges in a region.

use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-12-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 12.
pub fn run() {
    let contents = fs::read_to_string("res/day-12-input.txt").expect("Failed to read file");
    let garden = parse_input(&contents);

    println!("The total fencing cost is {}", garden.total_fencing_cost());
    println!(
        "The total discounted fencing cost is {}",
        garden.total_fencing_cost_with_discount()
    );
}

/// Coordinates of a plot within a [`Garden`]
type Plot = (usize, usize);

/// Implement deltas as a struct to allow some convenient consts and functions to be defined
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Delta(isize, isize);

impl Delta {
    /// Move upwards
    const UP: Delta = Delta(-1, 0);
    /// Move rightwards
    const RIGHT: Delta = Delta(0, 1);
    /// Move downwards
    const DOWN: Delta = Delta(1, 0);
    /// Move leftwards
    const LEFT: Delta = Delta(0, -1);

    /// Combine two deltas
    fn add(&self, other: &Self) -> Self {
        Delta(self.0 + other.0, self.1 + other.1)
    }

    /// Get the coordinates of the plot after applying this delta to the provided plot. This will be None if either
    /// axis becomes negative
    fn apply_to(&self, (r, c): Plot) -> Option<Plot> {
        r.checked_add_signed(self.0)
            .zip(c.checked_add_signed(self.1))
    }
}

/// Use to track which side of the current plot has the edge being followed when walking the perimeter
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Side {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

impl Side {
    /// Given a facing parallel to the current edge, headed clockwise, the plot forwards and left will be filled if
    /// the edge turns round a concave corner.
    fn concave_delta(&self) -> Delta {
        match self {
            Side::TOP => Delta::UP.add(&Delta::RIGHT),
            Side::RIGHT => Delta::RIGHT.add(&Delta::DOWN),
            Side::BOTTOM => Delta::DOWN.add(&Delta::LEFT),
            Side::LEFT => Delta::LEFT.add(&Delta::UP),
        }
    }

    /// Given a cell which potentially has an edge on this side, what is the delta to cross that edge, from inside
    /// the shape to outside
    fn cross_outwards_delta(&self) -> Delta {
        match self {
            Side::TOP => Delta::UP,
            Side::RIGHT => Delta::RIGHT,
            Side::BOTTOM => Delta::DOWN,
            Side::LEFT => Delta::LEFT,
        }
    }

    /// The facing parallel to this side, that walks the inside of that edge clockwise.
    fn follow_clockwise_delta(&self) -> Delta {
        self.turn_clockwise().cross_outwards_delta()
    }

    /// The side counterclockwise of this one
    fn turn_counterclockwise(&self) -> Side {
        match self {
            Side::TOP => Side::LEFT,
            Side::RIGHT => Side::TOP,
            Side::BOTTOM => Side::RIGHT,
            Side::LEFT => Side::BOTTOM,
        }
    }

    /// The side clockwise of this one
    fn turn_clockwise(&self) -> Side {
        match self {
            Side::TOP => Side::RIGHT,
            Side::RIGHT => Side::BOTTOM,
            Side::BOTTOM => Side::LEFT,
            Side::LEFT => Side::TOP,
        }
    }
}

/// A region that is a set of orthogonally adjacent plots in a [`Garden`] with the same crop. It stores the plots and
/// number of units in the perimeter for use by [`Garden::total_fencing_cost`].
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

    /// Helper for checking if plot is in the grid. Takes an `Option` to match [`Delta::apply_to`]
    fn contains(&self, plot: &Option<Plot>) -> bool {
        if let Some(coord) = plot {
            self.plots.iter().contains(coord)
        } else {
            false
        }
    }

    /// Given a plot with a known edge, recursively follow that edge clockwise until it loops. Uses a shared visited
    /// set so that reaching the same edge from a different plot or side doesn't lead to duplicates.
    ///
    /// Given this region:
    /// ```text
    ///     +---+---+
    ///     | A | B |
    /// +---+ - + - +
    /// | E | D | C |
    /// +---+---+---+
    /// ```
    ///
    /// - If I start at A then the cell going clockwise I start facing rightwards. The cell above B is not in the
    /// shape, so
    ///   the edge doesn't turn right. Straight on is B, so I move there keeping the same facing.
    /// - For B, the concave space is still not in the shape, neither is straight on, so stay in B and turn right,
    /// - The same happens, moving into C then turning, incrementing the count again.
    /// - C to D to E are all straight on.
    /// - In E, the two cells are outside, so turn, and then ahead is still blocked, so turn again, incrementing the
    /// count
    ///   on each turn.
    /// - From E, heading rightwards is the first concave corner. A is in the shape, so move to A and turn left.
    /// - Finally, the way ahead is blocked, so turn, and I'm back to a position and facing already seen so that
    /// perimeter is
    ///   done.
    ///
    /// ```text
    /// Count:    0                 0                1                 1                 2
    ///           X                     X
    ///     +---+---+         +---+---+        +---+---+         +---+---+         +---+---+
    ///     | > | O |         |   | > | X      |   | V |         |   |   |         |   |   |
    /// +---+ - + - +     +---+ - + - +    +---+ - + - +     +---+ - + - +     +---+ - + - +
    /// |   |   |   |     |   |   |   |    |   |   | O | X   |   |   | V |     |   | O | < |
    /// +---+---+---+     +---+---+---+    +---+---+---+     +---+---+---+     +---+---+---+
    ///                                                                X   X         X
    ///
    ///
    /// Count:    2                 2                 3                 4                 5
    ///                                                                           X   X
    ///     +---+---+         +---+---+         +---+---+         +---+---+         +---+---+
    ///     |   |   |         |   |   |   X   X |   |   |         | O |   |         | ^ |   |
    /// +---+ - + - +     +---+ - + - +     +---+ - + - +     +---+ - + - +     +---+ - + - +
    /// | O | < |   |   X | < |   |   |     | ^ |   |   |     | > |   |   |     |   |   |   |
    /// +---+---+---+     +---+---+---+     +---+---+---+     +---+---+---+     +---+---+---+
    ///   X             X
    ///
    /// Count:    6
    ///
    ///     +---+---+
    ///     | > |   |
    /// +---+ - + - +
    /// |   |   |   |
    /// +---+---+---+
    /// ```
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

    /// For all the plots in this region, find any edges by checking if adjacent cells are not in the region. If
    /// so call [`Region::walk_perimeter`] to count the edges in the discovered perimeter, using a set to prevent
    /// that perimeter being walked again from other starting points.
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

/// A grid of plots containing regions of different crops
#[derive(Eq, PartialEq, Debug)]
struct Garden {
    plots: Vec<Vec<char>>,
}

impl Garden {
    /// Get the contents of a given plot, None if the coordinates are outside the garden
    fn get(&self, (r, c): Plot) -> Option<char> {
        self.plots.get(r).and_then(|row| row.get(c).copied())
    }

    /// Return each of the four orthogonally adjacent plots that are in the garden
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

    /// Do a modified bucket fill to determine the plots that make up the region that includes the starting plot.
    /// Keeping track of when the bucket fill reaches an edge as that gives the length of the perimeter.
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

    /// Iterate over each plots' coordinates in the garden
    fn iter_plots<'a>(&'a self) -> impl Iterator<Item = Plot> + 'a {
        self.plots
            .iter()
            .enumerate()
            .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, _)| (r, c)))
    }

    /// Return all the distinct crop regions in the garden
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

    /// The total cost to fence all the regions in the garden
    fn total_fencing_cost(&self) -> usize {
        self.find_regions()
            .iter()
            .map(|region| region.plots.len() * region.perimeter)
            .sum()
    }

    /// The total cost to fence all the regions in the garden after applying the "bulk discount"
    fn total_fencing_cost_with_discount(&self) -> usize {
        self.find_regions()
            .iter()
            .map(|region| region.plots.len() * region.count_edges())
            .sum()
    }
}

fn parse_input(input: &String) -> Garden {
    Garden {
        plots: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use crate::day_12::*;
    use crate::helpers::test::assert_contains_in_any_order;
    
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
        )
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

        assert_contains_in_any_order(
            garden.find_regions(),
            vec![region_a, region_b, region_c, region_d, region_e],
        );
    }

    #[test]
    fn can_calculate_costs() {
        assert_eq!(example_garden().total_fencing_cost(), 140);
        assert_eq!(enclave_example().total_fencing_cost(), 772);
        assert_eq!(larger_example().total_fencing_cost(), 1930);
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
}
