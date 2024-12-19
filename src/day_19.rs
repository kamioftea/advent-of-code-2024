//! This is my solution for [Advent of Code - Day 19: _Linen Layout_](https://adventofcode.com/2024/day/19)
//!
//! [`parse_input`] uses [`parse_patterns`] to turn the patterns into a tree of [`PatternTreeNodes`] by repeatedly
//! using [`PatternTreeNode::insert`], and the designs as a list of lists of [`Colour`].
//!
//! [`PatternTreeNode::count_matches`] solves part one, calling [`PatternTreeNode::matches`] for each design.
//!
//! [`PatternTreeNode::sum_combinations`] solves part one, calling [`PatternTreeNode::combinations`] for each design.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use Colour::*;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-19-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 19.
pub fn run() {
    let contents = fs::read_to_string("res/day-19-input.txt").expect("Failed to read file");
    let (pattern_tree, designs) = parse_input(&contents);

    println!(
        "{} of the designs can be made",
        pattern_tree.count_matches(&designs)
    );

    println!(
        "{} combinations of towels can be made into the designs",
        pattern_tree.sum_combinations(&designs)
    );
}

/// An enum for the possible towel colours
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Colour {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl From<char> for Colour {
    fn from(value: char) -> Self {
        match value {
            'w' => White,
            'u' => Blue,
            'b' => Black,
            'r' => Red,
            'g' => Green,
            _ => unreachable!(),
        }
    }
}

/// The reference used by a node to refer to its children, and to hold a ref back to the root node in the recursive
/// matchers.
type PatternTreeNodeRef = Rc<RefCell<PatternTreeNode>>;

/// A tree with branching factor of 5 for encoding the Set of all the possible patterns
#[derive(Debug, Eq, PartialEq, Clone)]
struct PatternTreeNode {
    is_match: bool,
    w: Option<PatternTreeNodeRef>,
    u: Option<PatternTreeNodeRef>,
    b: Option<PatternTreeNodeRef>,
    r: Option<PatternTreeNodeRef>,
    g: Option<PatternTreeNodeRef>,
}

impl PatternTreeNode {
    /// Create an empty node
    fn new() -> Self {
        PatternTreeNode {
            is_match: false,
            w: None,
            u: None,
            b: None,
            r: None,
            g: None,
        }
    }

    /// helper for getting a reference to a node
    fn into_ref(self) -> PatternTreeNodeRef {
        Rc::new(RefCell::new(self))
    }

    /// Helper to map a given colour to its child node if that exists
    fn get_node(&self, colour: &Colour) -> Option<PatternTreeNodeRef> {
        match colour {
            White => self.w.clone(),
            Blue => self.u.clone(),
            Black => self.b.clone(),
            Red => self.r.clone(),
            Green => self.g.clone(),
        }
    }

    /// Get a reference to the node for a colour, creating it if it doesn't exist
    fn upsert_node(&mut self, colour: &Colour) -> PatternTreeNodeRef {
        (match colour {
            White => &mut self.w,
            Blue => &mut self.u,
            Black => &mut self.b,
            Red => &mut self.r,
            Green => &mut self.g,
        })
        .get_or_insert_with(|| PatternTreeNode::new().into_ref())
        .clone()
    }

    /// Recursively insert a pattern into the tree, creating the required nodes and marking the final node as
    /// terminating a pattern
    fn insert(&mut self, mut colours: impl Iterator<Item = Colour>) {
        match colours.next() {
            Some(colour) => self.upsert_node(&colour).borrow_mut().insert(colours),
            None => self.is_match = true,
        }
    }

    /// Does this tree match the design? he inner recursive function walks the tree matching the characters in the
    /// design, jumping back to the root node when patterns are matched
    fn matches(&self, design: &Vec<Colour>) -> bool {
        fn matches_impl(
            node_ref: PatternTreeNodeRef,
            design: &Vec<Colour>,
            start: usize,
            root: &PatternTreeNodeRef,
        ) -> bool {
            let node = node_ref.borrow();

            if node.is_match && matches_impl(root.clone(), design, start, root) {
                return true;
            }

            if start >= design.len() {
                return &node_ref == root;
            }

            design
                .get(start)
                .and_then(|colour| node.get_node(colour))
                .is_some_and(|next_node_ref| matches_impl(next_node_ref, design, start + 1, root))
        }

        let root_ref = self.clone().into_ref();

        matches_impl(root_ref.clone(), design, 0, &root_ref)
    }

    /// Solves part 1 by counting the designs that the pattern tree can match
    fn count_matches(&self, designs: &Vec<Vec<Colour>>) -> usize {
        designs
            .iter()
            .filter(|&design| self.matches(design))
            .count()
    }

    /// Similar to [`Self::matches`], but doesn't bail early when the root node matches the rest of the pattern,
    /// instead increments a count. Caches combinations that start at the root node for performance.
    fn combinations(&self, design: &Vec<Colour>) -> usize {
        fn combinations_impl(
            node_ref: PatternTreeNodeRef,
            design: &Vec<Colour>,
            start: usize,
            root: &PatternTreeNodeRef,
            cache: &mut HashMap<usize, usize>,
        ) -> usize {
            let node = node_ref.borrow();
            let mut count = 0;

            if node.is_match {
                if let Some(sub_count) = cache.get(&start) {
                    count += sub_count;
                } else {
                    let sub_count = combinations_impl(root.clone(), design, start, root, cache);
                    cache.insert(start, sub_count);

                    count += sub_count;
                }
            } else if start >= design.len() {
                return if &node_ref == root { 1 } else { 0 };
            }

            count += design
                .get(start)
                .and_then(|colour| node.get_node(colour))
                .map(|next_node_ref| {
                    combinations_impl(next_node_ref, design, start + 1, root, cache)
                })
                .unwrap_or(0);

            count
        }

        let root_ref = self.clone().into_ref();
        let mut cache = HashMap::new();

        combinations_impl(root_ref.clone(), design, 0, &root_ref, &mut cache)
    }

    /// Solves part, by calling [`Self::combinations`] for all designs and summing the result,
    fn sum_combinations(&self, designs: &Vec<Vec<Colour>>) -> usize {
        designs.iter().map(|design| self.combinations(design)).sum()
    }
}

/// Turn the list of patterns into a tree that matches them. expected format e.g. `r, wr, b, g, bwu, rb, gb, br`
fn parse_patterns(input: &str) -> PatternTreeNode {
    let mut root = PatternTreeNode::new();

    input
        .split(", ")
        .for_each(|pattern| root.insert(pattern.chars().map(|c| c.into())));

    root
}

/// Turn the list of designs to match into the internal representation, one design per line.
fn parse_designs(input: &str) -> Vec<Vec<Colour>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect()
}

/// Split the input file into patterns and design on a blank line, and hand each to their parsing function
fn parse_input(input: &String) -> (PatternTreeNode, Vec<Vec<Colour>>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    (parse_patterns(patterns), parse_designs(designs))
}

#[cfg(test)]
mod tests {
    use crate::day_19::*;

    fn example_pattern_tree() -> PatternTreeNode {
        let mut root = PatternTreeNode::new();

        let mut w = PatternTreeNode::new();
        let mut b = PatternTreeNode::new();
        let mut r = PatternTreeNode::new();
        let mut g = PatternTreeNode::new();

        // r
        r.is_match = true;
        // wr
        let mut wr = PatternTreeNode::new();
        wr.is_match = true;
        w.r = Some(wr.into_ref());
        // b
        b.is_match = true;
        // g
        g.is_match = true;
        // bwu
        let mut bw = PatternTreeNode::new();
        let mut bwu = PatternTreeNode::new();
        bwu.is_match = true;
        bw.u = Some(bwu.into_ref());
        b.w = Some(bw.into_ref());
        // rb
        let mut rb = PatternTreeNode::new();
        rb.is_match = true;
        r.b = Some(rb.into_ref());
        // gb
        let mut gb = PatternTreeNode::new();
        gb.is_match = true;
        g.b = Some(gb.into_ref());
        // br
        let mut br = PatternTreeNode::new();
        br.is_match = true;
        b.r = Some(br.into_ref());

        root.w = Some(w.into_ref());
        root.b = Some(b.into_ref());
        root.r = Some(r.into_ref());
        root.g = Some(g.into_ref());

        root
    }

    fn example_designs() -> Vec<Vec<Colour>> {
        vec![
            vec![Black, Red, White, Red, Red],
            vec![Black, Green, Green, Red],
            vec![Green, Black, Black, Red],
            vec![Red, Red, Black, Green, Black, Red],
            vec![Blue, Black, White, Blue],
            vec![Black, White, Blue, Red, Red, Green],
            vec![Black, Red, Green, Red],
            vec![Black, Black, Red, Green, White, Black],
        ]
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_parse_input() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"
        .to_string();

        let (patterns, designs) = parse_input(&input);
        assert_eq!(patterns, example_pattern_tree());
        assert_eq!(designs, example_designs());
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_match_pattern() {
        let root = example_pattern_tree();
        // brwrr can be made with a br towel, then a wr towel, and then finally an r towel.
        assert_eq!(root.matches(&vec![Black, Red, White, Red, Red]), true);
        // bggr can be made with a b towel, two g towels, and then an r towel.
        assert_eq!(root.matches(&vec![Black, Green, Green, Red]), true);
        // gbbr can be made with a gb towel and then a br towel.
        assert_eq!(root.matches(&vec![Green, Black, Black, Red]), true);
        // rrbgbr can be made with r, rb, g, and br.
        assert_eq!(
            root.matches(&vec![Red, Red, Black, Green, Black, Red]),
            true
        );
        // ubwu is impossible.
        assert_eq!(root.matches(&vec![Blue, Black, White, Blue]), false);
        // bwurrg can be made with bwu, r, r, and g.
        assert_eq!(
            root.matches(&vec![Black, White, Blue, Red, Red, Green]),
            true
        );
        // brgr can be made with br, g, and r.
        assert_eq!(root.matches(&vec![Black, Red, Green, Red]), true);
        // bbrgwb is impossible.
        assert_eq!(
            root.matches(&vec![Black, Black, Red, Green, White, Black]),
            false
        );
    }

    #[test]
    fn can_count_matches() {
        assert_eq!(example_pattern_tree().count_matches(&example_designs()), 6)
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_count_combinations() {
        let root = example_pattern_tree();
        // brwrr can be made with a br towel, then a wr towel, and then finally an r towel.
        assert_eq!(root.combinations(&vec![Black, Red, White, Red, Red]), 2);
        // bggr can be made with a b towel, two g towels, and then an r towel.
        assert_eq!(root.combinations(&vec![Black, Green, Green, Red]), 1);
        // gbbr can be made with a gb towel and then a br towel.
        assert_eq!(root.combinations(&vec![Green, Black, Black, Red]), 4);
        // rrbgbr can be made with r, rb, g, and br.
        assert_eq!(
            root.combinations(&vec![Red, Red, Black, Green, Black, Red]),
            6
        );
        // ubwu is impossible.
        assert_eq!(root.combinations(&vec![Blue, Black, White, Blue]), 0);
        // bwurrg can be made with bwu, r, r, and g.
        assert_eq!(
            root.combinations(&vec![Black, White, Blue, Red, Red, Green]),
            1
        );
        // brgr can be made with br, g, and r.
        assert_eq!(root.combinations(&vec![Black, Red, Green, Red]), 2);
        // bbrgwb is impossible.
        assert_eq!(
            root.combinations(&vec![Black, Black, Red, Green, White, Black]),
            0
        );
    }

    #[test]
    fn can_sum_combinations() {
        assert_eq!(
            example_pattern_tree().sum_combinations(&example_designs()),
            16
        )
    }
}
