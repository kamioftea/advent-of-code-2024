---
day: 1
tags: [ post ]
header: 'Day 1: Historian Hysteria'
---

Most of today was hastily copying my project setup from 2023 and making sure everything built OK. I feel I mostly
delegated the hard parts of the day to library functions, but that's fine for day 1.

## Part 1

Parsing the input was done by building two lists of numbers line by line. I had thought about using a BinaryHeap to
sort them as I went, but the numbers can only be pulled out of the heap once, so I decided it didn't offer anything over
building the list and sorting it afterwards.

```rust
fn parse_input(input: &String) -> (Vec<u32>, Vec<u32>) {
    let mut left = vec![];
    let mut right = vec![];
    
    input
        .lines()
        .flat_map(|line| line.split_once("   "))
        .for_each(|(l, r)| {
            left.push(l.parse::<u32>().unwrap());
            right.push(r.parse::<u32>().unwrap());
        });
    
    (left, right)
}

fn sample_input() -> String {
    "3   4
4   3
2   5
1   3
3   9
3   3"
        .to_string()
}

#[test]
fn can_parse_input() {
    assert_eq!(
        parse_input(&sample_input()),
        (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
    );
}
```

Once I had the lists, Itertools's `sorted` and standard library `zip` handled getting the ascending pairs of ids

```rust
fn to_sorted_pairs(left: &Vec<u32>, right: &Vec<u32>) -> Vec<(u32, u32)> {
    left.iter()
        .sorted()
        .zip(right.iter().sorted())
        .map(|(&l, &r)| (l, r))
        .collect()
}

#[test]
fn can_generate_pairs() {
    assert_eq!(
        to_sorted_pairs(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]),
        vec!((1, 3), (2, 3), (3, 3), (3, 4), (3, 5), (4, 9))
    );
}
```

And then those could be reduced to the sum of their absolute distance to provide the puzzle solution

```rust
fn sum_diffs(pairs: &Vec<(u32, u32)>) -> u32 {
    pairs.iter().map(|&(l, r)| l.abs_diff(r)).sum()
}

fn can_sum_diff() {
    assert_eq!(
        sum_diffs(&vec!((1, 3), (2, 3), (3, 3), (3, 4), (3, 5), (4, 9))),
        11
    );
}
```

## Part 2

Itertools's `counts` implements getting the lookup of id -> number of times it appears in the list. That lookup can then
be used to map the list of ids to scores, and sum them to get the puzzle solution.

```rust
fn sum_similarity_scores(left: &Vec<u32>, right: &Vec<u32>) -> usize {
    let lookup = right.iter().counts();
    left.iter()
        .map(|&id| (id as usize) * lookup.get(&id).unwrap_or(&0usize))
        .sum()
}

#[test]
fn can_sum_similarity_scores() {
    assert_eq!(
        sum_similarity_scores(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]),
        31
    )
}
```
