//! This is my solution for [Advent of Code - Day 17: _Chronospatial Computer_](https://adventofcode.com/2024/day/17)
//!
//!

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-17-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 17.
pub fn run() {
    let contents = fs::read_to_string("res/day-17-input.txt").expect("Failed to read file");
    let computer = parse_input(&contents);

    println!(
        "The output of running the program is {}",
        computer.clone().run().iter().join(",")
    );

    println!(
        "The program is a quine when register A is {}",
        reverse_engineer_quine(&computer)
    );
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<u8>,
    instruction_pointer: usize,
}

impl Computer {
    fn with_register_a(&self, value: usize) -> Computer {
        Computer {
            register_a: value,
            ..self.clone()
        }
    }

    fn next_instruction(&self) -> Option<(u8, u8)> {
        self.program
            .get(self.instruction_pointer)
            .zip(self.program.get(self.instruction_pointer + 1))
            .map(|(&inst, &operand)| (inst, operand))
    }

    /// Combo operands 0 through 3 represent literal values 0 through 3.
    /// Combo operand 4 represents the value of register A.
    /// Combo operand 5 represents the value of register B.
    /// Combo operand 6 represents the value of register C.
    /// Combo operand 7 is reserved and will not appear in valid programs.
    fn combo(&self, operand: u8) -> usize {
        match operand {
            0 | 1 | 2 | 3 => operand as usize,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            op => unreachable!("Invalid combo operand {op}"),
        }
    }

    // The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is
    // found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2)
    // ; an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then
    // written to the A register
    fn adv(&mut self, operand: u8) {
        self.register_a = self.register_a / (2usize.pow(self.combo(operand) as u32));
        self.instruction_pointer += 2;
    }

    // The bxl instruction (opcode 1) calculates the bitwise XOR of register B  and the instruction's literal operand,
    // then stores the result in register B.
    fn bxl(&mut self, operand: u8) {
        self.register_b ^= operand as usize;
        self.instruction_pointer += 2;
    }

    // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest
    // 3 bits), then writes that value to the B register.
    fn bst(&mut self, operand: u8) {
        self.register_b = self.combo(operand) % 8;
        self.instruction_pointer += 2;
    }

    // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it
    // jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the
    // instruction pointer is not increased by 2 after this instruction.
    fn jnz(&mut self, operand: u8) {
        if self.register_a != 0 {
            self.instruction_pointer = operand as usize;
        } else {
            self.instruction_pointer += 2;
        }
    }

    // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in
    // register B. (For legacy reasons, this instruction reads an operand but ignores it.)
    fn bxc(&mut self, _: u8) {
        self.register_b ^= self.register_c;
        self.instruction_pointer += 2;
    }

    // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a
    // program outputs multiple values, they are separated by commas.)
    fn out(&mut self, operand: u8) -> u8 {
        self.instruction_pointer += 2;
        (self.combo(operand) % 8) as u8
    }

    // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B
    // register. (The numerator is still read from the A register.)
    fn bdv(&mut self, operand: u8) {
        self.register_b = self.register_a / (2usize.pow(self.combo(operand) as u32));
        self.instruction_pointer += 2;
    }

    // The cdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the C
    // register. (The numerator is still read from the A register.)
    fn cdv(&mut self, operand: u8) {
        self.register_c = self.register_a / (2usize.pow(self.combo(operand) as u32));
        self.instruction_pointer += 2;
    }

    fn run(&mut self) -> Vec<u8> {
        let mut output = Vec::new();

        while let Some((instruction, operand)) = self.next_instruction() {
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

#[allow(dead_code)]
fn brute_force_quine(computer: &Computer) -> usize {
    (0..)
        .map(|i| (i, computer.with_register_a(i).run()))
        .find(|(_, out)| *out == computer.program)
        .unwrap()
        .0
}

fn reverse_engineer_quine(computer: &Computer) -> usize {
    let mut partial_quines = vec![0];
    for &next_digit_to_match in computer.program.iter().rev() {
        let mut next_partial_quines = Vec::new();
        for &partial in partial_quines.iter() {
            let next_partial = partial * 8;
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

#[cfg(test)]
mod tests {
    use crate::day_17::*;

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

    #[test]
    fn can_run_instructions() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut sample_1 = Computer {
            register_a: 0,
            register_b: 0,
            register_c: 9,
            program: vec![2, 6],
            instruction_pointer: 0,
        };

        assert_eq!(sample_1.run(), Vec::<u8>::new());
        assert_eq!(sample_1.register_b, 1);

        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let mut sample_2 = Computer {
            register_a: 10,
            register_b: 0,
            register_c: 0,
            program: vec![5, 0, 5, 1, 5, 4],
            instruction_pointer: 0,
        };

        assert_eq!(sample_2.run(), vec![0, 1, 2]);

        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in
        // register A.
        let mut sample_3 = Computer {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            instruction_pointer: 0,
        };

        assert_eq!(sample_3.run(), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(sample_3.register_a, 0);
        //                                                                                                                      If register B contains 29, the program 1,7 would set register B to 26.
        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
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
}
