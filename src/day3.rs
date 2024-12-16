use crate::{get_text_file, SolutionResult};
use itertools::Itertools;
use regex::Regex;
use std::io::Read;

const INPUT_URL: &str = "https://adventofcode.com/2024/day/3/input";

const MUL: &str = "mul";
const DO: &str = "do";
const DONT: &str = "don't";

#[derive(Debug)]
enum Instruction {
    Mul(i64, i64),
    Do,
    Dont,
}

#[derive(PartialEq, Eq, Debug)]
enum Token {
    Number(i64),
    Identifier(String),
    OpenParen,
    CloseParen,
    Comma,
}

pub fn part_1() -> SolutionResult {
    let mut text = String::new();
    get_text_file(INPUT_URL)?.read_to_string(&mut text)?;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;

    let result = re
        .captures_iter(&text)
        .map(|c| {
            let (_, [lhs, rhs]) = c.extract();
            lhs.parse::<i64>().unwrap() * rhs.parse::<i64>().unwrap()
        })
        .sum();

    Ok(result)
}

pub fn part_2() -> SolutionResult {
    let mut text = String::new();
    get_text_file(INPUT_URL)?.read_to_string(&mut text)?;

    let mut tokens: Vec<Token> = Vec::new();
    let mut char_iter = text.chars().peekable();

    while let Some(char) = char_iter.next() {
        tokens.push(match char {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            ',' => Token::Comma,
            ch if ch.is_digit(10) => {
                let mut num = ch.to_string();
                while let Some(peek) = char_iter.peek() {
                    if peek.is_digit(10) {
                        num.push(char_iter.next().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Number(num.parse().unwrap())
            }
            ch if ch == '\'' || ch.is_ascii_alphabetic() => {
                let mut ident = ch.to_string();
                while let Some(peek) = char_iter.peek() {
                    if peek == &'\'' || peek.is_ascii_alphabetic() {
                        ident.push(char_iter.next().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Identifier(ident)
            }
            _ => continue,
        })
    }

    let mut instructions: Vec<Instruction> = Vec::new();
    let mut tokens_iter = tokens.iter();
    while let Some(token) = tokens_iter.next() {
        if let Token::Identifier(ident) = token {
            instructions.push(if ident.ends_with(MUL) {
                let peeks = tokens_iter.clone().take(5).collect_vec();
                if peeks.len() < 5 {
                    continue;
                }
                if let (
                    Token::OpenParen,
                    Token::Number(lhs),
                    Token::Comma,
                    Token::Number(rhs),
                    Token::CloseParen,
                ) = (peeks[0], peeks[1], peeks[2], peeks[3], peeks[4])
                {
                    Instruction::Mul(*lhs, *rhs)
                } else {
                    continue;
                }
            } else if ident.ends_with(DO) {
                let peeks = tokens_iter.clone().take(2).collect_vec();
                if peeks.len() < 2 {
                    continue;
                }
                if let (Token::OpenParen, Token::CloseParen) = (peeks[0], peeks[1]) {
                    Instruction::Do
                } else {
                    continue;
                }
            } else if ident.ends_with(DONT) {
                let peeks = tokens_iter.clone().take(2).collect_vec();
                if peeks.len() < 2 {
                    continue;
                }
                if let (Token::OpenParen, Token::CloseParen) = (peeks[0], peeks[1]) {
                    Instruction::Dont
                } else {
                    continue;
                }
            } else {
                continue;
            });
        }
    }

    let mut enable_mul = true;
    let result = instructions
        .iter()
        .filter_map(|instruction| {
            match instruction {
                Instruction::Mul(lhs, rhs) => {
                    if enable_mul {
                        return Some(lhs * rhs);
                    }
                }
                Instruction::Do => enable_mul = true,
                Instruction::Dont => enable_mul = false,
            }
            None
        })
        .sum();

    Ok(result)
}
