use crate::{get_text_file, SolutionResult};
use itertools::{repeat_n, Itertools};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/7/input";

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;

    let result: i64 = iter_input(file)
        .filter_map(|(value, operands)| {
            let operators = [|lhs, rhs| lhs + rhs, |lhs, rhs| lhs * rhs];
            if check_operators(value, &operands, &operators) {
                Some(value)
            } else {
                None
            }
        })
        .sum();

    Ok(result.to_string())
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;

    let result: i64 = iter_input(file)
        .filter_map(|(value, operands)| {
            let operators = [
                |lhs, rhs| lhs + rhs,
                |lhs, rhs| lhs * rhs,
                |lhs, rhs| lhs * 10_i64.pow(num_digits(rhs)) + rhs,
            ];
            if check_operators(value, &operands, &operators) {
                Some(value)
            } else {
                None
            }
        })
        .sum();

    Ok(result.to_string())
}

pub fn iter_input(file: File) -> impl Iterator<Item = (i64, Vec<i64>)> {
    BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        let (value, operands) = line.split(':').collect_tuple().unwrap();
        (
            value.parse().unwrap(),
            operands
                .split_ascii_whitespace()
                .map(|op| op.parse().unwrap())
                .collect(),
        )
    })
}

fn check_operators(value: i64, operands: &[i64], operators: &[fn(i64, i64) -> i64]) -> bool {
    let mut operands_iter = operands.iter();
    if let Some(init_operand) = operands_iter.next() {
        repeat_n(operators.iter(), operands.len() - 1)
            .multi_cartesian_product()
            .any(|operators| {
                value
                    == operands_iter
                        .clone()
                        .zip(operators.iter())
                        .fold(*init_operand, |lhs, (rhs, op)| op(lhs, *rhs))
            })
    } else {
        value == 0
    }
}

fn num_digits(n: i64) -> u32 {
    let mut count = 0;
    let mut num = n.abs();
    if num == 0 {
        return 1;
    }
    while num > 0 {
        count += 1;
        num /= 10;
    }
    count
}
