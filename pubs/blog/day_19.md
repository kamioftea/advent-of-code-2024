---
day: 19
tags: [ post ]
header: 'Day 19: Linen Layout'
---

Today is combining Magic: The Gathering patterned towels to make larger designs. I've decided to implement the
patterns as a tree, which involves some bits of rust I'm not all that familiar with `Rc` and `RefCell`.

## Parsing the input

So first of all, to represent the patterns as a tree I need a type to represent a node in that tree. The plan is to
have each node up to 5 branches, one for each of the colours, and for each to have a flag to denote if it terminates
one of the available patterns. So the sample input is represented by the following tree:

```text
Input: r, wr, b, g, bwu, rb, gb, br

                             +-------------+
      +----------------------| root: false |----------------------+
      |                      +-------------+                      |
      |                       /              \                    |
+-----------+          +===========+         +===========+  +===========+
| W: false  |          || B: true ||         || R: true ||  || G: true ||
+-----------+          +===========+         +===========+  +===========+
      |                 /         \                |              |
+===========+  +-----------+  +===========+  +===========+  +===========+
|| R: true ||  | W: false  |  || R: true ||  || B: true ||  || G: true ||
+===========+  +-----------+  +===========+  +===========+  +===========+
                     |
               +===========+
               || U: true ||
               +===========+
```

The references to the child nodes need to allowed shared access, so I need to map them in `Rc`, and to allow
mutating those references the nodes need to be further wrapped in a `RefCell`.

```rust
type PatternTreeNodeRef = Rc<RefCell<PatternTreeNode>>;

#[derive(Debug, Eq, PartialEq, Clone)]
struct PatternTreeNode {
    is_match: bool,
    w: Option<PatternTreeNodeRef>,
    u: Option<PatternTreeNodeRef>,
    b: Option<PatternTreeNodeRef>,
    r: Option<PatternTreeNodeRef>,
    g: Option<PatternTreeNodeRef>,
}
```

I will end up matching on the colours to map them to the right branch, and I don't want to be littering the code
with `unreachable!()` to make those matches exhaustive, so I'll create an enum for the colours.

```rust
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
```

I now need to be able to build up the tree of patterns. To do this I need to be able to insert a new pattern into
the root node. Being a tree this is best done recursively

- Take the next colour in the pattern
- If there is one: Get the node for that colour, or create an empty one, and insert the rest of the pattern into that
  node
- Otherwise, we're at the end of the pattern, mark the current node as terminating a pattern and return.

Taking the next colour in the pattern as an `Option` can be done by passing in a mutable iterator and calling `next`.

```rust
impl PatternTreeNode {
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
    
    fn into_ref(self) -> PatternTreeNodeRef {
        Rc::new(RefCell::new(self))
    }
    
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
    
    fn insert(&mut self, mut colours: impl Iterator<Item=Colour>) {
        match colours.next() {
            Some(colour) => self.upsert_node(&colour).borrow_mut().insert(colours),
            None => self.is_match = true,
        }
    }
}
```

The remainder of the parsing is

- Split the file into the pattern and design lists
- Insert each pattern into the tree
- Turn the list of designs into a `Vec<Vex<Colour>>`

```rust
fn parse_patterns(input: &str) -> PatternTreeNode {
    let mut root = PatternTreeNode::new();
    
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

fn parse_input(input: &String) -> (PatternTreeNode, Vec<Vec<Colour>>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    
    (parse_patterns(patterns), parse_designs(designs))
}
```

Creating the sample tree is tedious, but helps me make sure I understand how the tree is working.

```rust
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
```

## Part 1 - Grand designs

The tree structure does a bunch of the work for implementing the matcher.

- Starting from the root node and the first character
- If current node is a terminal node, then a pattern has just been matched, try matching the rest of the pattern
  from the root node, if that matches rewind the recursion by returning true.
- If the node for the next character is set, try matching the rest of the pattern from there.
- If it is not set, then there is not a match.

I use the pattern of exposing a matches method, but having an inner function that does the recursion to make the
API simpler. In the recursive function I also need to keep track of where I am in the design, and keep a reference
to the root node for ease of jumping back.

```rust
impl PatternTreeNode {
    fn get_node(&self, colour: &Colour) -> Option<PatternTreeNodeRef> {
        match colour {
            White => self.w.clone(),
            Blue => self.u.clone(),
            Black => self.b.clone(),
            Red => self.r.clone(),
            Green => self.g.clone(),
        }
    }
    
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
                .is_some_and(|next_node_ref|
                    matches_impl(next_node_ref, design, start + 1, root)
                )
        }
        
        let root_ref = self.clone().into_ref();
        
        matches_impl(root_ref.clone(), design, 0, &root_ref)
    }
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
```

The puzzle solution is running that for each design and counting the matches.

```rust
impl PatternTreeNode {
    fn count_matches(&self, designs: &Vec<Vec<Colour>>) -> usize {
        designs
            .iter()
            .filter(|&design| self.matches(design))
            .count()
    }
}

#[test]
fn can_count_matches() {
    assert_eq!(example_pattern_tree().count_matches(&example_designs()), 6)
}
```

## Part 2 - Going infinite

The challenge now is to find all the ways that each design can be matched. I'd like to be able to repeat my plan for
[day 11 - part](../day_11/#part-2---an-exponential-stone-gathers-no-moss) and use `#[cached]` to handle memoisation.
Unfortunately it needs the arguments it's caching to implement `Copy` and I don't think it's worth moving the tree
over to use `Arc` and `Mutex`. I also only really need to cache when the function matches a pattern and loops back
to the root node.

There are some other minor changes to return a count rather than true on the first match, but it follows the same
logic as part 1, without bailing early if a terminal node is reached and the root node can match the rest.

```rust
impl PatternTreeNode {
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
                    let sub_count = combinations_impl(
                        root.clone(),
                        design,
                        start,
                        root,
                        cache
                    );
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
    
    fn sum_combinations(&self, designs: &Vec<Vec<Colour>>) -> usize {
        designs.iter().map(|design| self.combinations(design)).sum()
    }
}

#[test]
fn can_sum_combinations() {
    assert_eq!(
        example_pattern_tree().sum_combinations(&example_designs()),
        16
    )
}
```

## Wrap up

Today was an achievement for me. I've previously really struggled with building more complex data structures in rust
because satisfying the borrow checker can be a nightmare. I still muddled through today with the borrow checker
constantly complaining, but I was able to use that as guidance and once it compiled, it worked. It's also pretty
quick, 2-3ms to count ~850 trillion combinations.
