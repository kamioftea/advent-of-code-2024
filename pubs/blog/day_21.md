---
day: 21
tags: [ post ]
header: 'Day 21: Keypad Conundrum'
---

I made an absolute meal of today's puzzle. I had a hard time visualising the chain of effects, and so incorrectly
assumed that different ways of getting to the buttons didn't matter, but that was quickly disproven when my test for
`317A` failed. I couldn't get my head round the maths so ended up just trying every valid permutation of moves and
picking the best.

In refactoring to this I picked the wrong abstraction level and had a `KeyPad` trait implemented
by `DirectionalKeyPad`, and `NumericKeyPad`. This ended up with a lot of repeated code, and I eventually refactored
to have a `Keys` trait that just handled getting the coordinates of the keys. It also started off with using chars
for the keys to press, but this got confusing and that wasn't helping the type wrangling.

Defining enums for the keys allowed me to express what was actually happening in the types better, and with a bit of
refactoring I ended up with something cleaner. I'm going to present what I ended up with, and add notes about how it
got there rather than confusing the write-up with solutions that didn't work, and intermediate steps that didn't
compile.

## Parsing the input

I started with representing a code. Originally the code itself was a `Vec<char>` including the `A`. I eventually
ended up with enums for each set of input characters (`NumericButton` and `DirectionalButton`), and then a meta
`KeyPadButton` enum that expanded either list to include the `A` button common to both pads.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum DirectionalButton {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum NumericButton {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<char> for NumericButton {
    type Error = ();
    
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Zero),
            '1' => Ok(One),
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            '9' => Ok(Nine),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum KeyPadButton<T> {
    Input(T),
    A,
}
```

I ended storing the codes, excluding the `A`, because when interacting with the key pad, it was easiest to assume
everything ended with an `A` key, rather than add an `A` to the end of each move list.

```rust
#[derive(Eq, PartialEq, Debug)]
struct Code {
    buttons: Vec<NumericButton>,
    value: usize,
}

fn parse_code(code: &str) -> Code {
    let buttons = code.chars().flat_map(NumericButton::try_from).collect();
    let value = code
        .chars()
        .filter(|c| c.is_digit(10))
        .join("")
        .parse()
        .unwrap();
    
    Code { buttons, value }
}

fn parse_input(input: &String) -> Vec<Code> {
    input.lines().map(parse_code).collect()
}

fn example_codes() -> Vec<Code> {
    vec![
        Code {
            buttons: vec![Zero, Two, Nine],
            value: 29,
        },
        Code {
            buttons: vec![Nine, Eight, Zero],
            value: 980,
        },
        Code {
            buttons: vec![One, Seven, Nine],
            value: 179,
        },
        Code {
            buttons: vec![Four, Five, Six],
            value: 456,
        },
        Code {
            buttons: vec![Three, Seven, Nine],
            value: 379,
        },
    ]
}

#[test]
fn can_parse_input() {
    let input = "029A
980A
179A
456A
379A
"
        .to_string();
    
    assert_eq!(parse_input(&input), example_codes());
}
```

## Part 1 - The keypad's connected to the ro-bot...

As mentioned I first tried to implement this by ordering the inputs so that the robot never walked outside the key pad.
With that solution I was able to pass the output of one keypad into the next. This breaks down because moving
rightwards is more expensive than other directions. So a path that moves right twice in succession, then splits
moving back leftwards with some presses of A works better than the other way round.

I couldn't get a good enough understanding of what caused one or the other, and after some head-scratching reverted
to trying each possible order of the horizontal and vertical movements, and discarding any that walked on the
missing key.

Firstly I need a way to map keys to a position on the keypad, and workout whether a coordinate is on the keypad.
This is where the difference between the two keypads is, everything else can be implemented generically. After a few
refactorings I ended up with a `Keys<T>` trait implemented for each input button enum.

```rust
type Coordinates = (u8, u8);

trait Keys<T> {
    fn coordinate(key: KeyPadButton<T>) -> Coordinates;
    fn contains(coord: &Coordinates) -> bool;
}

impl Keys<NumericButton> for NumericButton {
    fn coordinate(key: KeyPadButton<NumericButton>) -> Coordinates {
        match key {
            Input(Zero) => (3, 1),
            Input(One) => (2, 0),
            Input(Two) => (2, 1),
            Input(Three) => (2, 2),
            Input(Four) => (1, 0),
            Input(Five) => (1, 1),
            Input(Six) => (1, 2),
            Input(Seven) => (0, 0),
            Input(Eight) => (0, 1),
            Input(Nine) => (0, 2),
            A => (3, 2),
        }
    }
    
    fn contains(coord: &Coordinates) -> bool {
        match coord {
            &(3, 0) => false,
            &(r, c) if r <= 3 && c <= 2 => true,
            _ => false,
        }
    }
}

impl Keys<DirectionalButton> for DirectionalButton {
    fn coordinate(key: KeyPadButton<DirectionalButton>) -> Coordinates {
        match key {
            Input(Up) => (0, 1),
            Input(Right) => (1, 2),
            Input(Down) => (1, 1),
            Input(Left) => (1, 0),
            A => (0, 2),
        }
    }
    
    fn contains(coord: &Coordinates) -> bool {
        match coord {
            &(0, 0) => false,
            &(r, c) if r <= 1 && c <= 2 => true,
            _ => false,
        }
    }
}
```

I also need to be able to move around the keypad

```rust
trait CoordinateExtensions: Sized {
    fn apply_move(&self, mv: &DirectionalButton) -> Option<Self>;
}

impl CoordinateExtensions for Coordinates {
    fn apply_move(&self, mv: &DirectionalButton) -> Option<Self> {
        let (r, c) = self;
        let (dr, dc) = match mv {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        };
        
        let r1 = r.checked_add_signed(dr);
        let c1 = c.checked_add_signed(dc);
        
        r1.zip(c1)
    }
}
```

Next I need to represent a key pad. It needs to hold a reference to the directional keypad that controls this pad's
robot all the way up the chain to the final pad controlled by someone actually pressing the keys. This needs to be
wrapped in an `Rc` so it can be cheaply cloned when passing the needed key presses to it.

The type also needs to be specific to one of the input key enums, but that doesn't show up in the controller property.
The [`PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html#unused-type-parameters) type exists to
solve that problem. I can use this to build chains of keypads.

```rust
struct KeyPad<T> {
    controller: Option<Rc<KeyPad<DirectionalButton>>>,
    key_type: PhantomData<T>,
}

impl<T> KeyPad<T>
where
    T: Keys<T> + Copy + Clone + Eq + Hash,
{
    fn direct_entry() -> KeyPad<T> {
        KeyPad::<T> {
            controller: None::<Rc<KeyPad<DirectionalButton>>>,
            key_type: PhantomData,
        }
    }
    
    fn controlled_by(controller: KeyPad<DirectionalButton>) -> KeyPad<T> {
        KeyPad::<T> {
            controller: Some(Rc::new(controller)),
            key_type: PhantomData,
        }
    }
}

fn keypad_chain() -> KeyPad<NumericButton> {
    let human_pad = KeyPad::direct_entry();
    let robot_direction = KeyPad::controlled_by(human_pad);
    KeyPad::controlled_by(robot_direction)
}
```

That setup, I'm able to recursively calculate the number of presses on the human key pad, by working out the list of
moves needed to input the codes, and passing those up to the next keypad. button presses at the base case is one
press for each move, plus 1 for pressing A to trigger the push. Building the key presses is further broken down as
follows.

* The moves can be generated for each pair of buttons that need to be presses, inserting A as the start and finish
  positions.
* Each pair has a set amount of vertical and horizontal movement, those can be interleaved in any order, but some
  routes won't be valid if they cross the missing key.
* Try each of the permutations, on the controlling keypad and pick the best.

```rust
impl<T> KeyPad<T>
where
    T: Keys<T> + Copy + Clone + Eq + Hash,
{
    fn repeat(
        positive: DirectionalButton,
        negative: DirectionalButton,
        a: u8,
        b: u8,
    ) -> Vec<DirectionalButton> {
        let char = if a < b { positive } else { negative };
        [char].repeat(a.abs_diff(b) as usize)
    }
    
    fn check_moves(moves: &Vec<&DirectionalButton>, start: &Coordinates) -> bool {
        let mut position = start.clone();
        for &mv in moves {
            match position.apply_move(mv) {
                Some(new_pos) => {
                    if !T::contains(&new_pos) {
                        return false;
                    }
                    position = new_pos
                }
                None => return false,
            }
        }
        
        true
    }
    
    fn controller_presses(&self, moves: Vec<&DirectionalButton>) -> usize {
        match self.controller.clone() {
            Some(keypad) => {
                let buttons = moves.into_iter().cloned().collect();
                keypad.key_presses(&buttons)
            }
            None => moves.len() + 1, // and A,
        }
    }
    
    fn presses_for_pair(&self, (a, b): (KeyPadButton<T>, KeyPadButton<T>)) -> usize {
        let (ra, ca) = T::coordinate(a);
        let (rb, cb) = T::coordinate(b);
        
        let moves: Vec<DirectionalButton> = chain(
            Self::repeat(Down, Up, ra, rb),
            Self::repeat(Right, Left, ca, cb),
        )
            .collect();
        
        moves
            .iter()
            .permutations(moves.len())
            .filter(|moves| Self::check_moves(moves, &(ra, ca)))
            .map(|moves| self.controller_presses(moves))
            .min()
            .expect("Failed to find safe route {a} -> {b}")
    }
    
    fn key_presses(&self, keys: &Vec<T>) -> usize {
        once(A)
            .chain(keys.iter().map(|&key| Input(key)))
            .chain(once(A))
            .tuple_windows()
            .map(|pair| self.presses_for_pair(pair))
            .sum()
    }
}

#[test]
fn can_count_key_presses() {
    let key_pad = keypad_chain();
    
    assert_eq!(key_pad.key_presses(&example_codes()[0].buttons), 68);
    assert_eq!(key_pad.key_presses(&example_codes()[1].buttons), 60);
    assert_eq!(key_pad.key_presses(&example_codes()[2].buttons), 68);
    assert_eq!(key_pad.key_presses(&example_codes()[3].buttons), 64);
    assert_eq!(key_pad.key_presses(&example_codes()[4].buttons), 64);
}
```

That in place, I now need to find the number of presses for each of the codes, turn that into a complexity rating,
and sum.

```rust
fn sum_complexities(codes: &Vec<Code>, door: &mut KeyPad<NumericButton>) -> usize {
    codes
        .iter()
        .map(|code| door.key_presses(&code.buttons) * code.value)
        .sum()
}

#[test]
fn can_sum_complexities() {
    assert_eq!(
        sum_complexities(&example_codes(), &mut keypad_chain()),
        126384
    )
}
```

## Part 2 - Recursive robots

The challenge for part two is to scale that for a chain of 25 robots. Setting up the longer chain can be done by
adding a length to the chain builder. There was an off-by-one error that caught me out here.

```rust
fn keypad_chain(length: usize) -> KeyPad<NumericButton> {
    let chain = (1..length).fold(KeyPad::direct_entry(), |prev, _| {
        KeyPad::controlled_by(prev)
    });
    KeyPad::controlled_by(chain)
}
```

The current setup isn't going to run with that chain without some caching. The presses to move between a given
pair of keys is going to be constant for that keypad, so I can add a cache of the cost for each pair in
`KeyPad::presses_for_pair`. The biggest change here is that `KeyPad` needs to be mutable for that to work, and that
means wrapping the reference to the controller in a RefCell. The cache handily includes the input key type, so I can
remove the `PhantomData`

```diff
  struct KeyPad<T> {
      controller: Option<Rc<RefCell<KeyPad<DirectionalButton>>>>,
-     key_type: PhantomData<T>,
+     cache: HashMap<(KeyPadButton<T>, KeyPadButton<T>), usize>,
  }
  impl<T> KeyPad<T>
  where
      T: Keys<T> + Copy + Clone + Eq + Hash,
  {
      fn direct_entry() -> KeyPad<T> {
          KeyPad::<T> {
-             controller: None::<Rc<KeyPad<DirectionalButton>>>,
-             key_type: PhantomData,
+             controller: None::<Rc<RefCell<KeyPad<DirectionalButton>>>>,
+             cache: HashMap::new(),
          }
      }
      
      fn controlled_by(controller: KeyPad<DirectionalButton>) -> KeyPad<T> {
          KeyPad::<T> {
-             controller: Some(Rc::new(controller)),
-             key_type: PhantomData,
+             controller: Some(Rc::new(RefCell::new(controller))),
+             cache: HashMap::new(),
          }
      }
      
      fn check_moves(moves: &Vec<&DirectionalButton>, start: &Coordinates) -> bool {
          let mut position = start.clone();
          for &mv in moves {
              match position.apply_move(mv) {
                  Some(new_pos) => {
                      if !T::contains(&new_pos) {
                          return false;
                      }
                      position = new_pos
                  }
                  None => return false,
              }
          }
          
          true
      }
      
-     fn controller_presses(&self, moves: Vec<&DirectionalButton>) -> usize {
+     fn controller_presses(&mut self, moves: Vec<&DirectionalButton>) -> usize {
          match self.controller.clone() {
              Some(keypad) => {
                  let buttons = moves.into_iter().cloned().collect();
                  keypad.key_presses(&buttons)
                  keypad.borrow_mut().key_presses(&buttons)
              }
              None => moves.len() + 1, // and A,
          }
      }
      
-     fn presses_for_pair(&self, (a, b): (KeyPadButton<T>, KeyPadButton<T>)) -> usize {
+     fn presses_for_pair(&mut self, (a, b): (KeyPadButton<T>, KeyPadButton<T>)) -> usize {
+         if let Some(&result) = self.cache.get(&(a, b)) {
+           return result;
+         }
        
          let (ra, ca) = T::coordinate(a);
          let (rb, cb) = T::coordinate(b);
          
          let moves: Vec<DirectionalButton> = chain(
              Self::repeat(Down, Up, ra, rb),
              Self::repeat(Right, Left, ca, cb),
          )
              .collect();
          
-         moves
+         let count = moves
              .iter()
              .permutations(moves.len())
              .filter(|moves| Self::check_moves(moves, &(ra, ca)))
              .map(|moves| self.controller_presses(moves))
              .min()
-             .expect("Failed to find safe route {a} -> {b}")
-             .expect("Failed to find safe route {a} -> {b}");

+         self.cache.insert((a, b), count);
+
+         count
      }
      
-     fn key_presses(&self, keys: &Vec<T>) -> usize {
+     fn key_presses(&mut self, keys: &Vec<T>) -> usize {
          once(A)
              .chain(keys.iter().map(|&key| Input(key)))`
              .chain(once(A))
              .tuple_windows()
              .map(|pair| self.presses_for_pair(pair))
              .sum()
      }
  }
```

There's no additional test case for part 2, but the original tests are still passing, and this solves both parts of
the puzzle, calculating ~250 trillion key presses in ~1ms.

## Wrap up

I found today really hard to hold in my head, and that lead to some quite messy code. As much as I spent a lot of
time wrnagling with the compiler, the error messages and guidance from it was really useful. Whilst it took far too
long to get there, I'm happy with the final code. Added bonus that the difference between the two keypad's buttons
is encoded in the type system which helped me organise and express my solution much better.
