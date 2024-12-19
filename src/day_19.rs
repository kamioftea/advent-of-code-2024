//! This is my solution for [Advent of Code - Day 19: _Linen Layout_](https://adventofcode.com/2024/day/19)
//!
//!

use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use Colour::*;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-19-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 19.
pub fn run() {
    let _contents = fs::read_to_string("res/day-19-input.txt").expect("Failed to read file");
}

#[derive(Eq, PartialEq, Debug)]
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

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, Eq, PartialEq)]
struct TreeNode {
    is_match: bool,
    w: Option<TreeNodeRef>,
    u: Option<TreeNodeRef>,
    b: Option<TreeNodeRef>,
    r: Option<TreeNodeRef>,
    g: Option<TreeNodeRef>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode {
            is_match: false,
            w: None,
            u: None,
            b: None,
            r: None,
            g: None,
        }
    }

    fn insert(&mut self, mut colours: impl Iterator<Item = Colour>) {
        match colours.next() {
            Some(White) => self
                .w
                .get_or_insert_with(|| TreeNode::new().into_ref())
                .borrow_mut()
                .insert(colours),
            Some(Blue) => self
                .u
                .get_or_insert_with(|| TreeNode::new().into_ref())
                .borrow_mut()
                .insert(colours),
            Some(Black) => self
                .b
                .get_or_insert_with(|| TreeNode::new().into_ref())
                .borrow_mut()
                .insert(colours),
            Some(Red) => self
                .r
                .get_or_insert_with(|| TreeNode::new().into_ref())
                .borrow_mut()
                .insert(colours),
            Some(Green) => self
                .g
                .get_or_insert_with(|| TreeNode::new().into_ref())
                .borrow_mut()
                .insert(colours),
            None => self.is_match = true,
        }
    }

    fn into_ref(self) -> TreeNodeRef {
        Rc::new(RefCell::new(self))
    }
}

fn parse_patterns(input: &str) -> TreeNode {
    let mut root = TreeNode::new();

    input
        .split(", ")
        .for_each(|pattern| root.insert(pattern.chars().map(|c| c.into())));

    root
}

fn parse_designs(input: &str) -> Vec<Vec<Colour>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect()
}

fn parse_input(input: &String) -> (TreeNode, Vec<Vec<Colour>>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    (parse_patterns(patterns), parse_designs(designs))
}

#[cfg(test)]
mod tests {
    use crate::day_19::*;

    fn example_pattern_tree() -> TreeNode {
        let mut root = TreeNode::new();

        let mut w = TreeNode::new();
        let mut b = TreeNode::new();
        let mut r = TreeNode::new();
        let mut g = TreeNode::new();

        // r
        r.is_match = true;
        // wr
        let mut wr = TreeNode::new();
        wr.is_match = true;
        w.r = Some(wr.into_ref());
        // b
        b.is_match = true;
        // g
        g.is_match = true;
        // bwu
        let mut bw = TreeNode::new();
        let mut bwu = TreeNode::new();
        bwu.is_match = true;
        bw.u = Some(bwu.into_ref());
        b.w = Some(bw.into_ref());
        // rb
        let mut rb = TreeNode::new();
        rb.is_match = true;
        r.b = Some(rb.into_ref());
        // gb
        let mut gb = TreeNode::new();
        gb.is_match = true;
        g.b = Some(gb.into_ref());
        // br
        let mut br = TreeNode::new();
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
bbrgwb"
            .to_string();

        let (patterns, designs) = parse_input(&input);
        assert_eq!(patterns, example_pattern_tree());
        assert_eq!(designs, example_designs());
    }
}
