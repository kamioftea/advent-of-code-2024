---
day: 13
tags: [ post ]
header: 'Day 13: Claw Contraption'
---

Today there is some linear equations cunningly disguised as arcade machines. Warning may contain algebra.

## Parsing the input

Today is the first instance this year of more complex parsing of blocks of syntax. It seems best to isolate this
a line per coordinate, then the block for a machine, then the list of machines.

For the Coordinates, everything except the numbers can be ignored.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Coords {
    x: i64,
    y: i64,
}

impl FromStr for Coords {
    type Err = ();
    
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn parse_part(part: &str) -> i64 {
            part.chars()
                .flat_map(|c| c.to_digit(10))
                .fold(0, |acc, digit| 10 * acc + digit as i64)
        }
        
        if let Some((_, coords)) = line.split_once(": ") {
            if let Some((x_part, y_part)) = coords.split_once(", ") {
                return Ok(Coords {
                    x: parse_part(x_part),
                    y: parse_part(y_part),
                });
            }
        }
        
        Err(())
    }
}
```

For the machines, each line becomes the associated coordinate.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Machine {
    a: Coords,
    b: Coords,
    prize: Coords,
}

impl FromStr for Machine {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        
        let a = lines.next().and_then(|line| line.parse().ok());
        let b = lines.next().and_then(|line| line.parse().ok());
        let prize = lines.next().and_then(|line| line.parse().ok());
        
        if let (Some(a), Some(b), Some(prize)) = (a, b, prize) {
            Ok(Machine { a, b, prize })
        } else {
            Err(())
        }
    }
}
```

Finally, the input file can be broken into machines by splitting on blank lines.

```rust
fn parse_input(input: &String) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|block| block.parse().unwrap())
        .collect()
}

fn example_machines() -> Vec<Machine> {
    vec![
        Machine {
            a: Coords { x: 94, y: 34 },
            b: Coords { x: 22, y: 67 },
            prize: Coords { x: 8400, y: 5400 },
        },
        Machine {
            a: Coords { x: 26, y: 66 },
            b: Coords { x: 67, y: 21 },
            prize: Coords { x: 12748, y: 12176 },
        },
        Machine {
            a: Coords { x: 17, y: 86 },
            b: Coords { x: 84, y: 37 },
            prize: Coords { x: 7870, y: 6450 },
        },
        Machine {
            a: Coords { x: 69, y: 23 },
            b: Coords { x: 27, y: 71 },
            prize: Coords { x: 18641, y: 10279 },
        },
    ]
}

#[test]
fn can_parse_input() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"
        .to_string();
    
    assert_eq!(parse_input(&input), example_machines())
}
```

## Part 1 - Clawstrophobia

I need to find an A and B that cause the claw to land on the prize. Taking the x and y targets separately we get a
pair of linear equations in the form

$$n_AA_x + n_BB_x = P_x$$
$$n_AA_y + n_BB_y = P_y$$

Where:

* $n_A$, $n_B$ are the number of times button A and B are pressed respectively
* $A_x$, $A_y, B_x, B_y$ are the distance in x and y that the claw moves when you press A or B
* $P_x$, $P_y$ are the coordinates of the prize

These can then be rearranged to be in terms of $n_A$:

$$n_A = \frac{P_x - B_xn_B}{A_x}$$
$$n_A = \frac{P_y - B_yn_B}{A_y}$$

Substituting $n_A$ gives the following equality to solve

$$\frac{P_x - B_xn_B}{A_x} = \frac{P_y - B_yn_B}{A_y}$$

Rearranging in terms of $n_B$

$$\frac{B_xn_B}{A_x} - \frac{B_yn_B}{A_y} = \frac{P_x}{A_x} - \frac{P_y}{A_y}$$
$$A_yB_xn_B - A_xB_yn_B = P_xA_y - P_yA_x$$
$$n_B(A_yB_x - A_xB_y) = P_xA_y - P_yA_x$$
$$n_B = \frac{P_xA_y - P_yA_x}{A_yB_x - A_xB_y}$$

This can be plugged back in to either of the two equations in terms of $n_A$ to give both numbers.

Turning that into code

```rust
impl Machine {
    fn get_presses(&self, offset: i64) -> Option<(i64, i64)> {
        let Machine { a, b, prize } = self;
        
        let nb = (a.y * prize.x - a.x * prize.y) / (a.y * b.x - a.x * b.y);
        let na = (prize.x - b.x * nb) / a.x;
        
        // check that a and b have not been rounded
        if na * a.x + nb * b.x == prize.x && na * a.y + nb * b.y == prize.y {
            Some((na, nb))
        } else {
            None
        }
    }
}

#[test]
fn can_get_presses() {
    let machines = example_machines();
    
    assert_eq!(machines.get(0).unwrap().get_presses(), Some((80, 40)));
    assert_eq!(machines.get(1).unwrap().get_presses(), None);
    assert_eq!(machines.get(2).unwrap().get_presses(), Some((38, 86)));
    assert_eq!(machines.get(3).unwrap().get_presses(), None);
}
```

Note that because I'm doing integer division, to check if the lines meet at a valid point, I put the results back
into the machine's equations. If they don't give the correct answer, one or more was rounded and the lines don't
cross at an exact number of button presses. In that case there isn't a valid solution for that machine and I return
`None`

To get the puzzle solution, the cost for a valid machine is $3n_A + n_B$

```rust
impl Machine {
    fn get_cost_for_prize(&self) -> Option<i32> {
        self.get_presses().map(|(a, b)| 3 * a + b)
    }
}

fn sum_prize_costs(machines: &Vec<Machine>) -> i32 {
    machines.iter().flat_map(Machine::get_cost_for_prize).sum()
}

#[test]
fn can_sum_costs() {
    assert_eq!(sum_prize_costs(&example_machines()), 480)
}
```

## Part 2 - Off by 10<sup>13</sup> Error

Having already solved part 1 using linear equations, adding 10<sup>13</sup> to the prize target required adding an
offset to the calls, but otherwise was solving the same equations. I also needed to use `i64` instead of `i32`.

```rust
impl Machine {
    fn get_presses(&self, offset: i64) -> Option<(i64, i64)> {
        let Machine { a, b, prize } = self;
        
        let nb = (a.y * (prize.x + offset) - a.x * (prize.y + offset))
            / (a.y * b.x - a.x * b.y);
        let na = (prize.x + offset - b.x * nb) / a.x;
        
        // check that a and b have not been rounded
        if na * a.x + nb * b.x == prize.x + offset
            && na * a.y + nb * b.y == prize.y + offset
        {
            Some((na, nb))
        } else {
            None
        }
    }
    
    fn get_cost_for_prize(&self, offset: i64) -> Option<i64> {
        self.get_presses(offset).map(|(a, b)| 3 * a + b)
    }
}

#[test]
fn can_sum_costs() {
    assert_eq!(sum_prize_costs(&example_machines(), 0), 480);
    assert_eq!(
        sum_prize_costs(&example_machines(), 10_000_000_000_000),
        875318608908
    );
}
```

It's irritating that the two puzzles this year that are designed to take too long with an inefficient algorithm
don't provide example answers for test cases this year. In previous years they have, and it's a good sanity check to
have.

## Wrap up

I'm thankful that I spotted early on that this was solving the intersection of two linear equations. It made the
solution very math based, but meant that I had very little to do for part 2 that had to be implemented that way to
run quickly. It has been pointed out to me that this is a case of the more general
[Cramer's rule](https://en.wikipedia.org/wiki/Cramer%27s_rule) for solving systems of linear equations. But it was
interesting working through how to do algebra myself.
