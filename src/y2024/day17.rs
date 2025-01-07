use crate::{get_text_file, SolutionResult};
use itertools::Itertools;
use num::{Num, NumCast};
use regex::Regex;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    iter,
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

#[derive(Clone, Debug)]
struct BitArray<const N: usize> {
    array: [Option<bool>; N],
}

impl<const N: usize> BitArray<N> {
    fn empty() -> Self {
        Self { array: [None; N] }
    }

    fn from_int<T>(mut value: T) -> Self
    where
        T: Num + NumCast + Clone,
    {
        let mut array = [None; N];
        let two: T = NumCast::from(2).unwrap();
        for bit in &mut array {
            *bit = Some((value.clone() % two.clone()).is_one());
            value = value / two.clone();
        }
        Self { array }
    }

    // self = X ^ other
    fn solve_xor(self, other: Self) -> (Vec<Self>, Vec<Self>) {
        let mut possible_values = vec![(Self::empty(), Self::empty())];
        type BitIterator = Box<dyn Iterator<Item = Option<bool>>>;

        for (idx, (bit, other_bit)) in self.array.into_iter().zip(other.array).enumerate() {
            let length = possible_values.len();

            let (result_iter, other_iter): (BitIterator, BitIterator) =
                if let (Some(a), None) = (bit, other_bit) {
                    possible_values.extend(possible_values.clone());

                    (
                        Box::new(
                            iter::repeat(Some(false))
                                .take(length)
                                .chain(iter::repeat(Some(true)).take(length)),
                        ),
                        Box::new(
                            iter::repeat(Some(a))
                                .take(length)
                                .chain(iter::repeat(Some(!a)).take(length)),
                        ),
                    )
                } else {
                    (
                        Box::new(iter::repeat(bit.zip(other_bit).map(|(a, b)| a ^ b)).take(length)),
                        Box::new(iter::repeat(other_bit).take(length)),
                    )
                };
            for ((result_bit, other_bit), (result, other)) in
                result_iter.zip(other_iter).zip(&mut possible_values)
            {
                result.array[idx] = result_bit;
                other.array[idx] = other_bit;
            }
        }
        possible_values.into_iter().unzip()
    }

    fn mask(self, mask: Self) -> Self {
        let mut new_array = [None; N];
        for ((bit, mask_bit), new_bit) in self.array.into_iter().zip(mask.array).zip(&mut new_array)
        {
            *new_bit = bit.zip(mask_bit).filter(|(a, b)| *a && *b).map(|(a, _)| a);
        }
        Self { array: new_array }
    }
}

type BitArray64 = BitArray<64>;

#[derive(Clone, Debug)]
enum Register {
    A,
    B,
    C,
}

#[derive(Clone, Debug)]
enum Operand {
    Literal(BitArray64),
    Variable(Register),
}

impl Operand {
    fn unwrap_literal(self) -> BitArray64 {
        match self {
            Self::Literal(val_) => val_,
            Self::Variable(_) => panic!("unwrap_literal on Operand::Variable"),
        }
    }

    fn register_a() -> Self {
        Self::Variable(Register::A)
    }

    fn register_b() -> Self {
        Self::Variable(Register::B)
    }

    fn register_c() -> Self {
        Self::Variable(Register::C)
    }
}

#[derive(Clone, Debug)]
enum Operation {
    BXorB(Operand),
    BMaskLast3(Operand),
    RightShiftA { assign: Register, operand: Operand },
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
        let literal = Operand::Literal(BitArray64::from_int(operand));
        let combo = match operand {
            0..=3 => literal.clone(),
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
                operand: combo,
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
                operand: combo,
            },
            // Instruction::Cdv => register_c = register_a >> combo,
            Instruction::Cdv => Operation::RightShiftA {
                assign: Register::C,
                operand: combo,
            },
        });
        if increment {
            pointer += 2
        }
    }
    let last_jump = operations
        .iter()
        .position(|op| matches!(op, Operation::AJump))
        .unwrap();
    operations = operations.into_iter().skip(last_jump + 1).collect();
    println!("{:?}", operations);
    let mut possible_registers = vec![(
        BitArray64::from_int(0),
        BitArray64::empty(),
        BitArray64::empty(),
    )];
    for operation in operations {
        let mut new_possible_registers = Vec::new();
        for registers in possible_registers.drain(..) {
            let (mut register_a, mut register_b, mut register_c) = registers;
            let get_operand_value = |operand: Operand| match operand {
                Operand::Literal(bit_array) => bit_array,
                Operand::Variable(Register::A) => register_a.clone(),
                Operand::Variable(Register::B) => register_b.clone(),
                Operand::Variable(Register::C) => register_c.clone(),
            };

            new_possible_registers.extend(match operation.clone() {
                // B = B ^ operand
                Operation::BXorB(operand) => {
                    let value = get_operand_value(operand.clone());
                    let (new_register_b, possible_values) = register_b.solve_xor(value);
                    let mut new_register_a = vec![register_a];
                    let mut new_register_c = vec![register_c];
                    match operand {
                        Operand::Variable(Register::A) => new_register_a = possible_values,
                        Operand::Variable(Register::C) => new_register_c = possible_values,
                        _ => {}
                    }
                    new_register_b
                        .into_iter()
                        .zip(new_register_a.into_iter().cycle())
                        .zip(new_register_c.into_iter().cycle())
                        .map(|((b, a), c)| (a, b, c))
                        .collect()
                }
                // B = operand & 0b111
                Operation::BMaskLast3(operand) => {
                    let value = get_operand_value(operand);
                    vec![(
                        register_a,
                        value.mask(BitArray64::from_int(0b111)),
                        register_c,
                    )]
                }
                // X = A >> operand,
                Operation::RightShiftA { assign, operand } => {
                    let value = get_operand_value(operand);
                    todo!()
                }
                // output: operand & 0b111,
                Operation::OutputMaskLast3(operand) => {
                    todo!()
                }
                // if A != 0: jump,
                Operation::AJump => {
                    todo!()
                }
            });
        }
        possible_registers = new_possible_registers;
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

fn parse_match<'a, T>(regex: &'a Regex, haystack: &'a str) -> Option<T>
where
    T: FromStr,
{
    Some(regex.captures(haystack)?.get(1)?.as_str().parse().ok()?)
}
