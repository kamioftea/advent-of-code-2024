---
day: 22
tags: [ post ]
header: 'Day 22: Monkey Market'
---

Today was mostly CPU intensive messing around with numbers. Given a seed, and an algorithm for generating a sequence
of pseudorandom numbers, generate 2000 entries in the sequence for ~2000 seeds.

## Parsing the input

The input doesn't need much parsing today, each line is a seed.

```rust
fn parse_input(input: &String) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[test]
fn can_parse_input() {
    let input = "1
10
100
2024
"
        .to_string();
    assert_eq!(parse_input(&input), vec![1, 10, 100, 2024])
}

```

## Part 1 - Random numbers

The algorithm is formulaic, so part one is writing that out. The bitwise xor `mix`, and modulo `prune` are repeated,
and to avoid lots of nested function calls I add a helper trait to make these methods on `u64`.

```rust
trait NumberExtensions {
    fn mix(&self, prev: &Self) -> Self;
    fn prune(&self) -> Self;
    fn next_secret(&self) -> Self;
}

impl NumberExtensions for u64 {
    fn mix(&self, prev: &Self) -> Self {
        self ^ prev
    }
    
    fn prune(&self) -> Self {
        self % 16777216
    }
    
    fn next_secret(&self) -> Self {
        let step_1 = (self * 64).mix(self).prune();
        let step_2 = (step_1 / 32).mix(&step_1).prune();
        (step_2 * 2048).mix(&step_2).prune()
    }
}

#[test]
fn can_mix_numbers() {
    assert_eq!(42.mix(&15), 37)
}

#[test]
fn can_prune_numbers() {
    assert_eq!(100000000.prune(), 16113920)
}

#[test]
fn can_generate_next_secret() {
    assert_eq!(123.next_secret(), 15887950)
}
```

I can then generate the sequence using `Itertools::generate`.

```rust
fn pseudorandom_sequence(seed: u64) -> impl Iterator<Item=u64> {
    iterate(seed, |prev| prev.next_secret())
}

#[test]
fn can_iterate_secret_number() {
    assert_eq!(
        pseudorandom_sequence(123).dropping(1).take(10).collect::<Vec<u64>>(),
        vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156,
            11100544, 12249484, 7753432, 5908254
        ]
    );
    
    assert_eq!(pseudorandom_sequence(1).nth(2000), Some(8685429));
    assert_eq!(pseudorandom_sequence(10).nth(2000), Some(4700978));
    assert_eq!(pseudorandom_sequence(100).nth(2000), Some(15273692));
    assert_eq!(pseudorandom_sequence(2024).nth(2000), Some(8667524));
}
```

From that I can then iterate each of the seeds and sum the result to get the part 1 solution.

```rust
fn iterate_and_sum(seeds: &Vec<u64>) -> u64 {
    seeds
        .iter()
        .map(|seed| pseudorandom_sequence(*seed).nth(2000).unwrap())
        .sum()
}

#[test]
fn can_iterate_and_sum_list() {
    assert_eq!(iterate_and_sum(&vec![1, 10, 100, 2024]), 37327623);
}
```

## Part 2 - Playing the markets

Part 2 I need to find the sequences of price differences that will get me the most bananas. For this I need to look
at each window of 5 prices, which generates 4 diffs, and track the bananas that would buy me. Once a sequence has
appeared, later copies of that don't matter, so I need to keep track of the seen sequences. If I use the sequence of
diffs as the key for a hashmap, the largest value will then be the number of bananas I can buy. The puzzle solution
only cares about that, so I don't need to do anything further with the sequence.

```rust
fn populate_sequence_scores(
    sequence_scores: &mut HashMap<(i8, i8, i8, i8), u64>,
    seed: u64
) {
    let mut seen = HashSet::new();
    pseudorandom_sequence(seed)
        .take(2000)
        .map(|secret| (secret % 10) as i8)
        .tuple_windows()
        .for_each(|(a, b, c, d, e)| {
            let diff_sequence = (b - a, c - b, d - c, e - d);
            if seen.insert(diff_sequence) {
                *(sequence_scores.entry(diff_sequence).or_default()) += e as u64;
            }
        })
}

fn bananas_from_best_diff_sequence(seeds: &Vec<u64>) -> u64 {
    let mut sequence_scores = HashMap::new();
    for &seed in seeds {
        populate_sequence_scores(&mut sequence_scores, seed);
    }
    
    sequence_scores.values().max().unwrap_or(&0).clone()
}

#[test]
fn can_find_best_sequence() {
    assert_eq!(bananas_from_best_diff_sequence(&vec![1, 2, 3, 2024]), 23)
}
```

## Optimisations

That is working, but it's taking close to 400ms to run. So I try some optimisations. First I make the sequence
generation use a mutable intermediate value, which saves about 100ms.

```diff
  trait NumberExtensions {
-     fn mix(&self, prev: &Self) -> Self;
-     fn prune(&self) -> Self;
+     fn mix(&mut self, prev: &Self) -> ();
+     fn prune(&mut self) -> ();
      fn next_secret(&self) -> Self;
  }

  impl NumberExtensions for u64 {
-     fn mix(&self, prev: &Self) -> Self {
-         self ^ prev
+     fn mix(&mut self, prev: &Self) -> () {
+         *self ^= prev
      }
  
-     fn prune(&self) -> Self {
-         self % 16777216
+     fn prune(&mut self) -> () {
+         *self %= 16777216
      }
  
      fn next_secret(&self) -> Self {
-         let step_1 = (self * 64).mix(self).prune();
-         let step_2 = (step_1 / 32).mix(&step_1).prune();
-         (step_2 * 2048).mix(&step_2).prune()
+         let mut next = *self;
+ 
+         next.mix(&(next * 64));
+         next.prune();
+ 
+         next.mix(&(next / 32));
+         next.prune();
+ 
+         next.mix(&(next * 2048));
+         next.prune();
+ 
+         next
      }
  }
```

I try switching out `HashMap` and `HashSet` for `rustc_hash::FxHashMap` and `rustc_hash::FxHashSet` which have
quicker hashing, which halves the time to ~!50ms. There is also repeated work in diffing the sequence pairs for each
window. Moving this to calculating the diffs in a separate step of the iterator saves another ~50ms, also as the
sequence just needs to stay unique I can add 10 to each diff to keep from switching between signed and unsigned ints.

```diff
  fn populate_sequence_scores(
-     sequence_scores: &mut HashMap<(i8, i8, i8, i8), u64>, 
+     sequence_scores: &mut FxHashMap<(i8, i8, i8, i8), u64>, 
      seed: u64
  ) {
-     let mut seen = HashSet::new();
+     let mut seen = FxHashSet::default();
      pseudorandom_sequence(seed)
          .take(2000)
-         .map(|secret| (secret % 10) as i8)
+         .map(|secret| (secret % 10) as u8)
          .tuple_windows()
-         .for_each(|(a, b, c, d, e)| {
-             let diff_sequence = (b - a, c - b, d - c, e - d);
+         .map(|(prev, current)| (10 + current - prev, current as u32))
+         .tuple_windows()
+         .for_each(|((a, _), (b, _), (c, _), (d, price))| {
+             let diff_sequence = (a, b, c, d);
              if seen.insert(diff_sequence) {
-                 *(sequence_scores.entry(diff_sequence).or_default()) += e as u64;
+                 *(sequence_scores.entry(diff_sequence).or_default()) += price;
              }
          })
  }
```

The other suggestion I see online is using a `Vec` instead of hashing things at all. For this to work the keys need
to be integers. Each entry is in the range, ±9 with +10 to keep it positive, that is 19, and fits in 5 bits. With
some bit-shifting / bit-masking I can store the current sequence of four diffs in 20 bits, which fits in  `u32` as
follows:

- Starting with the previous seed, apply a bitmask to keep only the first 15 bits (the latest three diffs in the
  sequence)
- Shift that number 5 bits leftwards, leaving the least-significant 5 bits clear
- Add the newest diff.

This takes the place of the 4-tuple windows step. I can replace that with a `seed` call that is like a map, but with
mutable internal state being passed to each of the callbacks. I also need to discard the first three entries as they
aren't generated from a full set of four diffs. I also steal another trick, which is passing in a mutable seen `Vec`
which only needs to be allocated once, and the index of the seed in the list. Then, if the seen list doesn't equal
the current id, set it to the current id and store the price in the sequence_scores `Vec`.

```diff
+ fn shift_diff_into_sequence_id(state: &mut usize, prev: usize, current: usize) {
+     *state &= (1 << 15) - 1;
+     *state <<= 5;
+     *state += 10 + current - prev;
+ }
  
  fn populate_sequence_scores(
-     sequence_scores: &mut FxHashMap<(u8, u8, u8, u8), u32>, 
-     seed: u64
+     sequence_scores: &mut Vec<u32>,
+     seen: &mut Vec<u32>,
+     seed: u64,
+     id: u32,
  ) {
-     let mut seen = FxHashSet::default();
      pseudorandom_sequence(seed)
          .take(2000)
          .map(|secret| secret % 10)
          .tuple_windows()
-         .map(|(prev, current)| (10 + current - prev, current as u32))
-         .tuple_windows()
-         .for_each(|((a, _), (b, _), (c, _), (d, price))| {
-             let diff_sequence = (a, b, c, d);
-             if seen.insert(diff_sequence) {
-                 *(sequence_scores.entry(diff_sequence).or_default()) += price;
+         .scan(0, |state, (prev, current)| {
+             shift_diff_into_sequence_id(state, prev, current);
+             Some((*state, current as u32))
+         })
+         .for_each(|(sequence, price)| {
+             if seen[sequence] != id {
+                 seen[sequence] = id;
+                 sequence_scores[sequence] += price
              }
          })
  }
```

This brings the running time down to ~25ms, and I'm going to leave it there.

## Wrap up

The actual puzzle today was fairly formulaic. It was quite interesting playing with the performance optimisations
and seeing what actually made a difference to the running time.
