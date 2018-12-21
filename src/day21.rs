use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::str::FromStr;

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
enum Parameter {
    Register(usize),
    Value(usize),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Instruction {
    operation: Operation,
    a: Parameter,
    b: Parameter,
    c: Parameter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CPU {
    registers: [usize; 6],
    ip: usize,
    ip_reg: usize,
    program: Vec<Instruction>,
    break_on_eqrr: bool,
    seen: HashSet<usize>,
    last_target: usize,
}

impl CPU {
    pub fn tick(&mut self) -> Option<usize> {
        if self.ip >= self.program.len() {
            return Some(self.registers[0]);
        }
        self.registers[self.ip_reg] = self.ip;
        let inst = self.program[self.ip];

        // part 2 nonsense
        if let Some(divisor_sum) = self.divisor_sum_pattern_match(inst) {
            return Some(divisor_sum);
        }

        self.execute(inst);
        self.ip = self.registers[self.ip_reg];
        self.ip += 1;
        None
    }

    fn execute(&mut self, instruction: Instruction) {
        let (a, b, c) = (instruction.a, instruction.b, instruction.c);
        use self::Operation::*;
        let out_value = match instruction.operation {
            Add => a.read(self) + b.read(self),
            Multiply => a.read(&self) * b.read(self),
            And => a.read(self) & b.read(self),
            Or => a.read(self) | b.read(self),
            Assign => a.read(self),
            Greater => {
                if a.read(self) > b.read(self) {
                    1
                } else {
                    0
                }
            }
            Equals => {
                if a.read(self) == b.read(self) {
                    1
                } else {
                    0
                }
            }
        };
        c.write(self, out_value);
    }

    fn divisor_sum_pattern_match(&mut self, instruction: Instruction) -> Option<usize> {
        // There will only ever be 1 "eqrr" instruction - the larger register value will be the
        // number to factor.
        use self::{Operation::*, Parameter::*};
        if let Instruction {
            operation: Equals,
            a: Register(a),
            b: Register(b),
            ..
        } = instruction
        {
            let target = std::cmp::max(self.registers[a], self.registers[b]);
            if !self.seen.insert(target) {
                return Some(self.last_target);
            }
            self.last_target = target;
            if !self.break_on_eqrr {
                return None;
            }
            return Some(target);
        }
        None
    }
}

impl Parameter {
    fn read(self, cpu: &CPU) -> usize {
        match self {
            Parameter::Register(reg) => cpu.registers[reg],
            Parameter::Value(val) => val,
        }
    }

    fn write(self, cpu: &mut CPU, val: usize) {
        match self {
            Parameter::Register(reg) => cpu.registers[reg] = val,
            Parameter::Value(_) => unreachable!(),
        }
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Box<CPU> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#ip (\d+)$").unwrap();
    }
    let mut lines: VecDeque<_> = input.trim().lines().collect();
    let ip_reg = RE.captures(lines.pop_front().unwrap()).unwrap()[1]
        .parse()
        .unwrap();
    let program = lines
        .into_iter()
        .map(Instruction::from_str)
        .collect::<Result<Vec<Instruction>, Box<Error>>>()
        .unwrap();
    Box::new(CPU {
        registers: [0; 6],
        ip: 0,
        ip_reg,
        program,
        break_on_eqrr: false,
        seen: HashSet::new(),
        last_target: 0,
    })
}

#[aoc(day21, part1)]
fn solve_part1(cpu: &CPU) -> usize {
    let mut cpu = cpu.clone();
    cpu.break_on_eqrr = true;
    loop {
        if let Some(val) = cpu.tick() {
            return val;
        }
    }
}

#[aoc(day21, part2)]
fn solve_part2(cpu: &CPU) -> usize {
    // note: theres got to be a better way?
    // this takes forever.
    let mut cpu = cpu.clone();
    cpu.break_on_eqrr = false;
    loop {
        if let Some(val) = cpu.tick() {
            return val;
        }
    }
}

impl FromStr for Instruction {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Operation::*;
        use self::Parameter::*;
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s*(\w+)\s+(\d+)\s+(\d+)\s+(\d+)\s*$").unwrap();
        }
        let caps = RE
            .captures(s)
            .ok_or_else(|| Box::<Error>::from("invalid instruction"))?;
        let (a, b, c): (usize, usize, usize) =
            (caps[2].parse()?, caps[3].parse()?, caps[4].parse()?);
        macro_rules! instruction {
            ($op:ident, $a:ident, $b:ident) => {
                Instruction {
                    operation: $op,
                    a: $a(a),
                    b: $b(b),
                    c: Register(c),
                }
            };
        }
        Ok(match &caps[1] {
            "addr" => instruction!(Add, Register, Register),
            "addi" => instruction!(Add, Register, Value),
            "mulr" => instruction!(Multiply, Register, Register),
            "muli" => instruction!(Multiply, Register, Value),
            "banr" => instruction!(And, Register, Register),
            "bani" => instruction!(And, Register, Value),
            "borr" => instruction!(Or, Register, Register),
            "bori" => instruction!(Or, Register, Value),
            "setr" => instruction!(Assign, Register, Register),
            "seti" => instruction!(Assign, Value, Value),
            "gtir" => instruction!(Greater, Value, Register),
            "gtri" => instruction!(Greater, Register, Value),
            "gtrr" => instruction!(Greater, Register, Register),
            "eqir" => instruction!(Equals, Value, Register),
            "eqri" => instruction!(Equals, Register, Value),
            "eqrr" => instruction!(Equals, Register, Register),
            inst => return Err(format!("unknown instruction {}", inst).into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn test_parse() {
        use self::Operation::*;
        use self::Parameter::*;
        let cpu = parse(EXAMPLE);
        assert_eq!(cpu.ip, 0);
        assert_eq!(
            &cpu.program[0],
            &Instruction {
                operation: Assign,
                a: Value(5),
                b: Value(0),
                c: Register(1)
            }
        );
        assert_eq!(cpu.program.len(), 7);
    }

    #[test]
    fn test_part1() {
        assert_eq!(6, solve_part1(&parse(EXAMPLE)));
    }
}
