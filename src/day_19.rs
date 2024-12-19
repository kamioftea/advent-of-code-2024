//! This is my solution for [Advent of Code - Day 19: _Linen Layout_](https://adventofcode.com/2024/day/19)
//!
//!

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
        count_matches(&pattern_tree, &designs)
    );

    println!(
        "{} combinations of the designs can be made",
        sum_combinations(&pattern_tree, &designs)
    );
}

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

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, Eq, PartialEq, Clone)]
struct TreeNode {
    is_match: bool,
    is_root: bool,
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
            is_root: false,
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

    fn get_node(&self, colour: &Colour) -> Option<TreeNodeRef> {
        match colour {
            White => self.w.clone(),
            Blue => self.u.clone(),
            Black => self.b.clone(),
            Red => self.r.clone(),
            Green => self.g.clone(),
        }
    }

    fn into_ref(self) -> TreeNodeRef {
        Rc::new(RefCell::new(self))
    }
}

fn parse_patterns(input: &str) -> TreeNode {
    let mut root = TreeNode::new();
    root.is_root = true;

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

fn matches_impl(
    node_ref: TreeNodeRef,
    design: &Vec<Colour>,
    start: usize,
    root: &TreeNodeRef,
) -> bool {
    let node = node_ref.borrow();

    if node.is_match && matches_impl(root.clone(), design, start, root) {
        return true;
    }

    if start >= design.len() {
        return node.is_root;
    }

    node.get_node(&design[start])
        .is_some_and(|next_node_ref| matches_impl(next_node_ref, design, start + 1, root))
}

fn matches(root: &TreeNode, design: &Vec<Colour>) -> bool {
    let root_ref = root.clone().into_ref();

    matches_impl(root_ref.clone(), design, 0, &root_ref)
}

fn count_matches(root: &TreeNode, designs: &Vec<Vec<Colour>>) -> usize {
    designs
        .iter()
        .filter(|&design| matches(root, design))
        .count()
}

fn combinations_impl(
    node_ref: TreeNodeRef,
    design: &Vec<Colour>,
    start: usize,
    root: &TreeNodeRef,
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
        return if node.is_root { 1 } else { 0 };
    }

    count += design
        .get(start)
        .and_then(|colour| node.get_node(colour))
        .map(|next_node_ref| combinations_impl(next_node_ref, design, start + 1, root, cache))
        .unwrap_or(0);

    count
}

fn combinations(root: &TreeNode, design: &Vec<Colour>) -> usize {
    let root_ref = root.clone().into_ref();
    let mut cache = HashMap::new();

    combinations_impl(root_ref.clone(), design, 0, &root_ref, &mut cache)
}

fn sum_combinations(root: &TreeNode, designs: &Vec<Vec<Colour>>) -> usize {
    designs
        .iter()
        .map(|design| combinations(root, design))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_19::*;

    fn example_pattern_tree() -> TreeNode {
        let mut root = TreeNode::new();
        root.is_root = true;

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
        assert_eq!(matches(&root, &vec![Black, Red, White, Red, Red]), true);
        // bggr can be made with a b towel, two g towels, and then an r towel.
        assert_eq!(matches(&root, &vec![Black, Green, Green, Red]), true);
        // gbbr can be made with a gb towel and then a br towel.
        assert_eq!(matches(&root, &vec![Green, Black, Black, Red]), true);
        // rrbgbr can be made with r, rb, g, and br.
        assert_eq!(
            matches(&root, &vec![Red, Red, Black, Green, Black, Red]),
            true
        );
        // ubwu is impossible.
        assert_eq!(matches(&root, &vec![Blue, Black, White, Blue]), false);
        // bwurrg can be made with bwu, r, r, and g.
        assert_eq!(
            matches(&root, &vec![Black, White, Blue, Red, Red, Green]),
            true
        );
        // brgr can be made with br, g, and r.
        assert_eq!(matches(&root, &vec![Black, Red, Green, Red]), true);
        // bbrgwb is impossible.
        assert_eq!(
            matches(&root, &vec![Black, Black, Red, Green, White, Black]),
            false
        );
    }

    #[test]
    fn can_count_matches() {
        assert_eq!(
            count_matches(&example_pattern_tree(), &example_designs()),
            6
        )
    }

    #[test]
    fn can_count_combinations() {
        let root = example_pattern_tree();
        // brwrr can be made with a br towel, then a wr towel, and then finally an r towel.
        assert_eq!(combinations(&root, &vec![Black, Red, White, Red, Red]), 2);
        // bggr can be made with a b towel, two g towels, and then an r towel.
        assert_eq!(combinations(&root, &vec![Black, Green, Green, Red]), 1);
        // gbbr can be made with a gb towel and then a br towel.
        assert_eq!(combinations(&root, &vec![Green, Black, Black, Red]), 4);
        // rrbgbr can be made with r, rb, g, and br.
        assert_eq!(
            combinations(&root, &vec![Red, Red, Black, Green, Black, Red]),
            6
        );
        // ubwu is impossible.
        assert_eq!(combinations(&root, &vec![Blue, Black, White, Blue]), 0);
        // bwurrg can be made with bwu, r, r, and g.
        assert_eq!(
            combinations(&root, &vec![Black, White, Blue, Red, Red, Green]),
            1
        );
        // brgr can be made with br, g, and r.
        assert_eq!(combinations(&root, &vec![Black, Red, Green, Red]), 2);
        // bbrgwb is impossible.
        assert_eq!(
            combinations(&root, &vec![Black, Black, Red, Green, White, Black]),
            0
        );
    }

    #[test]
    fn can_sum_combinations() {
        assert_eq!(
            sum_combinations(&example_pattern_tree(), &example_designs()),
            16
        )
    }
}
