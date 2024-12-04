---
day: 1
tags: [ post ]
header: 'Day 1: Historian Hysteria'
---

Most of today was hastily copying my project setup from 2023 and making sure everything built OK. There is more
general tooling around doing AoC in Rust than when I first tried. I've chosen to stick with what I know, as I'm
comfortable with my setup.

Boilerplate out of the way, today's task is to transpose the lines of input into a list of numbers for each column,
and sort each one, and then pair them up lowest to highest and sum the distance between them.

## Parsing the input

The main awkwardness here was transposing the rows/lines of input into two columns. As there were only two columns it
was less effort to manually pushed each column to its own mutable list. I thought about using BinaryHeaps as the
collectors, which would sort them as I went. The numbers can only be pulled out of the heap once, so I decided it
didn't offer enough benefit vs over building the lists as is, and sorting them afterwards.

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

## Part 1 - Find the total distance

Having obtained the lists, I have to sort each list, then pair lowest to lowest, and so on. There are existing
library methods that can do this: Sorting by Itertools's `sorted` and, then pairing up with standard library `zip`.

```rust
fn to_sorted_pairs(left: &Vec<u32>, right: &Vec<u32>) -> Vec<(u32, u32)> {
    let sorted_left = left.iter().cloned().sorted();
    let sorted_right = right.iter().cloned().sorted();
    sorted_left.zip(sorted_right).collect()
}

#[test]
fn can_generate_pairs() {
    assert_eq!(
        to_sorted_pairs(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]),
        vec!((1, 3), (2, 3), (3, 3), (3, 4), (3, 5), (4, 9))
    );
}
```

Once zipped up, each pair could be converted to their absolute difference, and the resulting iterator summed to give
the puzzle solution.

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

## Part 2 - Find the total "similarity score"

Part two used the same columns very differently. The left-hand column is a list of ids to loop through and calculate a
similarity score for each id, which is the id multiplied often it appears in the right-hand column.

Itertools's has `counts`, which implements getting a lookup by id to the number of times it appears in the list.
That lookup can then be used to map the list of ids to their scores, and then sum them to get the puzzle solution.

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
        sum_similarity_scores(
            &vec![3, 4, 2, 1, 3, 3],
            &vec![4, 3, 5, 3, 9, 3]
        ),
        31
    )
}
```

## Wrap up

I feel I mostly delegated the hard parts of the day to library functions, but that's fine for day 1. It will get
harder from here, and gave me time to sort out the setup.
