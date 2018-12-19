use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
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
    Register(u32),
    Value(u32),
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
    registers: [u32; 6],
    ip: usize,
    ip_reg: usize,
    program: Vec<Instruction>,
    break_on_eqrr: bool,
}

impl CPU {
    pub fn tick(&mut self) -> Option<u32> {
        if self.ip >= self.program.len() {
            return Some(self.registers[0]);
        }
        self.registers[self.ip_reg] = self.ip as u32;
        let inst = self.program[self.ip];

        // part 2 nonsense
        if let Some(factor_sum) = self.part2_pattern_match(inst) {
            return Some(factor_sum);
        }

        self.execute(inst);
        self.ip = self.registers[self.ip_reg] as usize;
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

    fn part2_pattern_match(&self, instruction: Instruction) -> Option<u32> {
        if !self.break_on_eqrr {
            return None;
        }

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
            let mut sum = 0u32;
            let num = std::cmp::max(self.registers[a as usize], self.registers[b as usize]);
            let mut i = 1;
            while i * i <= num {
                if num % i == 0 {
                    sum += i;
                    sum += num / i;
                }
                i += 1;
            }
            return Some(sum);
        }
        None
    }
}

impl Parameter {
    fn read(self, cpu: &CPU) -> u32 {
        match self {
            Parameter::Register(reg) => cpu.registers[reg as usize],
            Parameter::Value(val) => val,
        }
    }

    fn write(self, cpu: &mut CPU, val: u32) {
        match self {
            Parameter::Register(reg) => cpu.registers[reg as usize] = val,
            Parameter::Value(_) => unreachable!(),
        }
    }
}

#[aoc_generator(day19)]
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
    })
}

#[aoc(day19, part1)]
fn solve_part1(cpu: &CPU) -> u32 {
    let mut cpu = cpu.clone();
    loop {
        if let Some(val) = cpu.tick() {
            return val;
        }
    }
}

#[aoc(day19, part2)]
fn solve_part2(cpu: &CPU) -> u32 {
    let mut cpu = cpu.clone();
    cpu.registers[0] = 1;
    cpu.break_on_eqrr = true;
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
        let (a, b, c): (u32, u32, u32) = (caps[2].parse()?, caps[3].parse()?, caps[4].parse()?);
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
