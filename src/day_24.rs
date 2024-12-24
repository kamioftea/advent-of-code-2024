//! This is my solution for [Advent of Code - Day 24: _Crossed Wires_](https://adventofcode.com/2024/day/24)
//!
//!

use crate::day_24::GateType::*;
use crate::day_24::Wire::*;
use std::collections::HashMap;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-24-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 24.
pub fn run() {
    let contents = fs::read_to_string("res/day-24-input.txt").expect("Failed to read file");
    let (input_wires, mut device) = parse_input(&contents);

    device.apply_input_wires(&input_wires);
    println!("The device outputs {}", device.output_value())
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Gate<'a> {
    gate_type: GateType,
    left_id: &'a str,
    left_value: bool,
    right_id: &'a str,
    right_value: bool,
    out_id: &'a str,
}

impl<'a> Gate<'a> {
    pub fn new(gate_type: GateType, left_id: &'a str, right_id: &'a str, out: &'a str) -> Self {
        Self {
            gate_type,
            left_id,
            left_value: false,
            right_id,
            right_value: false,
            out_id: out,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Wire {
    GateLeft(usize),
    GateRight(usize),
    Output(usize),
}

#[derive(Eq, PartialEq, Debug)]
struct MonitoringDevice<'a> {
    wires: HashMap<&'a str, Vec<Wire>>,
    gates: Vec<Gate<'a>>,
    outputs: Vec<bool>,
}

impl<'a> MonitoringDevice<'a> {
    fn update_gate(&mut self, gate_id: usize, is_right: bool, value: bool) {
        let gate = self.gates.get_mut(gate_id).unwrap();
        if is_right {
            gate.right_value = value
        } else {
            gate.left_value = value
        }

        let out_value = match gate.gate_type {
            And => gate.left_value & gate.right_value,
            Or => gate.left_value | gate.right_value,
            Xor => gate.left_value ^ gate.right_value,
        };

        let out_id = gate.out_id;

        self.apply_input(out_id, out_value)
    }

    fn apply_input(&mut self, wire_id: &'a str, value: bool) {
        for wire in self.wires.get(wire_id).unwrap().clone() {
            match wire {
                GateLeft(gate_id) => self.update_gate(gate_id, false, value),
                GateRight(gate_id) => self.update_gate(gate_id, true, value),
                Output(output_id) => *self.outputs.get_mut(output_id).unwrap() = value,
            }
        }
    }

    fn apply_input_wires(&mut self, wires: &HashMap<&'a str, bool>) {
        for (&wire, &value) in wires {
            self.apply_input(wire, value)
        }
    }

    fn output_value(&self) -> usize {
        self.outputs
            .iter()
            .enumerate()
            .map(|(id, &value)| (if value { 1usize } else { 0usize }) << id)
            .sum()
    }
}

fn parse_gate(line: &str) -> Gate {
    let mut parts = line.split(" ");

    let left_id = parts.next().unwrap();
    let gate_type = match parts.next() {
        Some("AND") => And,
        Some("OR") => Or,
        Some("XOR") => Xor,
        gate => unreachable!("unexpected logic gate {:?}", gate),
    };
    let right_id = parts.next().unwrap();
    parts.next();
    let out = parts.next().unwrap();

    Gate::new(gate_type, left_id, right_id, out)
}

fn parse_device(input: &str) -> MonitoringDevice {
    let mut wires: HashMap<&str, Vec<Wire>> = HashMap::new();
    let mut gates = Vec::new();
    let mut max_out = 0;

    for gate in input.lines().map(parse_gate) {
        let id = gates.len();

        wires.entry(gate.left_id).or_default().push(GateLeft(id));
        wires.entry(gate.right_id).or_default().push(GateRight(id));

        if gate.out_id.starts_with("z") {
            let out_id = gate.out_id.replace("z", "").parse().unwrap();

            wires.entry(gate.out_id).or_default().push(Output(out_id));
            max_out = max_out.max(out_id);
        }

        gates.push(gate);
    }

    MonitoringDevice {
        wires,
        gates,
        outputs: vec![false; max_out + 1],
    }
}

fn parse_input_wires(wires: &str) -> HashMap<&str, bool> {
    wires
        .lines()
        .map(|line| {
            let (id, value) = line.split_once(": ").unwrap();
            (id, value != "0")
        })
        .collect()
}

fn parse_input(input: &String) -> (HashMap<&str, bool>, MonitoringDevice) {
    let (input_wires, device) = input.split_once("\n\n").unwrap();

    (parse_input_wires(input_wires), parse_device(device))
}

#[cfg(test)]
mod tests {
    use crate::day_24::*;

    fn small_example_device() -> MonitoringDevice<'static> {
        MonitoringDevice {
            wires: vec![
                ("x00", vec![GateLeft(0)]),
                ("y00", vec![GateRight(0)]),
                ("x01", vec![GateLeft(1)]),
                ("y01", vec![GateRight(1)]),
                ("x02", vec![GateLeft(2)]),
                ("y02", vec![GateRight(2)]),
                ("z00", vec![Output(0)]),
                ("z01", vec![Output(1)]),
                ("z02", vec![Output(2)]),
            ]
            .into_iter()
            .collect(),
            gates: vec![
                Gate::new(And, "x00", "y00", "z00"),
                Gate::new(Xor, "x01", "y01", "z01"),
                Gate::new(Or, "x02", "y02", "z02"),
            ],
            outputs: vec![false; 3],
        }
    }

    fn small_example_inputs() -> HashMap<&'static str, bool> {
        vec![
            ("x00", true),
            ("x01", true),
            ("x02", true),
            ("y00", false),
            ("y01", true),
            ("y02", false),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn can_parse_input() {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"
        .to_string();

        let (inputs, device) = parse_input(&input);

        assert_eq!(inputs, small_example_inputs());
        assert_eq!(device, small_example_device());
    }

    #[test]
    fn can_apply_inputs() {
        let mut device = small_example_device();
        device.apply_input_wires(&small_example_inputs());

        assert_eq!(device.output_value(), 4)
    }

    #[test]
    fn can_apply_larger_example() {
        let example = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"
        .to_string();

        let (wires, mut device) = parse_input(&example);

        device.apply_input_wires(&wires);
        println!("{:?}", device.outputs);
        assert_eq!(device.output_value(), 2024);
    }
}
