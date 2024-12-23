---
day: 23
tags: [ post ]
header: 'Day 23: LAN Party'
---

Today looks to be an undirected graph traversal problem, with the graph representing a computer network.

## Parsing the input

I may later need to turn the computer ids into numbers, but I'll wait until using string slices becomes an issue.
Finding the computers that start with `t` for part 1 will be easier if they're strings. Because `&str`s are
inherently a reference, I need to put some lifetimes on the struct, but hopefully all the logic will be constrained
to methods on the struct, so they can use the `struct`'s lifetime which should mean I can mostly ignore them.

As the connections are undirected I double up the data for convenience, inserting both computers in a pair into the
connected set for the other. I.e. If `ab` is connected to `cd` then I add `cd` to `ab`'s set of connected computers,
and `ab` into `cd`'s set.

```rust
#[derive(Eq, PartialEq, Debug)]
struct Network<'a> {
    links: HashMap<&'a str, HashSet<&'a str>>,
}

fn parse_input(input: &String) -> Network {
    let mut links: HashMap<&str, HashSet<&str>> = HashMap::new();
    
    for (a, b) in input.lines().map(|line| line.split_once("-").unwrap()) {
        links.entry(a).or_default().insert(b);
        links.entry(b).or_default().insert(a);
    }
    
    Network { links }
}

fn example_network() -> Network<'static> {
    let links = vec![
        ("kh", vec!["tc", "qp", "ub", "ta"].into_iter().collect()),
        ("tc", vec!["kh", "wh", "td", "co"].into_iter().collect()),
        ("qp", vec!["kh", "ub", "td", "wh"].into_iter().collect()),
        ("de", vec!["cg", "co", "ta", "ka"].into_iter().collect()),
        ("cg", vec!["de", "tb", "yn", "aq"].into_iter().collect()),
        ("ka", vec!["co", "tb", "ta", "de"].into_iter().collect()),
        ("co", vec!["ka", "ta", "de", "tc"].into_iter().collect()),
        ("yn", vec!["aq", "cg", "wh", "td"].into_iter().collect()),
        ("aq", vec!["yn", "vc", "cg", "wq"].into_iter().collect()),
        ("ub", vec!["qp", "kh", "wq", "vc"].into_iter().collect()),
        ("tb", vec!["cg", "ka", "wq", "vc"].into_iter().collect()),
        ("vc", vec!["aq", "ub", "wq", "tb"].into_iter().collect()),
        ("wh", vec!["tc", "td", "yn", "qp"].into_iter().collect()),
        ("ta", vec!["co", "ka", "de", "kh"].into_iter().collect()),
        ("td", vec!["tc", "wh", "qp", "yn"].into_iter().collect()),
        ("wq", vec!["tb", "ub", "aq", "vc"].into_iter().collect()),
    ]
        .into_iter()
        .collect();
    
    Network { links }
}

#[test]
fn can_parse_input() {
    let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"
        .to_string();
    
    let actual = parse_input(&input);
    let expected = example_network();
    
    assert_contains_in_any_order(actual.links.keys(), expected.links.keys());
    for (key, value) in actual.links {
        assert_contains_in_any_order(&value, expected.links.get(key).unwrap())
    }
}
```

## Part 1 - ...and three come along at once

I need to find all the unique sets of three interconnected computers in the network. I can do this by starting with
the list connected to a computer, then finding pairs within it's links that are also connected to each other.

```rust
impl<'a> Network<'a> {
    fn trios(&self) -> HashSet<Vec<&str>> {
        let mut clusters = HashSet::new();
        
        for (start, connected) in self.links.clone() {
            for (a, b) in connected.iter().tuple_combinations() {
                if self.links.get(a).unwrap().contains(b) {
                    clusters.insert(vec![start, a, b].into_iter().sorted().collect());
                }
            }
        }
        
        clusters
    }
}

#[test]
fn can_find_clusters() {
    assert_eq!(
        example_network().trios(),
        vec![
            vec!["aq", "cg", "yn"],
            vec!["aq", "vc", "wq"],
            vec!["co", "de", "ka"],
            vec!["co", "de", "ta"],
            vec!["co", "ka", "ta"],
            vec!["de", "ka", "ta"],
            vec!["kh", "qp", "ub"],
            vec!["qp", "td", "wh"],
            vec!["tb", "vc", "wq"],
            vec!["tc", "td", "wh"],
            vec!["td", "wh", "yn"],
            vec!["ub", "vc", "wq"],
        ]
            .into_iter()
            .collect()
    );
}
```

I can then filter out those that don't contain a node starting with `t`.

```rust
impl<'a> Network<'a> {
    fn clusters_containing(&self, char: &str) -> Vec<Vec<&str>> {
        self.trios()
            .iter()
            .filter(|cluster| cluster.iter().any(|node| node.starts_with(char)))
            .cloned()
            .collect()
    }
}

#[test]
fn can_find_clusters_starting_with_t() {
    assert_contains_in_any_order(
        example_network().clusters_containing("t"),
        vec![
            vec!["co", "de", "ta"],
            vec!["co", "ka", "ta"],
            vec!["de", "ka", "ta"],
            vec!["qp", "td", "wh"],
            vec!["tb", "vc", "wq"],
            vec!["tc", "td", "wh"],
            vec!["td", "wh", "yn"],
        ]
            .into_iter()
            .collect::<Vec<Vec<&str>>>(),
    );
}
```

I could do the filtering on the start nodes of the trios, but that's working quick enough.

## Part 2 - Crashing the party

I now need to find the biggest interconnected cluster of computers. I first of all try what feels like a naive way of
finding the cluster. If I start from each node and walk its connections and include them in the cluster if they're
connected to every member so far it will find clusters that meet the criteria, but I'm not sure if it will always
identify the biggest. It's a starting point and I can review later if it doesn't work.

I split this up into building up the cluster from a single computer and its list of connections. Then call that for
each computer and find the biggest. That can then be sorted and joined into the password.

```rust
impl<'a> Network<'a> {
    fn find_fully_connected_cluster(
        &self,
        start: &'a str,
        connected: &HashSet<&'a str>,
    ) -> Vec<&str> {
        let mut cluster = vec![start];
        for computer in connected {
            if cluster.iter().all(|b| self.links[computer].contains(b)) {
                cluster.push(*computer);
            }
        }
        cluster
    }
    
    fn find_lan_password(&self) -> String {
        self.links
            .iter()
            .map(|(&start, connected)| self.find_fully_connected_cluster(start, connected))
            .max_by_key(|c| c.len())
            .unwrap()
            .iter()
            .sorted()
            .join(",")
    }
}

#[test]
fn can_find_lan_password() {
    assert_eq!(example_network().find_lan_password(), "co,de,ka,ta");
}
```

This works, and is reasonably quick, so I'm happy to leave it there.

## Wrap up

Today felt like there was something I was missing. I think my solution happened to work because the input was
designed in a way that the naive approach worked, when more generally it wouldn't.
