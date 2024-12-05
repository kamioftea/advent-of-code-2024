---
day: 5
tags: [ post ]
header: 'Day 5: Print Queue'
---

Today was about ordering pages based on a list of rules, each a specific order for two of the page numbers. The
first of this year with two sections of input. I have some intuition that treating the list of rules for a specific
starting page should be a set to allow efficient lookups. Then for each list of pages to print, for each pair
compare what has been seen before with what has to be after, and if there is any overlap, that set of updated pages
is invalid.

## Parsing the input

First some types

```rust
type Rules = HashMap<u32, HashSet<u32>>;
type Update = Vec<u32>;
```

The more complex part of the input is reducing the rules into a form that is efficient to use. I'm choosing to
represent the rule list as a map from page number to the set of pages that have to be after it. This can be built by
inserting each rule into the relevant set, with `Map::Entry::or_default` to create an empty set the first time each
left-hand page number is seen.

```rust
fn parse_rules(input: &str) -> Rules {
    let mut rules: Rules = HashMap::new();
    input
        .lines()
        .flat_map(|line| line.split_once("|"))
        .for_each(|(before, after)| {
            let key = before.parse().unwrap();
            let value = after.parse().unwrap();
            
            rules.entry(key).or_default().insert(value);
        });
    
    rules
}
```

The update lists is a more simple conversion into a nested `Vec<Vec<u32>>`.

```rust
fn parse_updates(input: &str) -> Vec<Update> {
    input
        .lines()
        .map(|line| line.split(",").flat_map(|page| page.parse()).collect())
        .collect()
}
```

The full input could be split on the blank line. It was easiest to test the parsing as a whole. It was a bit of work
to convert the input into expected results, but quicker than causing a hard to detect bug.

```rust
fn parse_input(input: &String) -> (Rules, Vec<Update>) {
    let (rule_input, updates_input) = input.split_once("\n\n").unwrap();
    
    (parse_rules(rule_input), parse_updates(updates_input))
}

fn example_rules() -> Rules {
    vec![
        (97, vec![13, 61, 47, 29, 53, 75].into_iter().collect()),
        (75, vec![29, 53, 47, 61, 13].into_iter().collect()),
        (61, vec![13, 53, 29].into_iter().collect()),
        (29, vec![13].into_iter().collect()),
        (53, vec![29, 13].into_iter().collect()),
        (47, vec![53, 13, 61, 29].into_iter().collect()),
    ]
        .into_iter()
        .collect()
}

fn example_updates() -> Vec<Update> {
    vec![
        vec![75, 47, 61, 53, 29],
        vec![97, 61, 53, 29, 13],
        vec![75, 29, 13],
        vec![75, 97, 47, 61, 53],
        vec![61, 13, 29],
        vec![97, 13, 75, 29, 47],
    ]
}

#[test]
fn can_parse_input() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        .to_string();
    
    let (rules, updates) = parse_input(&input);
    
    assert_contains_in_any_order(rules, example_rules());
    assert_contains_in_any_order(updates, example_updates());
}
```

See the third code block in the [refactoring section from day 3, 2023](
https://www.jeff-horton.uk/advent-of-code-2023/blog/day_3/#refactoring) for more details about
`assert_contains_in_any_order`.

## Part 1 - Pager duty

The next step is to work out which lists of updated pages are in the correct order. I've decided to reduce this to
checking that there is no intersection between the set of pages previously seen and the list of pages that have to
come later. I don't think it actually matters for the puzzle input, but where there isn't any rules it defaults to
an empty set for convenience.

```rust
fn validate_update(update: &Update, rules: &Rules) -> bool {
    let mut seen = HashSet::new();
    let empty = HashSet::new();
    
    for page in update {
        let rule = rules.get(page).unwrap_or(&empty);
        if seen.intersection(rule).next().is_some() {
            return false;
        }
        seen.insert(*page);
    }
    
    true
}

#[test]
fn can_validate_updates() {
    let rules = example_rules();
    
    assert!(validate_update(&vec![75, 47, 61, 53, 29], &rules));
    assert!(validate_update(&vec![97, 61, 53, 29, 13], &rules));
    assert!(validate_update(&vec![75, 29, 13], &rules));
    assert!(!validate_update(&vec![75, 97, 47, 61, 53], &rules));
    assert!(!validate_update(&vec![61, 13, 29], &rules));
    assert!(!validate_update(&vec![97, 13, 75, 29, 47], &rules));
}
```

To reduce the list of updates to the puzzle solution, I have to:

* Find all the correctly ordered update lists
* Map them to their middle page number
* Sum those page numbers

```rust
fn get_middle(update: &Update) -> u32 {
    let middle = (update.len() - 1) / 2;
    update.get(middle).unwrap().clone()
}

fn sum_valid_middle_pages(updates: &Vec<Update>, rules: &Rules) -> u32 {
    updates
        .iter()
        .filter(|update| validate_update(update, rules))
        .map(|update| get_middle(update))
        .sum()
}

#[test]
fn can_get_middle_page() {
    assert_eq!(get_middle(&vec![75, 47, 61, 53, 29]), 61);
    assert_eq!(get_middle(&vec![97, 61, 53, 29, 13]), 53);
    assert_eq!(get_middle(&vec![75, 29, 13]), 29);
    assert_eq!(get_middle(&vec![75, 97, 47, 61, 53]), 47);
    assert_eq!(get_middle(&vec![61, 13, 29]), 13);
    assert_eq!(get_middle(&vec![97, 13, 75, 29, 47]), 75);
}

#[test]
fn can_sum_valid_middle_pages() {
    assert_eq!(
        sum_valid_middle_pages(&example_updates(), &example_rules()),
        143
    )
}
```

## Part 2 - Ordering some pages

The almost inevitable part two is sorting those update lists that are not in the correct order. I originally was
quite daunted by this, because working out relative positions has a lot of moving parts. Then I remembered that I
could use the built-in sorting of collections if I could provide a function that can sort any two pages in the list.
That could be done by checking if either page was in the other page's list of later pages and returning the
corresponding `Ordering`. This could lead to some wierd behaviour if there are page pairs that don't explicitly have
rules, but rely on surrounding page's rules to imply an ordering, but I'll try this first and revisit the plan if it
doesn't work.

```rust
fn sort_pages(update: &Update, rules: &Rules) -> Update {
    let empty = HashSet::new();
    
    update
        .iter()
        .sorted_by(|page_a, page_b| {
            let rule_a = rules.get(page_a).unwrap_or(&empty);
            let rule_b = rules.get(page_b).unwrap_or(&empty);
            
            if rule_a.contains(page_b) {
                Ordering::Less
            } else if rule_b.contains(page_a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
        .cloned()
        .collect()
}

#[test]
fn can_sort_pages() {
    let rules = example_rules();
    assert_eq!(
        sort_pages(&vec![75, 97, 47, 61, 53], &rules),
        vec![97, 75, 47, 61, 53]
    );
    assert_eq!(sort_pages(&vec![61, 13, 29], &rules), vec![61, 29, 13]);
    assert_eq!(
        sort_pages(&vec![97, 13, 75, 29, 47], &rules),
        vec![97, 75, 47, 29, 13]
    );
}
```

Reducing that to the puzzle solution is very similar to part 1, instead selecting and mapping the invalid lists.

```rust
fn sort_and_sum_invalid_middle_pages(updates: &Vec<Update>, rules: &Rules) -> u32 {
    updates
        .iter()
        .filter(|update| !validate_update(update, rules))
        .map(|update| sort_pages(update, &rules))
        .map(|update| get_middle(&update))
        .sum()
}

#[test]
fn can_sort_and_sum_invalid_middle_pages() {
    assert_eq!(
        sort_and_sum_invalid_middle_pages(&example_updates(), &example_rules()),
        123
    )
}
```

## Wrap up

I feel that my previous advent of code experience helped today. Intuiting the need for a `HashSet` here comes from
learning the hard way. I've previously been bitten by using `Vec`s in similar puzzles, and having performance issue
as a result.  
