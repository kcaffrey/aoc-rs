use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Registers([u32; 4]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Parameter {
    Register(u32),
    Value(u32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
    And,
    Or,
    Assign,
    Greater,
    Equals,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Instruction {
    opcode: u8,
    a: u32,
    b: u32,
    c: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct InterpretedInstruction {
    operation: Operation,
    a: Parameter,
    b: Parameter,
    c: Parameter,
}

trait Interpet {
    fn interpret(&self, inst: Instruction) -> InterpretedInstruction;
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Registers {
    fn execute(self, instruction: InterpretedInstruction) -> Self {
        let mut out = self;
        let (a, b, c) = (instruction.a, instruction.b, instruction.c);
        use self::Operation::*;
        match instruction.operation {
            Add => c.write(&mut out, a.read(&self) + b.read(&self)),
            Multiply => c.write(&mut out, a.read(&self) * b.read(&self)),
            And => c.write(&mut out, a.read(&self) & b.read(&self)),
            Or => c.write(&mut out, a.read(&self) | b.read(&self)),
            Assign => c.write(&mut out, a.read(&self)),
            Greater => c.write(&mut out, if a.read(&self) > b.read(&self) { 1 } else { 0 }),
            Equals => c.write(&mut out, if a.read(&self) == b.read(&self) { 1 } else { 0 }),
        }
        out
    }
}

impl Parameter {
    fn read(self, registers: &Registers) -> u32 {
        match self {
            Parameter::Register(reg) => registers.0[reg as usize],
            Parameter::Value(val) => val,
        }
    }

    fn write(self, registers: &mut Registers, val: u32) {
        match self {
            Parameter::Register(reg) => registers.0[reg as usize] = val,
            Parameter::Value(_) => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sample {
    before: Registers,
    instruction: Instruction,
    after: Registers,
}

struct Input {
    samples: Vec<Sample>,
    program: Vec<Instruction>,
}

impl Instruction {
    fn new(opcode: u8, a: u32, b: u32, c: u32) -> Self {
        Instruction { opcode, a, b, c }
    }
}

impl Interpet for Opcode {
    fn interpret(&self, inst: Instruction) -> InterpretedInstruction {
        let (a, b, c) = (inst.a, inst.b, inst.c);
        use self::{Opcode::*, Operation::*, Parameter::*};
        match self {
            Addr => InterpretedInstruction {
                operation: Add,
                a: Register(a),
                b: Register(b),
                c: Register(c),
            },
            Addi => InterpretedInstruction {
                operation: Add,
                a: Register(a),
                b: Value(b),
                c: Register(c),
            },
            Mulr => InterpretedInstruction {
                operation: Multiply,
                a: Register(a),
                b: Register(b),
                c: Register(c),
            },
            Muli => InterpretedInstruction {
                operation: Multiply,
                a: Register(a),
                b: Value(b),
                c: Register(c),
            },
            Banr => InterpretedInstruction {
                operation: And,
                a: Register(a),
                b: Register(b),
                c: Register(c),
            },
            Bani => InterpretedInstruction {
                operation: And,
                a: Register(a),
                b: Value(b),
                c: Register(c),
            },
            Borr => InterpretedInstruction {
                operation: Or,
                a: Register(a),
                b: Register(b),
                c: Register(c),
            },
            Bori => InterpretedInstruction {
                operation: Or,
                a: Register(a),
                b: Value(b),
                c: Register(c),
            },
            Setr => InterpretedInstruction {
                operation: Assign,
                a: Register(a),
                b: Register(b),
                c: Register(c),
            },
            Seti => InterpretedInstruction {
                operation: Assign,
                a: Value(a),
                b: Value(b),
                c: Register(c),
            },
            Gtir => InterpretedInstruction {
                operation: Greater,
                a: Value(a),
                b: Register(b),
                c: Register(c),
            },
            Gtri => InterpretedInstruction {
                operation: Greater,
                a: Register(a),
                b: Value(b),
                c: Register(c),
            },
            Gtrr => InterpretedInstruction {
                operation: Greater,
                a: Register(a),
                b: Register(b),
                c: Register(c),
            },
            Eqir => InterpretedInstruction {
                operation: Equals,
                a: Value(a),
                b: Register(b),
                c: Register(c),
            },
            Eqri => InterpretedInstruction {
                operation: Equals,
                a: Register(a),
                b: Value(b),
                c: Register(c),
            },
            Eqrr => InterpretedInstruction {
                operation: Equals,
                a: Register(a),
                b: Register(b),
                c: Register(c),
            },
        }
    }
}

impl Interpet for [Opcode; 16] {
    fn interpret(&self, inst: Instruction) -> InterpretedInstruction {
        self[inst.opcode as usize].interpret(inst)
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Box<Input> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(Before:\s*\[(\d+), (\d+), (\d+), (\d+)\]\s*(\d+) (\d+) (\d+) (\d+)\s*After:\s*\[(\d+), (\d+), (\d+), (\d+)\])|(\n(\d+) (\d+) (\d+) (\d+))")
            .unwrap();
    }
    let mut samples = vec![];
    let mut program = vec![];
    for caps in RE.captures_iter(input) {
        if caps.get(1).is_some() {
            // Sample
            samples.push(Sample {
                before: Registers([
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                    caps[4].parse().unwrap(),
                    caps[5].parse().unwrap(),
                ]),
                instruction: Instruction::new(
                    caps[6].parse().unwrap(),
                    caps[7].parse().unwrap(),
                    caps[8].parse().unwrap(),
                    caps[9].parse().unwrap(),
                ),
                after: Registers([
                    caps[10].parse().unwrap(),
                    caps[11].parse().unwrap(),
                    caps[12].parse().unwrap(),
                    caps[13].parse().unwrap(),
                ]),
            })
        } else {
            // Part of the program
            program.push(Instruction::new(
                caps[15].parse().unwrap(),
                caps[16].parse().unwrap(),
                caps[17].parse().unwrap(),
                caps[18].parse().unwrap(),
            ));
        }
    }
    Box::new(Input { samples, program })
}

fn all_opcodes() -> impl Iterator<Item = Opcode> {
    use self::Opcode::*;
    [
        Addi, Addr, Muli, Mulr, Bani, Banr, Bori, Borr, Seti, Setr, Gtir, Gtri, Gtrr, Eqir, Eqri,
        Eqrr,
    ]
    .iter()
    .cloned()
}

#[aoc(day16, part1)]
fn solve_part1(input: &Input) -> u32 {
    let mut count = 0;
    for sample in &input.samples {
        let mut valid_count = 0;
        for inst in all_opcodes().map(|op| op.interpret(sample.instruction)) {
            if sample.before.execute(inst) == sample.after {
                valid_count += 1;
            }
            if valid_count >= 3 {
                count += 1;
                break;
            }
        }
    }
    count
}

#[aoc(day16, part2)]
fn solve_part2(input: &Input) -> u32 {
    let mut possible_codes: HashMap<Opcode, HashSet<u8>> =
        all_opcodes().map(|op| (op, (0..16).collect())).collect();
    let mut broken = false;
    for sample in &input.samples {
        for op in all_opcodes() {
            let inst = op.interpret(sample.instruction);
            if sample.before.execute(inst) != sample.after {
                possible_codes
                    .get_mut(&op)
                    .unwrap()
                    .remove(&sample.instruction.opcode);
                if possible_codes[&op].len() == 1 {
                    broken = true;
                }
            }
        }
        if broken {
            break;
        }
    }
    let mut opcode_table = [Opcode::Addi; 16];
    let mut assigned = 0;
    while assigned < 16 {
        let known_codes: Vec<_> = possible_codes
            .iter()
            .filter_map(|(op, codes)| {
                if codes.len() == 1 {
                    Some((*op, *codes.iter().next().unwrap()))
                } else {
                    None
                }
            })
            .collect();
        for (op, code) in known_codes {
            assigned += 1;
            opcode_table[code as usize] = op;
            for remaining in possible_codes.values_mut() {
                remaining.remove(&code);
            }
        }
    }
    let mut registers = Registers([0, 0, 0, 0]);
    for inst in &input.program {
        registers = registers.execute(opcode_table.interpret(*inst));
    }
    registers.0[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "
Before: [2, 0, 2, 2]
7 3 2 0
After:  [0, 0, 2, 2]

Before: [3, 1, 1, 0]
5 2 1 2
After:  [3, 1, 2, 0]


2 2 3 3
2 0 3 2";
        let input = parse(input);
        assert_eq!(
            input.program,
            vec![Instruction::new(2, 2, 3, 3), Instruction::new(2, 0, 3, 2)]
        );
        assert_eq!(
            input.samples,
            vec![
                Sample {
                    before: Registers([2, 0, 2, 2]),
                    instruction: Instruction::new(7, 3, 2, 0),
                    after: Registers([0, 0, 2, 2])
                },
                Sample {
                    before: Registers([3, 1, 1, 0]),
                    instruction: Instruction::new(5, 2, 1, 2),
                    after: Registers([3, 1, 2, 0])
                }
            ]
        );
    }
}
