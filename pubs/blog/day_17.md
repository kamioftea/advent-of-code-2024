---
day: 17
tags: [ post ]
header: 'Day 17: Chronospatial Computer'
---

Another advent of code classic, define a limited instruction set and require implementing an interpreter.

## Parsing the input

Today's puzzle input is minimal, which is always worrying. I thought about parsing the pairs of operators and operands
into an enum of instructions, but wasn't 100% sure the program counter would never move an odd number and invalidate
those pairs.

```rust
fn parse_register(line: &str) -> usize {
    let (_, num) = line.split_once(": ").unwrap();
    num.parse().unwrap()
}

fn parse_program(line: &str) -> Vec<u8> {
    let (_, program) = line.split_once(": ").unwrap();
    program
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect()
}

fn parse_input(input: &String) -> Computer {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut register_iter = registers.lines();
    
    Computer {
        register_a: parse_register(register_iter.next().unwrap()),
        register_b: parse_register(register_iter.next().unwrap()),
        register_c: parse_register(register_iter.next().unwrap()),
        program: parse_program(program),
        instruction_pointer: 0,
    }
}

fn example_computer() -> Computer {
    Computer {
        register_a: 729,
        register_b: 0,
        register_c: 0,
        program: vec![0, 1, 5, 4, 3, 0],
        instruction_pointer: 0,
    }
}

#[test]
fn can_parse_input() {
    let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
        .to_string();
    
    assert_eq!(parse_input(&input), example_computer());
}
```

## Part 1 - Assembling an interpreter

The registers and the instruction pointer updates will be more intuitive if the program runs within a mutable
computer. I implement the processing loop with the various op codes pointing to stub functions.

```rust
impl Computer {
    fn next_instruction(&self) -> Option<(u8, u8)> {
        self.program
            .get(self.instruction_pointer)
            .zip(self.program.get(self.instruction_pointer + 1))
            .map(|(&inst, &operand)| (inst, operand))
    }
    
    fn run(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        
        while let Some((instruction, operand)) = self.next_instruction() {
            self.instruction_pointer += 2;
            
            match instruction {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => output.push(self.out(operand)),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                op => unreachable!("Invalid op code: {op}"),
            }
        }
        
        output
    }
}
```

The samples provided allow me to work through implementing each of those stubs as the next example fails to run.

```rust
impl Computer {
    fn deref_combo(&self, operand: u8) -> usize {
        match operand {
            0 | 1 | 2 | 3 => operand as usize,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            op => unreachable!("Invalid combo operand {op}"),
        }
    }
    
    fn adv(&mut self, operand: u8) {
        self.register_a = self.register_a / (2usize.pow(self.deref_combo(operand) as u32));
    }
    
    fn bxl(&mut self, operand: u8) {
        self.register_b ^= operand as usize;
    }
    
    fn bst(&mut self, operand: u8) {
        self.register_b = self.deref_combo(operand) % 8;
    }
    
    fn jnz(&mut self, operand: u8) {
        if self.register_a != 0 {
            self.instruction_pointer = operand as usize;
        }
    }
    
    fn bxc(&mut self, _: u8) {
        self.register_b ^= self.register_c;
    }
    
    fn out(&mut self, operand: u8) -> u8 {
        (self.deref_combo(operand) % 8) as u8
    }
    
    fn bdv(&mut self, operand: u8) {
        self.register_b = self.register_a / (2usize.pow(self.deref_combo(operand) as u32));
    }
    
    fn cdv(&mut self, operand: u8) {
        self.register_c = self.register_a / (2usize.pow(self.deref_combo(operand) as u32));
    }
}

#[test]
fn can_run_instructions() {
    let mut sample_1 = Computer {
        register_a: 0,
        register_b: 0,
        register_c: 9,
        program: vec![2, 6],
        instruction_pointer: 0,
    };
    
    assert_eq!(sample_1.run(), Vec::<u8>::new());
    assert_eq!(sample_1.register_b, 1);
    
    
    let mut sample_2 = Computer {
        register_a: 10,
        register_b: 0,
        register_c: 0,
        program: vec![5, 0, 5, 1, 5, 4],
        instruction_pointer: 0,
    };
    
    assert_eq!(sample_2.run(), vec![0, 1, 2]);
    
    
    let mut sample_3 = Computer {
        register_a: 2024,
        register_b: 0,
        register_c: 0,
        program: vec![0, 1, 5, 4, 3, 0],
        instruction_pointer: 0,
    };
    
    assert_eq!(sample_3.run(), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    assert_eq!(sample_3.register_a, 0);
    
    
    let mut sample_4 = Computer {
        register_a: 0,
        register_b: 2024,
        register_c: 43690,
        program: vec![4, 0],
        instruction_pointer: 0,
    };
    
    assert_eq!(sample_4.run(), Vec::<u8>::new());
    assert_eq!(sample_4.register_b, 44354);
    
    let mut example_computer = example_computer();
    assert_eq!(example_computer.run(), vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
}
```

I need to join the final output into a comma separated string, but that is all that is needed to solve part 1.

## Part 2 - Interpreting the quine

Part two is finding the seed number in register A that will cause the program to emit itself (i.e. a
[Quine](https://en.wikipedia.org/wiki/Quine_(computing)))

I do try the brute force method and set it running just in case it finishes before I find a better way.

```rust
fn find_quine(computer: &Computer) -> usize {
    (0..)
        .map(|i| (i, computer.with_register_a(i).run()))
        .find(|(_, out)| *out == computer.program)
        .unwrap()
        .0
}

#[test]
fn can_find_quine() {
    let sample = Computer {
        register_a: 2024,
        register_b: 0,
        register_c: 0,
        program: vec![0, 3, 5, 4, 3, 0],
        instruction_pointer: 0,
    };
    
    assert_eq!(find_quine(&sample), 117440);
}
```

Whilst that is running, I have a look at my actual input and work out what it is doing

```text
2,4  bst,A,  B = A & 7        Read next digit into B
1,5  bxl,B,  B = B xor 5   
7,5  cdv,B,  C = A / 2^B      C depends on A and B
0,3  adv,3,  A = A / 2^3      Shift current digit out of A
4,1  bxc,_,  B = B xor C      B depends on C
1,6  bxl,6,  B = B xor 6   
5,5  out,B,  Output B        
3,0  jnz,0   Goto 0 if A > 0  
```

The key things here are that it is essentially shifting the least significant 3-bit number off A into B, XORing that
with some constants, but also C which in turn is read from the value of A before shifting. This means that
calculating the quine is slightly self-referential. For the final digit of the quine, A is only the value that
produces that digit, so I can find the single digit value(s) of A that produce a final 0, and work back from there.

Having done that I can keep working back until I find a number that produces itself. The sample quine is a simpler
version of that where it shifts the least-significant digit off A, prints the least significant digit of A and loops
until `a` is 0, so this code works for both the sample quine and my puzzle input.

```rust
fn reverse_engineer_quine(computer: &Computer) -> usize {
    let mut partial_quines = vec![0];
    for &next_digit_to_match in computer.program.iter().rev() {
        let mut next_partial_quines = Vec::new();
        for &partial in partial_quines.iter() {
            let mut next_partial = partial * 8;
            for digit in 0..8 {
                let register_a = next_partial + digit;
                let program_output = computer.with_register_a(register_a).run();
                
                if program_output.first() == Some(&next_digit_to_match) {
                    next_partial_quines.push(register_a);
                }
            }
        }
        
        partial_quines = next_partial_quines;
    }
    
    partial_quines.first().unwrap().clone()
}

#[test]
fn can_find_quine() {
    let sample = Computer {
        register_a: 2024,
        register_b: 0,
        register_c: 0,
        program: vec![0, 3, 5, 4, 3, 0],
        instruction_pointer: 0,
    };
    
    assert_eq!(brute_force_quine(&sample), 117440);
    assert_eq!(reverse_engineer_quine(&sample), 117440);
}
```

## Wrap up

Today was a satisfying puzzle. I enjoyed having to analyse what the puzzle input was actually doing, and it was good
that a simpler sample that has similar properties was provided. 
