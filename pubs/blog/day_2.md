---
day: 2
tags: [ post ]
header: 'Day 2: Red-Nosed Reports'
---

Today's data is rows of numbers representing levels so something in a reactor. The ask is to find those where they
are not gradually increasing or decreasing.

## Parsing the input

The internal representation is pretty close to the input text today. Each line maps to a report of unsigned numbers,
and the whole thing is then a list of reports.

```rust
type Report = Vec<u32>;

fn parse_line(line: &str) -> Report {
    line.split(" ").flat_map(|num| num.parse()).collect()
}

fn parse_input(input: &String) -> Vec<Report> {
    input.lines().map(parse_line).collect()
}

fn sample_input() -> String {
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        .to_string()
}

fn sample_reports() -> Vec<Report> {
    vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ]
}

#[test]
fn can_parse_input() {
    assert_eq!(parse_input(&sample_input()), sample_reports())
}
```

## Part 1

Itertools provides `Itertools::tuple_windows` that returns an iterator over each consecutive tuple of numbers, and
can determine from the pattern matching the size of the expected tuples. Determining if the numbers are not
changing, or changing too quickly can be done based on the absolute diff. The direction is slightly more awkward as
that is determined by the first pair. I've implemented this by having an external option that is set on the first
pair, then the rest of the pairs are checked against that first one.

```rust
fn is_report_safe(report: &Report) -> bool {
    let mut maybe_direction = None;
    for (&l, &r) in report.iter().tuple_windows() {
        if l == r || l.abs_diff(r) > 3 {
            return false;
        }
        
        maybe_direction = maybe_direction.or_else(|| Some(l > r));
        let direction = maybe_direction.unwrap();
        
        if direction ^ (l > r) {
            return false;
        }
    }
    
    true
}

#[test]
fn can_check_if_a_report_is_safe() {
    assert_eq!(is_report_safe(&vec![7, 6, 4, 2, 1]), true);
    assert_eq!(is_report_safe(&vec![1, 2, 7, 8, 9]), false);
    assert_eq!(is_report_safe(&vec![9, 7, 6, 2, 1]), false);
    assert_eq!(is_report_safe(&vec![1, 3, 2, 4, 5]), false);
    assert_eq!(is_report_safe(&vec![8, 6, 4, 4, 1]), false);
    assert_eq!(is_report_safe(&vec![1, 3, 6, 7, 9]), true);
}
```

The hard work done, the puzzle solution is the count of reports that pass the safety check.

```rust
fn analyse_reports(reports: &Vec<Report>) -> usize {
    reports
        .into_iter()
        .filter(|&report| is_report_safe(report))
        .count()
}

#[test]
fn can_analyse_reports() {
    assert_eq!(analyse_reports(&sample_reports()), 2)
}
```

## Part 2

Today's twist is that the reactor is more tolerant than first described. A report can be safe enough if only one of
the reported levels is unsafe, and removing that level makes the remaining report safe. Thinking about this, I can
reuse the existing check to return which pair failed. The level removed must be one of those two in most cases.

First I'll refactor the safety check. The list of pairs needs to be enumerated so that I can return the position if
it fails. Instead of a boolean, it now returns an Option which will be `None` if the report is safe, or `Some(index)`
if a bad pair is found.

```rust
fn first_bad_level_pair(report: &Report) -> Option<usize> {
    let mut maybe_direction = None;
    for (idx, (&l, &r)) in report.iter().tuple_windows().enumerate() {
        if l == r || l.abs_diff(r) > 3 {
            return Some(idx);
        }
        
        maybe_direction = maybe_direction.or_else(|| Some(l > r));
        let direction = maybe_direction.unwrap();
        
        if direction ^ (l > r) {
            return Some(idx);
        }
    }
    
    None
}
```

The tests and part 1 solution need to be updated to match the new return type.

```rust
#[test]
fn can_check_if_a_report_is_safe() {
    assert_eq!(first_bad_level_pair(&vec![7, 6, 4, 2, 1]), None);
    assert_eq!(first_bad_level_pair(&vec![1, 2, 7, 8, 9]), Some(1));
    assert_eq!(first_bad_level_pair(&vec![9, 7, 6, 2, 1]), Some(2));
    assert_eq!(first_bad_level_pair(&vec![1, 3, 2, 4, 5]), Some(1));
    assert_eq!(first_bad_level_pair(&vec![8, 6, 4, 4, 1]), Some(2));
    assert_eq!(first_bad_level_pair(&vec![1, 3, 6, 7, 9]), None);
    assert_eq!(first_bad_level_pair(&vec![4, 3, 6, 7, 9]), Some(1));
}

fn analyse_reports(reports: &Vec<Report>) -> usize {
    reports
        .into_iter()
        .filter(|&report| first_bad_level_pair(report).is_none())
        .count()
}
```

The index of the first level with an error is then the returned index or the next one, and that index must exist
because the pairs list has one less entry than the report. There is a final permutation is if the second pair in the
report fails - it could because the first number is the only one not in ascending/descending order. For example, a
report of `[5, 3, 4, 7, 9]` could be safe if the first level was removed, but the test will fail on the second `(3, 4)`
pair because it is ascending where the first pair was descending. To catch this I can add a special case when the
problem is at index 1.

Testing each permutation of removing digits can then be done with the existing safety check, and if any pass then
the report can be considered safe. I couldn't find a library method to remove an item from a list that didn't mutate
the list, so I wrote my own. The edge case was also not covered by the example input, so I added an extra test case
to cover it.

```rust
fn without_index(report: &Report, idx: usize) -> Report {
    let mut new = report.clone();
    new.remove(idx);
    
    new
}

fn report_check_with_dampener(report: &Report) -> bool {
    if let Some(pair_idx) = first_bad_level_pair(report) {
        let lower_bound = if pair_idx == 1 { 0 } else { pair_idx };
        (lower_bound..=(pair_idx + 1))
            .into_iter()
            .any(|level_idx| {
                first_bad_level_pair(&without_index(report, level_idx)).is_none()
            })
    } else {
        true
    }
}

#[test]
fn can_check_if_a_report_is_safe_with_dampener() {
    assert_eq!(report_check_with_dampener(&vec![7, 6, 4, 2, 1]), true);
    assert_eq!(report_check_with_dampener(&vec![1, 2, 7, 8, 9]), false);
    assert_eq!(report_check_with_dampener(&vec![9, 7, 6, 2, 1]), false);
    assert_eq!(report_check_with_dampener(&vec![1, 3, 2, 4, 5]), true);
    assert_eq!(report_check_with_dampener(&vec![8, 6, 4, 4, 1]), true);
    assert_eq!(report_check_with_dampener(&vec![1, 3, 6, 7, 9]), true);
    assert_eq!(report_check_with_dampener(&vec![5, 3, 4, 7, 9]), true);
}
```

The final count of safe tests is the same as part 1. I could make this generic - taking the predicate as an argument,
but it's not really worth the effort.

```rust
fn analyse_reports_with_dampener(reports: &Vec<Report>) -> usize {
    reports
        .into_iter()
        .filter(|&report| report_check_with_dampener(report))
        .count()
}

#[test]
fn can_analyse_reports_with_dampener() {
    assert_eq!(analyse_reports_with_dampener(&sample_reports()), 5)
}
```

## Wrap up

I was guided once again by methods that exist on iterators towards a solution. It was a bit weird that day two had
examples that didn't cover the edge case. The puzzle was hinting to try every permutation of dropping one level,
which wouldn't have hit the edge case, so perhaps it's me trying to be more efficient than needed.
