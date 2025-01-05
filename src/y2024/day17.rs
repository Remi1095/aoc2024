use crate::{get_text_file, SolutionResult};
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/17/input";

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from_opcode(opcode: u64) -> Option<Self> {
        Some(match opcode {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => None?,
        })
    }
    fn display(&self) -> String {
        match self {
            Instruction::Adv => "adv",
            Instruction::Bxl => "bxl",
            Instruction::Bst => "bxt",
            Instruction::Jnz => "jnz",
            Instruction::Bxc => "bxc",
            Instruction::Out => "out",
            Instruction::Bdv => "bdv",
            Instruction::Cdv => "cdv",
        }
        .to_string()
    }
}

enum Register {
    A,
    B,
    C,
}

enum Operand {
    Literal(u64),
    Variable(Register),
}

impl Operand {
    fn unwrap_literal(self) -> u64 {
        match self {
            Operand::Literal(val_) => val_,
            Operand::Variable(_) => panic!("unwrap_literal on Operand::Variable"),
        }
    }

    fn register_a() -> Self {
        Operand::Variable(Register::A)
    }

    fn register_b() -> Self {
        Operand::Variable(Register::B)
    }

    fn register_c() -> Self {
        Operand::Variable(Register::C)
    }
}

enum Operation {
    BXorB(Operand),
    BMaskLast3(Operand),
    RightShiftA { assign: Register, value: Operand },
    OutputMaskLast3(Operand),
    AJump,
}

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (program, mut register_a, mut register_b, mut register_c) = read_input(file);
    let instructions = program
        .clone()
        .into_iter()
        .map(|opcode| Instruction::from_opcode(opcode).unwrap())
        .collect_vec();

    // println!("Initial");
    // println!(
    //     "Registers: A {} B {} C {}",
    //     register_a, register_b, register_c
    // );

    let mut outputs = Vec::new();
    let mut pointer = 0;
    while pointer + 1 < program.len() {
        let instruction = &instructions[pointer];
        let operand = program[pointer + 1];
        let combo = match operand {
            0..=3 => operand,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            _ => panic!(),
        };
        let mut increment = true;
        match instruction {
            Instruction::Adv => register_a = register_a >> combo,
            Instruction::Bxl => register_b = register_b ^ operand,
            Instruction::Bst => register_b = combo & 0b111,
            Instruction::Jnz => {
                if register_a != 0 {
                    pointer = operand as usize;
                    increment = false;
                }
            }
            Instruction::Bxc => register_b = register_b ^ register_c,
            Instruction::Out => outputs.push(combo & 0b111),
            Instruction::Bdv => register_b = register_a >> combo,
            Instruction::Cdv => register_c = register_a >> combo,
        }
        // println!("\nPointer {}", pointer);
        // println!("Instruction {}", instruction.display());
        // println!("Operands: literal {}, combo {}", operand, combo);
        // println!(
        //     "Registers: A {}, B {}, C {}",
        //     register_a, register_b, register_c
        // );
        // println!("outputs {:?}", outputs);

        if increment {
            pointer += 2
        }
    }

    Ok(outputs.into_iter().join(","))
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (program, _, register_b, register_c) = read_input(file);

    let mut operations: VecDeque<Operation> = VecDeque::new();

    let mut pointer = 0;
    let mut outputs_len = 0;
    while outputs_len <= program.len() {
        let opcode = program[pointer];
        let instruction = Instruction::from_opcode(opcode).unwrap();

        let operand = program[pointer + 1];
        let literal = Operand::Literal(operand);
        let combo = match operand {
            0..=3 => Operand::Literal(operand),
            4 => Operand::Variable(Register::A),
            5 => Operand::Variable(Register::B),
            6 => Operand::Variable(Register::C),
            _ => panic!(),
        };

        let mut increment = true;
        operations.push_front(match instruction {
            // Instruction::Adv => register_a = register_a >> combo,
            Instruction::Adv => Operation::RightShiftA {
                assign: Register::A,
                value: combo,
            },
            // Instruction::Bxl => register_b = register_b ^ operand,
            Instruction::Bxl => Operation::BXorB(literal),
            // Instruction::Bst => register_b = combo & 0b111,
            Instruction::Bst => Operation::BMaskLast3(combo),
            Instruction::Jnz => {
                pointer = operand as usize;
                increment = false;
                Operation::AJump
            }
            // Instruction::Bxc => register_b = register_b ^ register_c,
            Instruction::Bxc => Operation::BXorB(Operand::register_c()),
            Instruction::Out => {
                outputs_len += 1;
                Operation::OutputMaskLast3(combo)
            }
            // Instruction::Bdv => register_b = register_a >> combo,
            Instruction::Bdv => Operation::RightShiftA {
                assign: Register::B,
                value: combo,
            },
            // Instruction::Cdv => register_c = register_a >> combo,
            Instruction::Cdv => Operation::RightShiftA {
                assign: Register::C,
                value: combo,
            },
        });
        if increment {
            pointer += 2
        }
    }
    let last_jump = operations.iter().position(|op| matches!(op, Operation::AJump)).unwrap();
    operations = operations.into_iter().skip(last_jump+1).collect();
    let possible_register_a = vec![0];
    for operation in operations {
        match operation {
            // B = B ^ operand
            Operation::BXorB(operand) => todo!(),
            // B = operand & 0b111
            Operation::BMaskLast3(operand) => todo!(),
            // X = A >> operand,
            Operation::RightShiftA { assign, value } => todo!(),
            // output: operand & 0b111,
            Operation::OutputMaskLast3(operand) => todo!(),
            // if A != 0: jump,
            Operation::AJump => todo!(),
        }
    }

    Err("Failed to solve".into())
}

fn read_input(file: File) -> (Vec<u64>, u64, u64, u64) {
    let register_a_regex = Regex::new(r"Register A: (\d+)").unwrap();
    let register_b_regex = Regex::new(r"Register B: (\d+)").unwrap();
    let register_c_regex = Regex::new(r"Register C: (\d+)").unwrap();
    let program_regex = Regex::new(r"Program: ((\d+,)*\d+)").unwrap();
    let mut lines = BufReader::new(file).lines().filter_map(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            None
        } else {
            Some(line)
        }
    });
    let register_a = parse_match(&register_a_regex, &lines.next().unwrap()).unwrap();
    let register_b = parse_match(&register_b_regex, &lines.next().unwrap()).unwrap();
    let register_c = parse_match(&register_c_regex, &lines.next().unwrap()).unwrap();
    let program = program_regex
        .captures(&lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split(',')
        .map(|d| d.parse().unwrap())
        .collect();
    (program, register_a, register_b, register_c)
}

fn check_execute(
    program: &[u64],
    instructions: &[Instruction],
    mut register_a: u64,
    mut register_b: u64,
    mut register_c: u64,
) -> bool {
    let mut outputs = Vec::new();
    let mut pointer = 0;
    while pointer + 1 < program.len() {
        let instruction = &instructions[pointer];
        let operand = program[pointer + 1];
        let combo = match operand {
            0..=3 => operand,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            _ => panic!(),
        };
        let mut increment = true;
        match instruction {
            Instruction::Adv => {
                register_a = register_a / (1 << combo);
            }
            Instruction::Bxl => {
                register_b = register_b ^ operand;
            }
            Instruction::Bst => {
                register_b = combo & 0b111;
            }
            Instruction::Jnz => {
                if register_a != 0 {
                    pointer = operand as usize;
                    increment = false;
                }
            }
            Instruction::Bxc => {
                register_b = register_b ^ register_c;
            }
            Instruction::Out => {
                let output = combo & 0b111;
                if output != program[outputs.len()] {
                    return false;
                }
                outputs.push(output);
            }
            Instruction::Bdv => {
                register_b = register_a / (1 << combo);
            }
            Instruction::Cdv => {
                register_c = register_a / (1 << combo);
            }
        }

        if increment {
            pointer += 2;
        }
    }
    program.len() == outputs.len()
}

fn parse_match<'a, T>(regex: &'a Regex, haystack: &'a str) -> Option<T>
where
    T: FromStr,
{
    Some(regex.captures(haystack)?.get(1)?.as_str().parse().ok()?)
}
