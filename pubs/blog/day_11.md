---
day: 11
tags: [ post ]
header: 'Day 11: Plutonian Pebbles'
---

Today there is a very thinly veiled description of a list of ints, that change everytime I blink. This feels like a
day when part 2 is going to be part 1, but big enough to take months. I don't instantly have an idea for making it
efficient now, so I'll implement part 1 naively, and then see exactly what the twist is, and hopefully I'll have some
more insight into how it works by then.

## Parsing the input

This seemed trivial, but ended up having a silly bug, so it still gets its own section. I originally had

```rust
fn parse_input(input: &String) -> Vec<u64> {
    input
        .split(" ")
        .flat_map(|num| num.parse())
        .collect()
}

#[test]
fn can_parse_input() {
    assert_eq!(
        parse_input(&"0 1 10 99 999".to_string()),
        vec![0, 1, 10, 99, 999]
    )
}
```

This worked, it wasn't until I submitted and got an answer that was too low that I noticed the subtle issue. The
input file ends with a new line, and `999\n` doesn't parse to u64. Because of the flat_map it silently dropped the
final number. The fix was to trim, but I also switched to map + unwrap so that unexpected input causes an error. I
also updated the test to include a terminating newline.

```rust
fn parse_input(input: &String) -> Vec<u64> {
    input
        .trim()
        .split(" ")
        .map(|num| num.parse().unwrap())
        .collect()
}

#[test]
fn can_parse_input() {
    assert_eq!(
        parse_input(&"0 1 10 99 999\n".to_string()),
        vec![0, 1, 10, 99, 999]
    )
}
```

## Part 1 - In the blink of an eye

First I'll implement a single blink. I can think of two ways to do the splitting:

- Convert to string, split at the midpoint, convert each back to a `u64`
- Use $log_{10} + 1$ to get the number of digits, ${stone}/{10^{digits / 2}}$ and $stone \mod 10^{digits / 2}$ to
  calculate the halves.

In the end I decide neither are more readable, and go with the log/pow version that is slightly quicker with a
comment for future me.

```rust
fn blink(stones: &Vec<u64>) -> Vec<u64> {
    stones
        .into_iter()
        .flat_map(|&stone| {
            if stone == 0 {
                return vec![1];
            }
            
            // Even number of digits, split in two
            let digits = stone.ilog(10) + 1;
            if digits % 2 == 0 {
                let midpoint = 10u64.pow(digits / 2);
                return vec![stone / midpoint, stone % midpoint];
            }
            
            return vec![stone * 2024];
        })
        .collect()
}

#[test]
fn can_step_stones() {
    assert_eq!(
        blink(&vec![0, 1, 10, 99, 999]),
        vec![1, 2024, 1, 0, 9, 9, 2021976]
    );
    assert_eq!(blink(&vec![125, 17]), vec![253000, 1, 7]);
    assert_eq!(blink(&vec![253000, 1, 7]), vec![253, 0, 2024, 14168]);
    assert_eq!(
        blink(&vec![253, 0, 2024, 14168]),
        vec![512072, 1, 20, 24, 28676032]
    );
    assert_eq!(
        blink(&vec![512072, 1, 20, 24, 28676032]),
        vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]
    );
    assert_eq!(
        blink(&vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]),
        vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
    );
    assert_eq!(
        blink(&vec![
            1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32
        ]),
        vec![
            2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48,
            80, 96, 2, 8, 6, 7, 6, 0, 3, 2
        ]
    );
}
```

Then that needs to be called 25 times.

```rust
fn count_after_blinks(stones: &Vec<u64>, number_of_blinks: u64) -> usize {
    let mut stones = stones.clone();
    
    for _ in 0..number_of_blinks {
        stones = blink(&stones)
    }
    
    stones.len()
}

#[test]
fn can_count_stones_after_n_blinks() {
    assert_eq!(count_after_blinks(&vec![125, 17], 6), 22);
    
    assert_eq!(count_after_blinks(&vec![125, 17], 25), 55312);
}
```

## Part 2 - An exponential stone gathers no moss

There is nothing fancy about part 2, it's the same but another 50 times, which very quickly blows up. One thing to
note is that there are a lot of repeated numbers. For example exploding `[1]`.

```text
[1]
[2024]
[20, 24]
[2, 0, 2, 4]
[4048, 1, 4048, 8096]
[40, 48, 2024, 40, 48, 80, 96]
[4, 0, 4, 8, 20, 24, 4, 0, 4, 8, 8, 0, 9, 6]
```

This suggests some form of caching the results would be useful. The current implementation doesn't lend itself
well to caching. Storing the numbers as a list will also get very big. Each stone doesn't care about its neighbours,
so a recursive function keeping track of the count would be enough.

The result is always going to be the same for a specific stone with the same number of iterations left, so that is
the cache key. I originally manage this with a `HashMap<(stone#, iterations#), result>`.

```rust
fn count_for_stone(
    stone: u64,
    blinks: u8,
    cache: &mut HashMap<(u64, u8), usize>
) -> usize {
    if let Some(&count) = cache.get(&(stone, blinks)) {
        return count;
    }
    
    // ... Calculate result
    
    cache.insert((stone, blinks), result);
    result
}
```

But passing the cache everywhere is a bit clunky, and it feels like there should be a generic way to do this. In
looking for this I found the [`cached` crate](https://docs.rs/cached/latest/cached/). That separates out the caching
into its own concern, keeping the code logic clearer.

```rust
#[cached]
fn count_for_stone(stone: u64, blinks: u8) -> usize {
    if blinks == 0 {
        return 1;
    }
    
    let result = blink(&vec![stone])
        .iter()
        .map(|&next_stone| count_for_stone(next_stone, blinks - 1))
        .sum();
    
    result
}

fn count_after_blinks(stones: &Vec<u64>, number_of_blinks: u8) -> usize {
    stones
        .iter()
        .map(|&stone| count_for_stone(stone, number_of_blinks))
        .sum()
}
```

All the tests still pass, and it runs both parts in ~160ms (~25ms when optimised). I tried inlining the blink to
avoid building a bunch of `Vec`s. It was about 2x quicker, but it's harder to write understandable tests for that,
and saving ~12ms is not worth it.

## Wrap up

I think I spent about as much time on fixing the parsing bug as I did the main puzzle. I feel I sort of lucked into
the solution. I find it quite hard to visualise what's going on when numbers get this big.
