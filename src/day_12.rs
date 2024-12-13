//! This is my solution for [Advent of Code - Day 12: _Garden Groups_](https://adventofcode.com/2024/day/12)
//!
//! [`parse_input`] turns the input file it a [`Garden`] as a `Vec<Vec<char>>`.
//!
//! [`Garden::find_regions`] splits the Garden into [`Region`]s. [`Garden::total_fencing_cost`] solves part 1 using
//! the data collected when finding the regions. [`Garden::total_fencing_cost_with_discount`] solves part 2, using
//! [`Region::count_edges`] to find the unique edges in a region by counting corners in the perimeter.

use itertools::Itertools;
use std::collections::HashSet;
use std::{fs, usize};

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
    fn apply_to(&self, (r, c): &Plot) -> Option<Plot> {
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
    /// Given a facing parallel to the current edge, headed clockwise, the plot forwards and Side::left will be filled if
    /// the edge turns round a concave corner.
    fn convex_delta(&self) -> Delta {
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
    fn straight_ahead_delta(&self) -> Delta {
        self.turn_clockwise().cross_outwards_delta()
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
    perimeter: HashSet<(Plot, Side)>,
}

impl Region {
    /// Initialise an empty region
    fn new(crop: char) -> Region {
        Region {
            crop,
            plots: HashSet::new(),
            perimeter: HashSet::new(),
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

    /// Given an edge on one side of a plot. Calculate if following that edge clockwise is a corner
    ///
    /// ```text
    ///     +---+
    ///     | A |
    /// +---+ - +
    /// | B | C |
    /// +---+---+
    /// ```
    ///
    /// There are only three cases:
    /// - Straight - no corner: The examples above are
    ///     - Following the Side::right of the shape from `A` to `C`.
    ///     - Following the Side::bottom of the shape from `C` to `B`.
    /// - Convex corner, e.g. Side::top B going to A
    /// - Concave corner which follow the Side::left and Side::top of A, Side::bottom and Side::left of B, and the Side::right of C.
    ///
    /// If the block straight-ahead is not set it's a concave corner. If it is set and the one ahead and to the Side::left
    /// is also set it's concave.
    fn check_for_corner(&self, plot: &Plot, side: &Side) -> bool {
        let next_convex = side.convex_delta().apply_to(&plot);
        let next_straight = side.straight_ahead_delta().apply_to(&plot);

        !self.contains(&next_straight) || self.contains(&next_convex)
    }

    /// For all the edges in the perimeter, check if they are followed by a corner
    fn count_edges(&self) -> usize {
        self.perimeter
            .iter()
            .filter(|(plot, side)| self.check_for_corner(plot, &side))
            .count()
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

    /// Do a modified bucket fill to determine the plots that make up the region that includes the starting plot. When
    /// an edge is encountered store the side of the plot it is on for later corner detection.
    fn walk_region(&self, start: Plot) -> Region {
        fn walk_region_iter(garden: &Garden, plot: Plot, region: &mut Region) {
            let crop = garden.get(plot).unwrap();

            if !region.plots.insert(plot) {
                // already visited
                return;
            }

            for side in [Side::TOP, Side::RIGHT, Side::BOTTOM, Side::LEFT] {
                match side
                    .cross_outwards_delta()
                    .apply_to(&plot)
                    .and_then(|next_plot| Some(next_plot).zip(garden.get(next_plot)))
                {
                    Some((next_plot, next_crop)) if next_crop == crop => {
                        walk_region_iter(garden, next_plot, region)
                    }
                    _ => {
                        region.perimeter.insert((plot, side));
                    }
                }
            }
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
            .map(|region| region.plots.len() * region.perimeter.len())
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

/// Parse a text grid into a [`Garden`]
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

    fn region_a() -> Region {
        Region {
            crop: 'A',
            plots: vec![(0, 0), (0, 1), (0, 2), (0, 3)].into_iter().collect(),
            perimeter: vec![
                ((0, 0), Side::TOP),
                ((0, 0), Side::BOTTOM),
                ((0, 0), Side::LEFT),
                ((0, 1), Side::TOP),
                ((0, 1), Side::BOTTOM),
                ((0, 2), Side::TOP),
                ((0, 2), Side::BOTTOM),
                ((0, 3), Side::TOP),
                ((0, 3), Side::RIGHT),
                ((0, 3), Side::BOTTOM),
            ]
            .into_iter()
            .collect(),
        }
    }

    fn region_b() -> Region {
        Region {
            crop: 'B',
            plots: vec![(1, 0), (1, 1), (2, 0), (2, 1)].into_iter().collect(),
            perimeter: vec![
                ((1, 0), Side::TOP),
                ((1, 0), Side::LEFT),
                ((1, 1), Side::TOP),
                ((1, 1), Side::RIGHT),
                ((2, 0), Side::BOTTOM),
                ((2, 0), Side::LEFT),
                ((2, 1), Side::RIGHT),
                ((2, 1), Side::BOTTOM),
            ]
            .into_iter()
            .collect(),
        }
    }

    fn region_c() -> Region {
        Region {
            crop: 'C',
            plots: vec![(1, 2), (2, 2), (2, 3), (3, 3)].into_iter().collect(),
            perimeter: vec![
                ((1, 2), Side::RIGHT),
                ((1, 2), Side::LEFT),
                ((2, 2), Side::BOTTOM),
                ((2, 3), Side::RIGHT),
                ((3, 3), Side::BOTTOM),
                ((2, 3), Side::TOP),
                ((1, 2), Side::TOP),
                ((2, 2), Side::LEFT),
                ((3, 3), Side::RIGHT),
                ((3, 3), Side::LEFT),
            ]
            .into_iter()
            .collect(),
        }
    }

    fn region_d() -> Region {
        Region {
            crop: 'D',
            plots: vec![(1, 3)].into_iter().collect(),
            perimeter: vec![
                ((1, 3), Side::TOP),
                ((1, 3), Side::BOTTOM),
                ((1, 3), Side::LEFT),
                ((1, 3), Side::RIGHT),
            ]
            .into_iter()
            .collect(),
        }
    }

    fn region_e() -> Region {
        Region {
            crop: 'E',
            plots: vec![(3, 0), (3, 1), (3, 2)].into_iter().collect(),
            perimeter: vec![
                ((3, 1), Side::TOP),
                ((3, 2), Side::RIGHT),
                ((3, 2), Side::BOTTOM),
                ((3, 0), Side::TOP),
                ((3, 2), Side::TOP),
                ((3, 1), Side::BOTTOM),
                ((3, 0), Side::LEFT),
                ((3, 0), Side::BOTTOM),
            ]
            .into_iter()
            .collect(),
        }
    }

    #[test]
    fn can_find_region() {
        let garden = example_garden();

        assert_eq!(garden.walk_region((0, 0)), region_a());
        assert_eq!(garden.walk_region((1, 0)), region_b());
        assert_eq!(garden.walk_region((1, 2)), region_c());
        assert_eq!(garden.walk_region((1, 3)), region_d());
        assert_eq!(garden.walk_region((3, 0)), region_e());

        assert_contains_in_any_order(
            garden.find_regions(),
            vec![region_a(), region_b(), region_c(), region_d(), region_e()],
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
            perimeter: vec![
                ((0, 0), Side::TOP),
                ((0, 0), Side::RIGHT),
                ((0, 0), Side::BOTTOM),
                ((0, 0), Side::LEFT),
            ]
            .into_iter()
            .collect(),
        };
        assert_eq!(basic.count_edges(), 4);

        assert_eq!(region_a().count_edges(), 4);

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
