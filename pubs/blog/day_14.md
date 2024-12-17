---
day: 14
tags: [ post ]
header: 'Day 14: Restroom Redoubt'
---

Today there are lots of robots moving in diagonals. This feels like a modular arithmetic puzzle.

## Parsing the input

First I need a way to represent each robot.

```rust
type Position = (usize, usize);
type Velocity = (isize, isize);

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity }
    }
}
```

Each line maps to a robot, but needs to be split into the different numbers so it seems best to implement `FromStr`.
The awkward part here is the position and velocity are parsed in exactly the same way, except that velocity needs to
be an `isize`. I could make them both `isize`s, but would mean a lot of conversion later. So I have a go at
implementing a part parser in a generic way.

```rust
impl FromStr for Robot {
    type Err = ();
    
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn parse_part<T>(part: &str) -> (T, T)
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            let (_, values) = part.split_once("=").unwrap();
            // Coordinates are x, y
            let (c, r) = values.split_once(",").unwrap();
            (r.parse::<T>().unwrap(), c.parse::<T>().unwrap())
        }
        
        let (position, velocity) = line.split_once(" ").ok_or(())?;
        
        Ok(Robot::new(parse_part(position), parse_part(velocity)))
    }
}
```

This is the first time that using `r` and `c` instead of `x`, and `y` has been slightly awkward. Because the
coordinates are expressed in `x` and `y`, I need to flip the ordering, which means that making test cases is a little
awkward too.

The input can then be processed line by line into a list of robots.

```rust
fn parse_input(input: &String) -> Vec<Robot> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn example_robots() -> Vec<Robot> {
    vec![
        Robot::new((4, 0), (-3, 3)),
        Robot::new((3, 6), (-3, -1)),
        Robot::new((3, 10), (2, -1)),
        Robot::new((0, 2), (-1, 2)),
        Robot::new((0, 0), (3, 1)),
        Robot::new((0, 3), (-2, -2)),
        Robot::new((6, 7), (-3, -1)),
        Robot::new((0, 3), (-2, -1)),
        Robot::new((3, 9), (3, 2)),
        Robot::new((3, 7), (2, -1)),
        Robot::new((4, 2), (-3, 2)),
        Robot::new((5, 9), (-3, -3)),
    ]
}

#[test]
fn can_parse_input() {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
        .to_string();
    
    assert_eq!(parse_input(&input), example_robots())
}
```

## Part 1 - Roving robots

Since robots wrap around, I can calculate the position after 100 steps by multiplying the velocity by the steps and
the take the modulus based on the grid size. The `%` in rust is a remainder, which is only the same as the modulus
for positive numbers. I can convert negative velocities by adding the grid width to the velocity until it is
positive, which doesn't affect the resulting modulus.

```rust
impl Robot {
    fn simulate(&self, steps: usize, &(max_r, max_c): &(usize, usize)) -> Position {
        let Robot {
            position: (r, c),
            velocity: (dr, dc),
        } = *self;
        
        let dr = ((dr % max_r as isize) + max_r as isize) as usize;
        let dc = ((dc % max_c as isize) + max_c as isize) as usize;
        
        ((r + dr * steps) % max_r, (c + dc * steps) % max_c)
    }
}

#[test]
fn can_simulate_robot() {
    let robot = Robot::new((4, 2), (-3, 2));
    assert_eq!(robot.simulate(0, &(7, 11)), (4, 2));
    assert_eq!(robot.simulate(1, &(7, 11)), (1, 4));
    assert_eq!(robot.simulate(2, &(7, 11)), (5, 6));
    assert_eq!(robot.simulate(3, &(7, 11)), (2, 8));
    assert_eq!(robot.simulate(4, &(7, 11)), (6, 10));
    assert_eq!(robot.simulate(5, &(7, 11)), (3, 1));
}
```

That done, I need to apply that to all the robots and check it produces the example grid. Turning the grid into the
internal representation is time-consuming, but it works.

```rust
fn simulate_robots(
    robots: &Vec<Robot>,
    steps: usize,
    bounds: &(usize, usize)
) -> Vec<Position> {
    robots
        .iter()
        .map(|robot| robot.simulate(steps, bounds))
        .collect()
}

fn positions_after_100_steps() -> Vec<(usize, usize)> {
    vec![
        (0, 6),
        (0, 6),
        (0, 9),
        (2, 0),
        (3, 1),
        (3, 2),
        (4, 5),
        (5, 3),
        (5, 4),
        (5, 4),
        (6, 1),
        (6, 6),
    ]
}

#[test]
fn can_simulate_robots() {
    let positions = simulate_robots(&example_robots(), 100, &(7, 11));
    assert_eq!(
        positions.iter().filter(|&&p| p == (0, 6)).count(),
        2,
        "There should be two robots at position (0,6)"
    );
    assert_eq!(
        positions.iter().filter(|&&p| p == (5, 4)).count(),
        2,
        "There should be two robots at position (5, 4)"
    );
    
    assert_contains_in_any_order(positions, positions_after_100_steps());
}
```

From those positions I need to split the robots' positions into quadrants, accounting for robots on the centre lines
not counting for any quadrant.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn partition_position(
    (r, c): Position,
    (max_r, max_c): &(usize, usize)
) -> Option<Quadrant> {
    let mid_r = max_r / 2;
    let mid_c = max_c / 2;
    
    if r < mid_r && c < mid_c {
        Some(TopLeft)
    } else if r < mid_r && c > mid_c {
        Some(TopRight)
    } else if r > mid_r && c < mid_c {
        Some(BottomLeft)
    } else if r > mid_r && c > mid_c {
        Some(BottomRight)
    } else {
        None
    }
}

#[test]
fn can_partition_robots() {
    assert_eq!(
        positions_after_100_steps()
            .into_iter()
            .map(|pos| partition_position(pos, &(7, 11)))
            .collect::<Vec<Option<Quadrant>>>(),
        vec![
            Some(TopRight),
            Some(TopRight),
            Some(TopRight),
            Some(TopLeft),
            None,
            None,
            None,
            Some(BottomLeft),
            Some(BottomLeft),
            Some(BottomLeft),
            Some(BottomLeft),
            Some(BottomRight),
        ]
    );
}
```

From there, I can use `Itertools::counts` to do the grouping and counting, and the puzzle solution is the product of
those counts.

```rust
fn total_safety_factor(
    robots: &Vec<Robot>,
    steps: usize, bounds:
    &(usize, usize)
) -> usize {
    let positions = simulate_robots(robots, steps, bounds);
    positions
        .iter()
        .flat_map(|&pos| partition_position(pos, bounds))
        .counts()
        .values()
        .product()
}

#[test]
fn can_calculate_total_safety_factor() {
    assert_eq!(total_safety_factor(&example_robots(), 100, &(7, 11)), 12)
}
```

## Part 2 - Take a picture

I have to admit that I didn't have a clue where to start with part 2, and I went to the
[advent of code subreddit](https://www.reddit.com/r/adventofcode/) to get some hints as to what the final picture
might look like. I tried to skim to see an image and avoid too many spoilers, but also noticed some memes about
clustering which led me to think about finding the most clustered frame.

Firstly I'll check that the robots loop in a reasonable amount of time. To do this I also need `simulate_robots` to
return robots, not the positions.

```rust
impl Robot {
    fn simulate(&self, steps: usize, &(max_r, max_c): &(usize, usize)) -> Robot {
        let Robot {
            position: (r, c),
            velocity: (dr, dc),
        } = *self;
        
        let dr = ((dr % max_r as isize) + max_r as isize) as usize;
        let dc = ((dc % max_c as isize) + max_c as isize) as usize;
        
        let new_pos = ((r + dr * steps) % max_r, (c + dc * steps) % max_c);
        
        Robot::new(new_pos, self.velocity)
    }
}

fn iterate_seconds<'a>(
    robots: &'a Vec<Robot>,
    bounds: &'a (usize, usize),
) -> impl Iterator<Item=Vec<Robot>> + 'a {
    let first = robots.clone();
    
    successors(Some(robots.clone()), move |robots| {
        let next = simulate_robots(robots, 1, bounds);
        if next != first {
            Some(next)
        } else {
            None
        }
    })
}

fn count_loops(robots: &Vec<Robot>, bounds: &(usize, usize)) -> usize {
    iterate_seconds(robots, bounds).count()
}
```

This is 10,403, so yes it's fine to check every frame. I later realise this has to be at most 101 &times; 103, which
is the same as I got from counting them.

Given that part 1 will be much lower for a frame where the picture is in one of the quadrants, I try taking the
minimum `total_safety_factor` and rendering the frame at that point. I also need to split out stepping the robots
and calculating the safety factor.

```rust
fn total_safety_factor(robots: &Vec<Robot>, bounds: &(usize, usize)) -> usize {
    let counts = robots
        .iter()
        .flat_map(|&robot| partition_position(robot.position, bounds))
        .counts();
    
    [TopLeft, TopRight, BottomLeft, BottomRight]
        .iter()
        .fold(1, |acc, quadrant| acc * counts.get(quadrant).unwrap_or(&0))
}

fn total_safety_factor_after_steps(
    robots: &Vec<Robot>,
    steps: usize,
    bounds: &(usize, usize),
) -> usize {
    let positions = simulate_robots(robots, steps, bounds);
    total_safety_factor(&positions, bounds)
}

fn guess_tree_second(robots: &Vec<Robot>, bounds: &(usize, usize)) -> usize {
    let (pos, _) = iterate_seconds(robots, bounds)
        .enumerate()
        .min_by_key(|(_, robots)| total_safety_factor(robots, bounds))
        .unwrap();
    
    render_robots(&frame, &bounds);
    
    pos
}

fn render_robots(robots: &Vec<Robot>, &(r_max, c_max): &(usize, usize)) {
    let positions: HashSet<Position> = robots.iter().map(|robot| robot.position).collect();
    
    for r in 0..r_max {
        for c in 0..c_max {
            if positions.contains(&(r, c)) {
                print!("#")
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
```

Which gives the following grid.

```ascii-art
   #
          #                                                                      #     #



                #                                  #                                #
                                                      #
               #

                                         #                                     #
                    #                              #
                                                   #                                     #   #
                                                                #
                                                          #
                                                                                        #        #



        #                                         #
                                     #                    #
                                   #                                                         #
                  #
                                           #
                                                    # ##                       #               #
                                          #
# #                    #  #  #                          #
         #                                      #                                      #
                                                                   #
                         #  ###############################               #               #
                            #                             #   #
                            #                             #
                            #                             #       #             #
                      #     #                             #                     #
                            #              #              #      #                    #
                            #             ###             #
 #                          #            #####            #                              #
                            #           #######           #               #
                            #          #########          #
                            #            #####            #                                   #
                            #           #######           #
             #              #          #########          #
                            #         ###########         #   #
                            #        #############        #                                         #
                            #          #########          #
                            #         ###########         # #
                            #        #############        #                         #
                            #       ###############       #
                     #      #      #################      #
                            #        #############        #
                            #       ###############       #      #
                        #   #      #################      #                            #
      #         #           #     ###################     #          #               #
                            #    #####################    #
                            #             ###             #
                            #             ###             #                                   #
                            #             ###             #                                    #    #
                            #                             #                      #
 #                          #                             #  #
                            #                             #
        #                   #                             #       #
                            ###############################                  #    #
                                                                             #

                                                            #            #

                                                              #
            #                  #

                       #
                                                  #              #
                                                   #                                #            #
                                                               #
                          #                                                #
   #                        #                                                                   #
                                                                            #
           #   #               #                  #                               #
                                                                                         #
                         #
                                                            #                 #
                                                                              #      #
                                             #
                                            #                     #
                                                                      #   #
        #                                                 #
                                #           #                                                     #
                                                                                        #
      #                   #
                                                            #
                                 #
                                                                             #
                                                                                             #
                 #

                                  #                                                     #

   #          #                     #
              #
        #                                                                #
                                                                                           #
                     #                             #                #                  #      #
                  #
                                        #                                              #
            #


```

## Wrap up

Today was a horrible puzzle that could have been great. The intro for part 2 was so vague and could have been asking
for so many different things, and there was nothing tangible to start forming a solution for.

Consider if instead there were a few more starting robots.

```text
p=5,4 v=-2,2
p=9,4 v=-1,-1
p=10,5 v=-1,2
p=4,4 v=2,-1
p=6,1 v=3,1
p=2,4 v=2,3
p=10,2 v=-1,-3
p=0,0 v=-1,-2
p=2,6 v=-3,2
p=10,6 v=-1,-1
p=10,1 v=-2,3
p=0,6 v=2,4
p=4,6 v=3,2
p=4,1 v=2,-1
p=4,2 v=-3,-2
p=5,0 v=-1,-2
p=7,0 v=-3,4
p=6,1 v=-2,2
```

Then for part 2, there was the following extra text:

> For example after 72 seconds on an 11 x 7 grid, the robots from part 1 look like this:
>
> ```text
> .#.......#.
> #...#......
> ...###.....
> #.#####...#  
> ....#......
> .....#..#..
> .....#.....
> ```
>
> You suspect with more robots and more space, the Easter egg becomes more elaborate.

I think that shows more concretely what is expected, without giving the game away too much. Then again, maybe
something like that was tried in play-testing and didn't work very well 🤷🏻.

```rust
fn tree_example_robots() -> Vec<Robot> {
    parse_input(
        &"p=5,4 v=-2,2
p=9,4 v=-1,-1
p=10,5 v=-1,2
p=4,4 v=2,-1
p=6,1 v=3,1
p=2,4 v=2,3
p=10,2 v=-1,-3
p=0,0 v=-1,-2
p=2,6 v=-3,2
p=10,6 v=-1,-1
p=10,1 v=-2,3
p=0,6 v=2,4
p=4,6 v=3,2
p=4,1 v=2,-1
p=4,2 v=-3,-2
p=5,0 v=-1,-2
p=7,0 v=-3,4
p=6,1 v=-2,2"
            .to_string(),
    )
}

#[test]
fn can_find_frame_with_lowest_safety_factor() {
    assert_eq!(guess_tree_seconds(&tree_example_robots(), &(7, 11)), 72);
}
```

In the end I enjoyed today despite initial frustrations at part 2. Possibly because I spent more time working out
sample input that approximated a tree for the second that had the lowest `total_safety_factor`.

I did notice when working that out that I had my `total_safety_factor` calculation wrong. The implementation above
ignores any empty quadrants, which should produce a `total_safety_factor` of 0. I updated it to fix this.

```rust
fn total_safety_factor(robots: &Vec<Robot>, bounds: &(usize, usize)) -> usize {
    let counts = robots
        .iter()
        .flat_map(|&robot| partition_position(robot.position, bounds))
        .counts();
    
    [TopLeft, TopRight, BottomLeft, BottomRight]
        .iter()
        .fold(1, |acc, quadrant| acc * counts.get(quadrant).unwrap_or(&0))
}

#[test]
fn can_calculate_total_safety_factor() {
    assert_eq!(
        total_safety_factor_after_steps(&example_robots(), 100, &(7, 11)),
        12
    );
    assert_eq!(
        total_safety_factor_after_steps(&example_robots(), 0, &(7, 11)),
        0
    )
}
```
