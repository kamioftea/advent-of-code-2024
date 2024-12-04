---
day: 3
tags: [ post ]
header: 'Day 3: Mull It Over'
---

Today was pulling things that look like a multiplication instruction out of a larger string of nonsense. This led
to me reaching for regular expressions. I try to avoid these where `trim`s, `split`s,  `replace`s, etc. can be used
as they're quite inefficient in comparison, but in this case they're a good fit.

## Parse Input

The regex should match `mul` followed by two positive numbers in braces, without any other characters (including
spaces). The numbers have to be 1 to 3 digits. This translates directly into a regex, which can then provide an
iterator or each match it finds in a string. `Captures::extract` can be used to pull out the two digit's capturing
groups, and these parsed into numbers. Because the regex can only return valid number strings, it's safe to use
`unwrap`.

```rust
fn extract_instructions(program: &String) -> Vec<(u32, u32)> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    pattern
        .captures_iter(program)
        .map(|c| c.extract())
        .map(|(_, [lhs, rhs])| (lhs.parse().unwrap(), rhs.parse().unwrap()))
        .collect()
}

//noinspection SpellCheckingInspection
fn sample_input() -> String {
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        .to_string()
}

#[test]
fn can_extract_instructions() {
    assert_eq!(
        extract_instructions(&sample_input()),
        vec![(2, 4), (5, 5), (11, 8), (8, 5)]
    )
}
```

## Part 1

The parsing has done most of the work for the puzzle, all that is left to do is apply the multiplications and sum
the results.

```rust
fn sum_instructions(instructions: &Vec<(u32, u32)>) -> u32 {
    instructions.iter().map(|(lhs, rhs)| lhs * rhs).sum()
}

#[test]
fn can_sum_instructions() {
    assert_eq!(
        sum_instructions(&vec![(2, 4), (5, 5), (11, 8), (8, 5)]),
        161
    )
}
```

## Part 2

The twist is that some of the rest of the nonsense input is `do()` and `don't()` instructions. `don't` should
cause the `mul` instructions to be ignored until a `do()` is seen. These don't nest so multiple `don't()` will still
be cancelled by a single `do`.

First I'll introduce a type to capture the three instructions.

```rust
use Instruction::*;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}
```

I then have a bunch of attempts and refinements to get an updated parser that captures the instructions correctly. I
start with `r"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)` but it's awkward to have each instruction in its
own capturing group when testing results. I also have to stop using `Captures::extract` as it doesn't work with
matches having variable numbers of capturing groups. This causes me to switch to named capturing groups so that it's
a bit clearer what I'm accessing from the matches that way. Finally, the Regex is now a bit more complex, so I use
the verbose flag to allow me to format it and add comments.

```rust
fn parse_named_group(c: &Captures, name: &str) -> u32 {
    c.name(name).unwrap().as_str().parse().unwrap()
}

fn extract_instructions(program: &String) -> Vec<Instruction> {
    let pattern = Regex::new(
        r"(?x)        # Enable verbose mode
(?<inst>mul|don't|do) # The instructions name
\(                    # Open the arguments list
  (                   # Optionally caputure two 1-3 digit arguments
    (?<lhs>\d{1,3}),
    (?<rhs>\d{1,3})
  )?
\)                    # Finally close the arguments list",
    )
        .unwrap();
    
    pattern
        .captures_iter(program)
        .map(|c| {
            let instruction = c.name("inst").map(|m| m.as_str());
            match instruction {
                Some("mul") =>
                    Mul(
                        parse_named_group(&c, "lhs"),
                        parse_named_group(&c, "rhs")
                    ),
                Some("do") => Do,
                Some("don't") => Dont,
                inst => unreachable!("Unexpected instruction '{:?}'", inst),
            }
        })
        .collect()
}

#[test]
fn can_extract_instructions() {
    let input =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            .to_string();
    
    assert_eq!(
        extract_instructions(&input),
        vec![Mul(2, 4), Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)]
    )
}
```

Technically this could match e.g. `do(1,23)` or `don't(456,7)`, but that doesn't cause issues with the puzzle input, so
I'm going to ignore that.

The aggregator for part 1 needs to be updated to use `Instruction::Mul`.

```rust
fn sum_muls(instructions: &Vec<Instruction>) -> u32 {
    instructions
        .iter()
        .map(|instruction| match instruction {
            Mul(lhs, rhs) => lhs * rhs,
            _ => 0,
        })
        .sum()
}
```

Finally, I need an equivalent that tracks if a `don't()` has disabled applying the instructions. This means
switching the sum for a fold that also tracks a flag for if summing is active or not.

```rust
fn sum_instructions(instructions: &Vec<Instruction>) -> u32 {
    instructions
        .iter()
        .fold((0, true), |(sum, active), instruction| match instruction {
            Mul(lhs, rhs) => (sum + if active { lhs * rhs } else { 0 }, active),
            Do => (sum, true),
            Dont => (sum, false),
        })
        .0
}

#[test]
fn can_sum_instructions() {
    assert_eq!(
        sum_instructions(
            &vec![Mul(2, 4), Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)]
        ),
        48
    )
}
```

## Wrap Up

It took a lot of refactoring to get to code I was happy with. The extra boilerplate needed to satisfy the
type/borrow checker got quite verbose at times. I think I've been able to work it into something that expresses
what it is doing quite well, so I'm happy with that.
