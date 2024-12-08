---
day: 7
tags: [ post ]
header: 'Day 7: Bridge Repair'
---

Today was finding ways to combine numbers with a set of operators to see if they could produce a certain answer.
This was another puzzle where previous advent of code experience has given me some insight into what might be a good
solution. In this case it feels like it is a good fit depth first search / Dijkstra's algorithm. As such the basic
framework of the solution is lifted from [AoC 2022 - day 12](
https://github.com/kamioftea/advent-of-code-2022/blob/main/src/day_12.rs), then tweaked to fit the value data structure
for this puzzle.

## Parsing the input

The equation type needs to capture current progress as each operator will need to be applied in turn, so as well as
the target and list of numbers I capture the running total.

```rust
#[derive(Eq, PartialEq, Debug, Clone)]
struct Equation {
    target: i64,
    total: i64,
    remaining_numbers: Vec<i64>,
}
```

The operations start being applied after the first number in the list, so I can take the first number and make that
the starting running total.

```rust
fn parse_calibration(line: &str) -> Equation {
    let (target, number_list) = line.split_once(": ").unwrap();
    let mut numbers = number_list.split(" ").flat_map(|num| num.parse());
    
    Equation::new(
        target.parse().unwrap(),
        numbers.next().unwrap(),
        numbers.collect(),
    )
}

fn parse_input(input: &String) -> Vec<Equation> {
    input.lines().map(parse_calibration).collect()
}

fn example_equations() -> Vec<Equation> {
    vec![
        Equation::new(190, 10, vec![19]),
        Equation::new(3267, 81, vec![40, 27]),
        Equation::new(83, 17, vec![5]),
        Equation::new(156, 15, vec![6]),
        Equation::new(7290, 6, vec![8, 6, 15]),
        Equation::new(161011, 16, vec![10, 13]),
        Equation::new(192, 17, vec![8, 14]),
        Equation::new(21037, 9, vec![7, 18, 13]),
        Equation::new(292, 11, vec![6, 16, 20]),
    ]
}

#[test]
fn can_parse_input() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
        .to_string();
    
    assert_eq!(parse_input(&input), example_equations());
}
```

## Part 1 - Order of operations

To implement Dijkstra, Equation needs to have a defined ordering, i.e. it needs to impl `Ord` (and therefore
`PartialOrd`). Rust's built-in binary heap is a max-heap, so the one considered closest to a solution should have
the greatest ordering.

```rust
impl Ord for Equation {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_remaining = self.remaining_numbers.len();
        let other_remaining = other.remaining_numbers.len();
        
        other_remaining
            .cmp(&self_remaining)
            .then_with(|| (other.target - other.total).cmp(&(self.target - self.total)))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Equation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn can_order_equations() {
    assert_eq!(
        Equation::new(190, 29, vec![]).cmp(&Equation::new(190, 19, vec![10])),
        Ordering::Greater
    );
    assert_eq!(
        Equation::new(190, 190, vec![]).cmp(&Equation::new(190, 29, vec![])),
        Ordering::Greater
    );
    
    let sorted: Vec<Equation> = example_equations().into_iter().sorted().collect();
    
    assert_eq!(
        sorted,
        vec![
            Equation::new(21037, 9, vec![7, 18, 13]),
            Equation::new(7290, 6, vec![8, 6, 15]),
            Equation::new(292, 11, vec![6, 16, 20]),
            Equation::new(161011, 16, vec![10, 13]),
            Equation::new(3267, 81, vec![40, 27]),
            Equation::new(192, 17, vec![8, 14]),
            Equation::new(190, 10, vec![19]),
            Equation::new(156, 15, vec![6]),
            Equation::new(83, 17, vec![5]),
        ]
    )
}
```

Next I need to be able to step the equation by one operation. One useful feature of the operators provided is that
with the terms of the equation all being positive, the total can only increase as addition or multiplication are
applied. This means I can ignore any paths that go over the equation's target. So I need a way to pass different
binary operations, that only return the next step if it could be valid.

```rust
type Operation = fn(i64, i64) -> Option<i64>;

impl Equation {
    fn apply(&self, operation: Operation) -> Option<Equation> {
        let mut remaining = self.remaining_numbers.iter();
        remaining
            .next()
            .and_then(|&next| operation(self.total, next))
            .filter(|&total| total <= self.target)
            .map(|sum| Equation::new(
                self.target,
                sum,
                remaining.cloned().collect()
            ))
    }
}

#[test]
fn can_apply_add() {
    let add: Operation = |acc, next| acc.checked_add(next);
    
    assert_eq!(
        Equation::new(190, 10, vec![19]).apply(add),
        Some(Equation::new(190, 29, vec![]))
    );
    assert_eq!(Equation::new(190, 190, vec![19]).apply(add), None);
    assert_eq!(Equation::new(190, 29, vec![]).apply(add), None);
}

#[test]
fn can_apply_mul() {
    let mul: Operation = |acc, next| acc.checked_mul(next);
    
    assert_eq!(
        Equation::new(190, 10, vec![19]).apply(mul),
        Some(Equation::new(190, 190, vec![]))
    );
    assert_eq!(Equation::new(190, 190, vec![]).apply(mul), None);
    assert_eq!(Equation::new(190, 10, vec![20]).apply(mul), None);
}
```

The next piece is the implementation of the depth-first search itself. This is formulaic implementation of
Dijkstra's algorithm.

* Push an initial value onto a `BinaryHeap`
* Take the next item of the heap in a loop
* If that item is a solved equation, return true: the equation is solvable
* Otherwise, push the possible next steps onto the heap
* If the heap runs out, the equation is not solvable, return false

```rust
fn is_solvable(equation: &Equation) -> bool {
    let ops: Vec<Operation> = vec![
        |acc, next| acc.checked_add(next),
        |acc, next| acc.checked_mul(next)
    ];
    
    let mut heap: BinaryHeap<Equation> = BinaryHeap::new();
    heap.push(equation.clone());
    
    while let Some(curr) = heap.pop() {
        if curr.target == curr.total && curr.remaining_numbers.is_empty() {
            return true;
        }
        
        ops.iter()
           .flat_map(|&op| curr.apply(op))
           .for_each(|eq| heap.push(eq))
    }
    
    false
}

#[test]
fn can_check_equation() {
    let equations = example_equations();
    let examples = equations.iter().zip(vec![
        true, true, false, false, false, false, false, false, true,
    ]);
    
    for (equation, expected) in examples {
        assert_eq!(is_solvable(equation), expected)
    }
}
```

All that is left is to run that for each of the equations and sum the target of those that can be solved.

```rust
fn calculate_calibration_total(equations: &Vec<Equation>) -> i64 {
    equations
        .into_iter()
        .filter(|&eq| is_solvable(eq))
        .map(|eq| eq.target)
        .sum()
}

#[test]
fn can_calculate_calibration_total() {
    assert_eq!(
        calculate_calibration_total(&example_equations()),
        3749
    )
}
```

## Part 2 - Combining concatenation

The second part is adding a third operator `||` which concatenates the two numbers as strings. The good news here is
that the invariant that once the target is exceeded, that equation is not possibly valid, still holds. I need to add
in a third operation to the list, but otherwise the same solution will solve the problem.

First try out the concatenation operator

```rust
#[test]
fn can_apply_concat() {
    let concat: Operation = |acc, next| format!("{acc}{next}").parse().ok();
    
    assert_eq!(
        Equation::new(1090, 10, vec![19]).apply(concat),
        Some(Equation::new(1090, 1019, vec![]))
    );
    assert_eq!(Equation::new(190, 190, vec![]).apply(concat), None);
    assert_eq!(Equation::new(190, 10, vec![19]).apply(concat), None);
}
```

Then make `is_solveable` and `calculate_calibration_total` take the list of operations as an argument, and test that
the example equations return as expected.

```rust
//noinspection RsUnnecessaryParentheses Prevent rust_fmt mangling the closures
fn part_1_operations() -> Vec<Operation> {
    vec![
        (|acc, next| acc.checked_add(next)),
        (|acc, next| acc.checked_mul(next)),
    ]
}

//noinspection RsUnnecessaryParentheses Prevent rust_fmt mangling the closures
fn part_2_operations() -> Vec<Operation> {
    vec![
        (|acc, next| acc.checked_add(next)),
        (|acc, next| acc.checked_mul(next)),
        (|acc, next| format!("{acc}{next}").parse().ok()),
    ]
}

#[test]
fn can_check_equation_part_2() {
    let equations = example_equations();
    let examples = equations.iter().zip(vec![
        true, true, false, true, true, false, true, false, true,
    ]);
    let ops = part_2_operations();
    
    for (equation, expected) in examples {
        assert_eq!(
            is_solvable(equation, &ops),
            expected,
            "Expected {equation:?} to be {expected}"
        )
    }
}

#[test]
fn can_calculate_calibration_total() {
    assert_eq!(
        calculate_calibration_total(&example_equations(), &part_1_operations()),
        3749
    );
    
    assert_eq!(
        calculate_calibration_total(&example_equations(), &part_2_operations()),
        11387
    )
}
```

## Optimisations

The day runs quite well, 5s unoptimised, 0.5s with `cargo run --release`. But after today and yesterday having quite
expensive functions over iterators I decide it's time to have a quick look at the
[`rayon` crate](https://crates.io/crates/rayon) that I recall someone mentioning on one of the puzzle write-ups last
year. I decide to look into what it would take to run the different equations in parallel. I'm pleasantly surprised
that it is as simple as adding `use rayon::prelude::*`, and replacing `.iter()` with `.par_iter`.

```diff
+ use rayon::prelude::*;
  
  // ...

  fn calculate_calibration_total(
      equations: &Vec<Equation>, 
      ops: &Vec<Operation>
  ) -> i64 {
      equations
-         .iter()
+         .par_iter()
          .filter(|&eq| is_solvable(eq, &ops))
          .map(|eq| eq.target)
          .sum()
  }
```

This took it to ~0.1s to run. Going back to day 6 and adding rayon to bit more work, see
[the note added to yesterday's write-up]( ../day_6/#edit-7th-dec-2024---adding-rayon-%2F-parallelism).

## Wrap up

I'm not sure how much using an efficient depth-first search algorithm actually helped. It turned out that I had the
sorting the wrong way round (the corrected version is above). Fixing that resulted in a 2x speed up, which is OK,
but not amazing. Comparatively, parallelising the work was a ~5x speed up (on a 14 core processor).

It was also a day of small mistakes. As well as getting the ordering wrong I got confused with some of the variables
in implementing the depth-first search resulting in an infinite loop. I was very glad to have the tests setup to
help debug and cover that.
